use {
    crate::{routes::Result, App},
    actix_web::{web, HttpRequest, HttpResponse},
    serde::{Deserialize, Serialize},
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("auth").route("/register", web::post().to(register)));
}

#[derive(Serialize, Deserialize)]
struct RegisterPayload {
    pub first_name: String,
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
    Ok(HttpResponse::Ok().json("{}"))
}
