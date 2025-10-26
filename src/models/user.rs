use {
    crate::{
        bitflags_convector,
        routes::{HttpError, Result as HttpResult},
        utils::snowflake::Snowflake,
    },
    bitflags::bitflags,
    serde::{Deserialize, Serialize},
    sha256::digest,
    sqlx::{sqlite::SqliteValueRef, Decode, Sqlite, SqliteExecutor},
};

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct Permissions: i64 {
        /// Allows requesting books from other users
        const REQUEST_BOOKS = 1 << 0;
        /// Allows creatin of new books
        const CREATE_BOOKS = 1 << 1;
        /// Allows managment of books
        const MANAGE_BOOKS = 1 << 2;
        /// Allows for the addition of reviews for books, book holders
        const ADD_REVIEWS = 1 << 3;
        /// Allows for deletion reviews
        const DELETE_REVIEWS = 1 << 4;
        /// Allows for editing (reseting passwords, changing permissions), deleting, viewing all users on platform
        const MODERATE_USERS = 1 << 6;
        /// Allows all permissions and grants access to all endpoints (This is dangerous permission to grant)
        const ADMINISTRATOR = i64::MAX;
    }
}

bitflags_convector!(Permissions, i64);

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Snowflake,
    pub username: String,
    pub email: String,
    pub first_name: String,
    pub second_name: String,
    #[serde(default, skip)]
    pub password_hash: String,
    pub school_name: Option<String>,
    pub permissions: Permissions,
}

impl User {
    pub fn new(
        id: Snowflake, username: &str, email: &str, first_name: &str, second_name: &str,
        password: &str, school: Option<String>,
    ) -> Self {
        Self {
            id,
            username: username.to_string(),
            email: email.to_string(),
            first_name: first_name.to_string(),
            second_name: second_name.to_string(),
            password_hash: digest(password),
            school_name: school,
            permissions: Permissions::ADD_REVIEWS
                | Permissions::REQUEST_BOOKS
                | Permissions::CREATE_BOOKS,
        }
    }

    pub async fn save<'a, E: SqliteExecutor<'a>>(self, executor: E) -> HttpResult<Self> {
        sqlx::query!(r#"INSERT INTO users(id, username, email, first_name, second_name, password_hash, school_name) VALUES ($1, $2, $3, $4, $5, $6, $7)"#,
            self.id.0, self.username, self.email, self.first_name, self.second_name, self.password_hash, self.school_name
        )
            .execute(executor).await
            .map(|_| self)
            .map_err(HttpError::Database)
    }

    /// Checks whether user has required [`Permissions`]
    pub fn has_permission(&self, permission: Permissions) -> bool {
        self.permissions.contains(permission)
    }
}
