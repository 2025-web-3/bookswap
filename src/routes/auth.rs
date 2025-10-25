use {
    crate::{
        models::{
            requests::{LoginPayload, RegisterPayload},
            session::Session,
            user::User,
        },
        routes::{HttpError, Result},
        utils::authorization::{extract_ip_from_request, is_valid_password, new_token},
        App,
    },
    actix_web::{web, HttpRequest, HttpResponse},
    serde::Serialize,
    sha256::digest,
    validator::Validate,
};

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login)),
    );
}

#[derive(Serialize)]
struct AuthResponse {
    pub token: String,
    pub user: User,
}

async fn register(
    request: HttpRequest, payload: web::Json<RegisterPayload>, app: web::Data<App>,
) -> Result<HttpResponse> {
    payload.validate().map_err(|err| HttpError::Validation(err))?;

    if app.database.fetch_user_by_username(&payload.username).await.is_some() {
        return Err(HttpError::TakenUsername);
    }

    if app.database.fetch_user_by_email(&payload.email).await.is_some() {
        return Err(HttpError::TakenEmail);
    }

    if !is_valid_password(&payload.password) {
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

    // NOTE: Token length must be 128, 2 was set for debugging purposes.
    let token = new_token(2);
    Session::new(id, token.clone(), extract_ip_from_request(&request)?).save(&app.pool).await?;

    Ok(HttpResponse::Ok().json(AuthResponse { user: user, token: token }))
}

async fn login(
    request: HttpRequest, payload: web::Json<LoginPayload>, app: web::Data<App>,
) -> Result<HttpResponse> {
    payload.validate().map_err(|err| HttpError::Validation(err))?;

    if (payload.username.is_some() && payload.email.is_some())
        || (payload.username.is_none() && payload.email.is_none())
    {
        return Err(HttpError::InvalidCredentials(
            "Only username or email must be specified".to_string(),
        ));
    }

    let user = if payload.email.is_some() {
        app.database
            .fetch_user_by_email(&payload.email.clone().unwrap())
            .await
            .ok_or(HttpError::InvalidCredentials("Login or password is invalid".to_string()))?
    } else if payload.username.is_some() {
        app.database
            .fetch_user_by_username(&payload.username.clone().unwrap())
            .await
            .ok_or(HttpError::InvalidCredentials("Login or password is invalid".to_string()))?
    } else {
        return Err(HttpError::InvalidCredentials("Login or password is invalid".to_string()));
    };

    if user.password_hash != digest(&payload.password) {
        return Err(HttpError::InvalidCredentials("Login or password is invalid".to_string()));
    }

    let token = new_token(128);
    Session::new(user.id, token.clone(), extract_ip_from_request(&request)?)
        .save(&app.pool)
        .await?;

    Ok(HttpResponse::Ok().json(AuthResponse { user: user, token: token }))
}
