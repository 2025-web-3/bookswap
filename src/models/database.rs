use {
    crate::{
        models::{session::Session, user::User},
        utils::snowflake::Snowflake,
    },
    sha256::digest,
    sqlx::SqlitePool,
};

pub struct Database {
    pool: SqlitePool,
}

/// Application Database Manager
impl Database {
    /// Create a new application database manager
    pub const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn fetch_user_by_id(&self, user_id: Snowflake) -> Option<User> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", user_id.0)
            .fetch_optional(&self.pool)
            .await
            .ok()?
    }

    pub async fn fetch_user_by_username(&self, username: &str) -> Option<User> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE username = $1", username)
            .fetch_optional(&self.pool)
            .await
            .ok()?
    }

    pub async fn fetch_user_by_email(&self, email: &str) -> Option<User> {
        sqlx::query_as!(User, "SELECT * FROM users WHERE email = $1", email)
            .fetch_optional(&self.pool)
            .await
            .ok()?
    }

    pub async fn check_session_by_user_id_and_ip_exists(
        &self, user_id: Snowflake, ip: &str,
    ) -> bool {
        sqlx::query_scalar!(
            "SELECT COUNT(*) FROM sessions WHERE user_id = $1 AND ip_address = $2",
            user_id.0,
            ip
        )
        .fetch_one(&self.pool)
        .await
        .unwrap_or(0)
            > 0
    }

    pub async fn fetch_session_by_token_and_ip(&self, token: &str, ip: &str) -> Option<Session> {
        let token = digest(token);

        sqlx::query_as!(
            Session,
            "SELECT * FROM sessions WHERE token_hash = $1 AND ip_address = $2",
            token,
            ip
        )
        .fetch_optional(&self.pool)
        .await
        .ok()?
    }

    pub async fn fetch_user_by_token_and_ip(&self, token: &str, ip: &str) -> Option<User> {
        let user_id =
            self.fetch_session_by_token_and_ip(token, ip).await.map(|user| user.user_id)?;

        self.fetch_user_by_id(user_id).await
    }
}
