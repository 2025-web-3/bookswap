use {
    crate::{
        models::{session::Session, user::User},
        routes::{HttpError, Result},
        utils::authorization::extract_ip_from_request,
        App,
    },
    actix_web::{web, HttpRequest, HttpResponse},
    nanoid::nanoid,
    regex::Regex,
    serde::{Deserialize, Serialize},
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

const _TOKEN_CHARS: [char; 33] = [
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'a', 'b', 'c', 'd', 'e', 'f', 'A', 'B', 'C',
    'D', 'E', 'F', 'k', 'n', 'm', 'K', 'N', 'M', 'h', 'H', 'l', 'L', '.',
];

fn new_token(length: usize) -> String {
    nanoid!(length, &_TOKEN_CHARS)
}

#[derive(Serialize, Deserialize, Validate)]
struct RegisterPayload {
    #[validate(length(
        min = 2,
        max = 32,
        message = "Username length must be between 2 and 32 characters"
    ))]
    pub username: String,
    #[validate(email(message = "Incorrect email address"))]
    pub email: String,
    #[validate(length(
        min = 1,
        max = 24,
        message = "First name length must be between 1 and 24 characters"
    ))]
    pub first_name: String,
    #[validate(length(
        min = 1,
        max = 24,
        message = "Second name length must be between 1 and 24 characters"
    ))]
    pub second_name: String,
    pub school_name: Option<String>,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
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

    let strong_password = Regex::new(r"^(?=.*[a-z])(?=.*[A-Z])(?=.*\d)[A-Za-z\d]{8,}$").unwrap();
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

    let token = new_token(128);
    Session::new(id, token.clone(), extract_ip_from_request(&request)?).save(&app.pool).await?;

    Ok(HttpResponse::Ok().json(AuthResponse { user: user, token: token }))
}

#[derive(Serialize, Deserialize, Validate)]
struct LoginPayload {
    #[validate(length(
        min = 2,
        max = 32,
        message = "Username length must be between 2 and 32 characters"
    ))]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
    pub password: String,
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
