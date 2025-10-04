use {
    crate::{models::database::Database, utils::snowflake::SnowflakeBuilder},
    sqlx::SqlitePool,
    std::sync::Mutex,
};

pub mod models;
pub mod routes;
pub mod utils;

pub struct App {
    pub snowflake: Mutex<SnowflakeBuilder>,
    pub database: Database,
    pub pool: SqlitePool,
}
