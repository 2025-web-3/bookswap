use {
    crate::{
        models::book::Book,
        routes::{HttpError, Result},
        App,
    },
    actix_web::{web, HttpResponse},
    serde::Deserialize,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("books")
            .route("search", web::get().to(book_search))
            .route("{book_id}", web::get().to(get_book_by_id))
            .route("{book_id}/holders", web::get().to(get_holders_by_book_id))
            .route("requests/{request_id}", web::patch().to(update_requests_acceptance)),
    );
}

async fn get_book_by_id(book_id: web::Path<i64>, app: web::Data<App>) -> Result<HttpResponse> {
    let book =
        app.database.fetch_book(book_id.to_owned().into()).await.ok_or(HttpError::UnknownBook)?;

    Ok(HttpResponse::Ok().json(book))
}

async fn get_holders_by_book_id(
    book_id: web::Path<i64>, app: web::Data<App>,
) -> Result<HttpResponse> {
    let book = app
        .database
        .fetch_holders_by_book_id(book_id.to_owned().into())
        .await
        .ok_or(HttpError::UnknownBook)?;

    Ok(HttpResponse::Ok().json(book))
}

#[derive(Deserialize)]
struct BookSearchQuery {
    pub query: String,
}

async fn book_search(
    app: web::Data<App>, query: web::Query<BookSearchQuery>,
) -> Result<HttpResponse> {
    let search = format!("%{}%", query.query);

    let books =
        sqlx::query_as!(Book, "SELECT * FROM books WHERE title LIKE $1 ORDER BY id DESC", search)
            .fetch_all(&app.pool)
            .await
            .ok()
            .ok_or(HttpError::UnknownBook)?;

    Ok(HttpResponse::Ok().json(books))
}

#[derive(Deserialize)]
struct UpdateAcceptancePayload {
    pub is_accepted: bool,
}

async fn update_requests_acceptance(
    app: web::Data<App>, path: web::Path<i64>, payload: web::Json<UpdateAcceptancePayload>,
) -> Result<HttpResponse> {
    let request_id = path.to_owned();

    sqlx::query!(
        "UPDATE books_requests SET is_accepted = $1 AND accepted_at = CURRENT_TIMESTAMP WHERE id = $2",
        payload.is_accepted, request_id
    )
    .execute(&app.pool)
    .await
    .map_err(|err| HttpError::Database(err))?;

    Ok(HttpResponse::NoContent().finish())
}
