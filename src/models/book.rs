use {
    crate::{
        routes::{HttpError, Result as HttpResult},
        utils::snowflake::Snowflake,
    },
    chrono::NaiveDateTime,
    serde::{Deserialize, Serialize},
    sqlx::SqliteExecutor,
};

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub id: Snowflake,
    pub title: String,
    pub description: String,
    pub author: String,
    pub subjects: Option<String>,
    pub pages: Option<i64>,
    pub cover_url: Option<String>,
    pub publish_date: NaiveDateTime,
}

impl Book {
    pub fn new(
        id: Snowflake, title: &str, description: &str, author: &str, subjects: Option<String>,
        pages: Option<i64>, cover_url: Option<String>, publish_date: NaiveDateTime,
    ) -> Self {
        Self {
            id,
            title: title.to_string(),
            description: description.to_string(),
            author: author.to_string(),
            subjects,
            pages,
            cover_url,
            publish_date,
        }
    }

    pub async fn save<'a, E: SqliteExecutor<'a>>(self, executor: E) -> HttpResult<Self> {
        sqlx::query!(r#"INSERT INTO books(id, title, description, author, subjects, pages, cover_url, publish_date) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"#,
            self.id.0, self.title, self.description, self.author, self.subjects, self.pages, self.cover_url, self.publish_date
        )
            .execute(executor).await
            .map(|_| self)
            .map_err(HttpError::Database)
    }
}
