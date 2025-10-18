use {
    crate::{
        routes::{HttpError, Result as HttpResult},
        utils::snowflake::Snowflake,
    },
    chrono::{NaiveDate, NaiveDateTime, NaiveTime},
    serde::{Deserialize, Serialize},
    sha256::digest,
    sqlx::SqliteExecutor,
};

#[derive(Serialize, Deserialize)]
pub struct Session {
    pub user_id: Snowflake,
    pub token_hash: String,
    pub ip_address: String,
    pub created_at: NaiveDateTime,
}

impl Session {
    pub fn new(user_id: Snowflake, token: String, ip: String) -> Self {
        Self {
            user_id,
            token_hash: digest(token),
            ip_address: ip,
            created_at: NaiveDateTime::new(
                NaiveDate::from_ymd_opt(1, 1, 1).unwrap(),
                NaiveTime::from_hms_opt(0, 0, 0).unwrap(),
            ),
        }
    }

    pub async fn save<'a, E: SqliteExecutor<'a>>(self, executor: E) -> HttpResult<Self> {
        sqlx::query!(
            r#"INSERT INTO sessions(token_hash, user_id, ip_address) VALUES ($1, $2, $3)"#,
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
