use {
    crate::{
        routes::{HttpError, Result},
        App,
    },
    actix_web::{web, HttpRequest, HttpResponse},
    serde::{Deserialize, Serialize},
    validator::Validate,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("auth").route("/register", web::post().to(register)));
}

#[derive(Serialize, Deserialize, Validate)]
struct RegisterPayload {
    pub username: String,
    pub first_name: String,
    pub second_name: String,
    pub school_name: Option<String>,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct RegisterResponse {
    pub token: String,
    pub user: String,
}

async fn register(
    request: HttpRequest,
    payload: web::Json<RegisterPayload>,
    app: web::Data<App>,
) -> Result<HttpResponse> {
    payload
        .validate()
        .map_err(|err| HttpError::Validation(err))?;

    if app
        .database
        .fetch_user_by_username(&payload.username)
        .await
        .is_some()
    {
        return Err(HttpError::TakenUsername);
    }

    if payload.password.is_empty() {
        return Err(HttpError::WeekPassword);
    }

    Ok(HttpResponse::Ok().json("{}"))
}
