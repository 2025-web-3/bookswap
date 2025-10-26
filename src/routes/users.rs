use {
    crate::{
        models::{
            book::{Book, BookSharing},
            requests::NewBook,
            user::{Permissions, User},
        },
        routes::{HttpError, Result},
        App,
    },
    actix_web::{web, HttpResponse},
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("users")
            .service(
                web::scope("@me")
                    .route("", web::get().to(get_current_user))
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
    credentials: Option<web::ReqData<User>>,
) -> Result<HttpResponse> {
    let _me = credentials.ok_or(HttpError::Unauthorized)?;

    return Err(HttpError::MissingAccess);
}

async fn get_sharing_by_user_and_book(
    path: web::Path<(i64, i64)>, app: web::Data<App>,
) -> Result<HttpResponse> {
    let sharings = app
        .database
        .fetch_holding_by_book_and_holder_id(path.to_owned().1.into(), path.to_owned().0.into())
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
        payload.condition.clone(),
    )
    .save(&app.pool)
    .await?;

    Ok(HttpResponse::Ok().json(sharing))
}

async fn request_book(
    path: web::Path<(i64, i64)>, app: web::Data<App>, credentials: Option<web::ReqData<User>>,
) -> Result<HttpResponse> {
    let me = credentials.ok_or(HttpError::Unauthorized)?;

    let sharings = app
        .database
        .fetch_holding_by_book_and_holder_id(path.to_owned().1.into(), path.to_owned().0.into())
        .await
        .ok_or(HttpError::UnknownBook)?;

    Ok(HttpResponse::Ok().json(sharings))
}
