use {crate::models::book::BookCondition, serde::Deserialize, validator::Validate};

#[derive(Deserialize, Validate)]
pub struct RegisterPayload {
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
    #[validate(length(
        max = 64,
        message = "School name length must be between 1 and 64 characters"
    ))]
    pub school_name: Option<String>,
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct LoginPayload {
    #[validate(length(
        min = 2,
        max = 32,
        message = "Username length must be between 2 and 32 characters"
    ))]
    pub username: Option<String>,
    #[validate(email(message = "Incorrect email address"))]
    pub email: Option<String>,
    pub password: String,
}

#[derive(Deserialize, Validate)]
pub struct NewBook {
    #[validate(length(
        min = 10,
        max = 13,
        message = "ISBN length must be between 10 and 13 characters"
    ))]
    pub isbn: String,
    #[validate(length(
        min = 2,
        max = 1024,
        message = "Comment length must be between 2 and 1024 characters"
    ))]
    pub comment: Option<String>,
    pub condition: BookCondition,
}
