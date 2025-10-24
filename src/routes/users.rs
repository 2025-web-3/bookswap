use {
    crate::{
        models::user::User,
        routes::{HttpError, Result},
    },
    actix_web::{web, HttpResponse},
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("users").route("@me", web::get().to(get_current_user)));
}

async fn get_current_user(credentials: Option<web::ReqData<User>>) -> Result<HttpResponse> {
    credentials.ok_or(HttpError::Unauthorized).map(|row| HttpResponse::Ok().json(row.into_inner()))
}
