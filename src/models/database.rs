use {crate::models::user::User, sqlx::SqlitePool};

pub struct Database {
    pool: SqlitePool,
}

/// Application Database Manager
impl Database {
    /// Create a new application database manager
    pub const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn fetch_user_by_username(&self, username: &str) -> Option<User> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(&self.pool)
            .await
            .ok()?
    }
}
