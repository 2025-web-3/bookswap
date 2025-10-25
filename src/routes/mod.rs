use {
    crate::{models::error, utils::middleware::authorization_middleware},
    actix_web::{
        http::StatusCode,
        middleware::from_fn,
        {web, HttpResponse},
    },
};

mod auth;
mod books;
mod users;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("api/v1")
            .configure(auth::config)
            .configure(users::config)
            .configure(books::config)
            .wrap(from_fn(authorization_middleware)),
    );
}

pub type Result<T> = core::result::Result<T, HttpError>;

#[derive(thiserror::Error, Debug)]
pub enum HttpError {
    #[error("Unknown Book")]
    UnknownBook,
    #[error("{0}")]
    Payload(#[from] actix_web::error::JsonPayloadError),
    #[error("Validation error: {0}")]
    Validation(#[from] validator::ValidationErrors),
    #[error("{0}")]
    Path(#[from] actix_web::error::PathError),
    #[error("{0}")]
    Query(#[from] actix_web::error::QueryPayloadError),
    #[error("{0}")]
    Header(String),
    #[error("Error while interacting with the database: {0}")]
    Database(#[from] sqlx::Error),
    #[error("Missing access")]
    MissingAccess,
    #[error("The username is already taken")]
    TakenUsername,
    #[error("The email is already taken")]
    TakenEmail,
    #[error("Too week password")]
    WeekPassword,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("{0}")]
    InvalidCredentials(String),
}

impl actix_web::ResponseError for HttpError {
    fn status_code(&self) -> StatusCode {
        match self {
            HttpError::UnknownBook => StatusCode::NOT_FOUND,

            HttpError::Payload(..)
            | HttpError::Validation(..)
            | HttpError::Query(..)
            | HttpError::Path(..)
            | HttpError::Header(..)
            | HttpError::TakenUsername
            | HttpError::TakenEmail
            | HttpError::WeekPassword
            | HttpError::InvalidCredentials(..) => StatusCode::BAD_REQUEST,

            HttpError::Unauthorized => StatusCode::UNAUTHORIZED,

            HttpError::MissingAccess => StatusCode::FORBIDDEN,

            HttpError::Database(..) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code()).json(error::Error {
            code: match self {
                // The 1xxxx class of error code indicates that some data wasn't found
                HttpError::UnknownBook => 10000,

                // The 2xxxx class of error code indicates that data was malformed or invalid
                HttpError::Payload(..) => 20000,
                HttpError::Path(..) => 20001,
                HttpError::Query(..) => 20002,
                HttpError::Header(..) => 20003,
                HttpError::Validation(..) => 20004,
                HttpError::InvalidCredentials(..) => 20005,
                HttpError::Database(..) => 20007,
                HttpError::TakenUsername => 20010,
                HttpError::TakenEmail => 20011,

                // The 3xxxx class of error code indicates that authorization process failed
                HttpError::Unauthorized => 30000,
                HttpError::WeekPassword => 30001,

                // The 4xxxx class of error code indicates that recourse requires special permission
                HttpError::MissingAccess => 40000,
            },
            description: self.to_string(),
        })
    }
}
