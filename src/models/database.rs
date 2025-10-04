use sqlx::SqlitePool;

pub struct Database {
    pool: SqlitePool,
}

/// Application Database Manager
impl Database {
    /// Create a new application database manager
    pub const fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}
