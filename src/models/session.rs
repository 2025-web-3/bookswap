use {
    crate::{
        routes::{HttpError, Result as HttpResult},
        utils::snowflake::Snowflake,
    },
    chrono::{DateTime, TimeZone, Utc},
    serde::{Deserialize, Serialize},
    sha256::digest,
    sqlx::SqliteExecutor,
};

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub user_id: Snowflake,
    pub token_hash: String,
    pub ip_address: String,
    pub created_at: DateTime<Utc>,
}

impl Session {
    pub fn new(user_id: Snowflake, token: String, ip: String) -> Self {
        Self {
            user_id,
            token_hash: digest(token),
            ip_address: ip,
            created_at: Utc.with_ymd_and_hms(0, 1, 1, 0, 0, 0).unwrap(),
        }
    }

    pub async fn save<'a, E: SqliteExecutor<'a>>(self, executor: E) -> HttpResult<Self> {
        sqlx::query!(
            r#"INSERT INTO sessions(token, user_id, ip_address) VALUES ($1, $2, $3)"#,
            self.token_hash,
            self.user_id.0,
            self.ip_address
        )
        .execute(executor)
        .await
        .map(|_| self)
        .map_err(HttpError::Database)
    }
}
