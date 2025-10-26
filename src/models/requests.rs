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
        min = 16,
        max = 2048,
        message = "Sharing length must be between 16 and 2048 characters"
    ))]
    pub isbn: String,
    pub comment: Option<String>,
    pub condition: BookCondition,
}
