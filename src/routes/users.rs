use {
    crate::{
        models::{book::Book, user::User},
        routes::{HttpError, Result},
        utils::openlibrary::get_open_library_books,
        App,
    },
    actix_web::{web, HttpRequest, HttpResponse},
    serde::Deserialize,
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
            .route("{user_id}/books/{book_id}", web::get().to(get_sharing_by_user_and_book)),
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
    let message = app
        .database
        .fetch_holding_by_book_and_holder_id(path.to_owned().0.into(), path.to_owned().1.into())
        .await
        .ok_or(HttpError::UnknownBook)?;

    Ok(HttpResponse::Ok().json(message))
}

#[derive(Deserialize)]
struct NewBook {
    pub isbn: String,
}

async fn add_new_book(
    _request: HttpRequest, payload: web::Json<NewBook>, app: web::Data<App>,
    credentials: Option<web::ReqData<User>>,
) -> Result<HttpResponse> {
    let _me = credentials.ok_or(HttpError::Unauthorized)?;

    let book = if let Some(book) = app.database.fetch_book_by_isbn(&payload.isbn).await {
        book
    } else {
        let id = app.snowflake.lock().unwrap().build();

        let isbn_key = format!("ISBN:{}", &payload.isbn);
        let bibkeys = vec![isbn_key.clone()];

        let mut book_map =
            get_open_library_books(bibkeys).await.map_err(|_| HttpError::UnknownBook)?;
        let book_data = book_map.remove(&isbn_key).ok_or_else(|| HttpError::UnknownBook)?;
        let title_opt = book_data.title.as_deref().unwrap_or("No Title");

        let description_ref = book_data
            .excerpts
            .as_ref()
            .and_then(|e| e.first())
            .map(|e| e.text.as_str())
            .unwrap_or("No Description");

        let author_name = book_data
            .authors
            .as_ref()
            .and_then(|a| a.first())
            .map(|a| a.name.as_str())
            .unwrap_or("Unknown Author");

        let subject_name = book_data.subjects.and_then(|s| s.first().map(|s| s.name.clone()));
        let cover_url = book_data.cover.and_then(|cover| cover.medium);

        Book::new(
            id,
            Some(payload.isbn.clone()),
            title_opt,
            description_ref,
            author_name,
            subject_name,
            book_data.number_of_pages,
            cover_url,
            None,
        )
    };

    Ok(HttpResponse::Ok().json(book))
}
