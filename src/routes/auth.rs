use {
    crate::{
        models::{new_hex_id, session::Session, user::User},
        routes::{HttpError, Result},
        utils::authorization::extract_ip_from_request,
        App,
    },
    actix_web::{web, HttpRequest, HttpResponse},
    regex::Regex,
    serde::{Deserialize, Serialize},
    validator::Validate,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("auth").route("/register", web::post().to(register)));
}

#[derive(Serialize, Deserialize, Validate)]
struct RegisterPayload {
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub second_name: String,
    pub school_name: Option<String>,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct RegisterResponse {
    pub token: String,
    pub user: User,
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

    let strong_password = Regex::new(r"^[a-zA-Z0-9!@#$&()\\-`.+,/]*${12,}").unwrap();
    if !strong_password.is_match(&payload.password) {
        return Err(HttpError::WeekPassword);
    }

    let id = app.snowflake.lock().unwrap().build();

    let user = User::new(
        id,
        &payload.username,
        &payload.email,
        &payload.first_name,
        &payload.second_name,
        &payload.password,
        payload.school_name.clone(),
    )
    .save(&app.pool)
    .await?;

    let token = new_hex_id(32);
    Session::new(id, token.clone(), extract_ip_from_request(&request)?)
        .save(&app.pool)
        .await?;

    Ok(HttpResponse::Ok().json(RegisterResponse {
        user: user,
        token: token,
    }))
}
