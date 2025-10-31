use {
    crate::{
        models::{
            book::{Book, BookRequest, BookSharing},
            requests::NewBook,
            user::{Permissions, User},
        },
        routes::{HttpError, Result},
        App,
    },
    actix_web::{web, HttpResponse},
    serde::Deserialize,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("users")
            .service(
                web::scope("@me")
                    .route("", web::get().to(get_current_user))
                    .route("requests", web::get().to(get_current_user_requested_books))
                    .route("books/requests", web::get().to(get_current_user_book_requests))
                    .route("books", web::post().to(add_new_book)),
            )
            .service(
                web::scope("{user_id}")
                    .route("books/{book_id}", web::get().to(get_sharing_by_user_and_book))
                    .route("books/{book_id}/request", web::post().to(request_book))
                    .route("books", web::get().to(get_all_avalible_books_from_user)),
            ),
    );
}

async fn get_current_user(credentials: Option<web::ReqData<User>>) -> Result<HttpResponse> {
    credentials.ok_or(HttpError::Unauthorized).map(|row| HttpResponse::Ok().json(row.into_inner()))
}

async fn get_current_user_book_requests(
    app: web::Data<App>, credentials: Option<web::ReqData<User>>,
) -> Result<HttpResponse> {
    let me = credentials.ok_or(HttpError::Unauthorized)?;

    let res = app.database.fetch_user_requested_books(me.id).await.ok_or(HttpError::UnknownBook)?;

    Ok(HttpResponse::Ok().json(res))
}

async fn get_current_user_requested_books(
    app: web::Data<App>, credentials: Option<web::ReqData<User>>,
) -> Result<HttpResponse> {
    let me = credentials.ok_or(HttpError::Unauthorized)?;

    let reqs =
        app.database.fetch_user_requested_books(me.id).await.ok_or(HttpError::UnknownBook)?;

    Ok(HttpResponse::Ok().json(reqs))
}

async fn get_sharing_by_user_and_book(
    path: web::Path<(i64, i64)>, app: web::Data<App>,
) -> Result<HttpResponse> {
    let sharings = app
        .database
        .fetch_sharings_by_book_and_holder_id(path.1.into(), path.0.into())
        .await
        .ok_or(HttpError::UnknownBook)?;

    Ok(HttpResponse::Ok().json(sharings))
}

async fn get_all_avalible_books_from_user(
    path: web::Path<i64>, app: web::Data<App>,
) -> Result<HttpResponse> {
    let sharings = app
        .database
        .fetch_all_unique_shared_books(path.to_owned().into())
        .await
        .ok_or(HttpError::UnknownBook)?;

    Ok(HttpResponse::Ok().json(sharings))
}

async fn add_new_book(
    payload: web::Json<NewBook>, app: web::Data<App>, credentials: Option<web::ReqData<User>>,
) -> Result<HttpResponse> {
    let me = credentials.ok_or(HttpError::Unauthorized)?;

    if !me.has_permission(Permissions::CREATE_BOOKS) {
        return Err(HttpError::MissingAccess);
    }

    let book = if let Some(book) = app.database.fetch_book_by_isbn(&payload.isbn).await {
        book
    } else {
        let id = app.snowflake.lock().unwrap().build();
        Book::from_isbn(id, &payload.isbn).await?.save(&app.pool).await?
    };

    let sharing_id = app.snowflake.lock().unwrap().build();

    let sharing = BookSharing::new(
        sharing_id,
        book,
        payload.comment.clone(),
        me.id,
        payload.condition.clone().into(),
    )
    .save(&app.pool)
    .await?;

    Ok(HttpResponse::Ok().json(sharing))
}

#[derive(Deserialize)]
struct BookSharingIdQuery {
    pub sharing_id: Option<i64>,
}

/// TODO: Add new errors for specific situations
async fn request_book(
    path: web::Path<(i64, i64)>, query: web::Query<BookSharingIdQuery>, app: web::Data<App>,
    credentials: Option<web::ReqData<User>>,
) -> Result<HttpResponse> {
    let me = credentials.ok_or(HttpError::Unauthorized)?;

    if !me.has_permission(Permissions::REQUEST_BOOKS) {
        return Err(HttpError::MissingAccess);
    }

    let sharings = app
        .database
        .fetch_sharings_by_book_and_holder_id(path.0.into(), path.1.into())
        .await
        .ok_or(HttpError::UnknownSharing)?;

    if sharings.len() > 1 && query.sharing_id.is_none() {
        return Err(HttpError::MissingAccess);
    }

    let reqs = app.database.fetch_user_requested_books(me.id).await;

    if reqs.is_some() {
        if reqs.unwrap().iter().filter(|x| x.book_id == path.1.into()).count() > 0 {
            return Err(HttpError::MissingAccess);
        }
    }

    let request_id = app.snowflake.lock().unwrap().build();

    let req = if query.sharing_id.is_none() {
        let f = sharings.first().unwrap();
        BookRequest::new(request_id, f.id, f.book.id, me.id).save(&app.pool).await?
    } else {
        let sharing = app
            .database
            .fetch_sharing(query.sharing_id.unwrap().into())
            .await
            .ok_or(HttpError::UnknownSharing)?;
        BookRequest::new(request_id, sharing.id, sharing.book.id, me.id)
    };

    Ok(HttpResponse::Ok().json(req))
}
