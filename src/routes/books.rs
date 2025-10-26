use {
    crate::{
        routes::{HttpError, Result},
        App,
    },
    actix_web::{web, HttpResponse},
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("books")
            .route("{book_id}", web::get().to(get_book_by_id))
            .route("{book_id}/holders", web::get().to(get_holders_by_book_id)),
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
