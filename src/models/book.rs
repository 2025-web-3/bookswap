use {
    crate::{
        routes::{HttpError, Result as HttpResult},
        utils::snowflake::Snowflake,
    },
    chrono::NaiveDateTime,
    serde::{Deserialize, Serialize},
    sqlx::{sqlite::SqliteValueRef, Decode, Sqlite, SqliteExecutor},
};

#[derive(Serialize, Deserialize)]
pub struct Book {
    pub id: Snowflake,
    pub isbn: Option<String>,
    pub title: String,
    pub description: String,
    pub author: String,
    pub subjects: Option<String>,
    pub pages: Option<i64>,
    pub cover_url: Option<String>,
    pub publish_date: Option<NaiveDateTime>,
}

impl Book {
    pub fn new(
        id: Snowflake, isbn: Option<String>, title: &str, description: &str, author: &str,
        subjects: Option<String>, pages: Option<i64>, cover_url: Option<String>,
        publish_date: Option<NaiveDateTime>,
    ) -> Self {
        Self {
            id,
            isbn,
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
        sqlx::query!(r#"INSERT INTO books(id, isbn, title, description, author, subjects, pages, cover_url, publish_date) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"#,
            self.id.0, self.isbn, self.title, self.description, self.author, self.subjects, self.pages, self.cover_url, self.publish_date
        )
            .execute(executor).await
            .map(|_| self)
            .map_err(HttpError::Database)
    }
}

#[derive(Serialize)]
pub struct BookSharing {
    pub id: Snowflake,
    pub book: Book,
    pub comment: Option<String>,
    pub holder_id: Snowflake,
}

impl Decode<'_, Sqlite> for Book {
    fn decode(
        value: SqliteValueRef<'_>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let s: sqlx::types::Json<Book> = sqlx::Decode::<'_, Sqlite>::decode(value)?;
        Ok(s.0)
    }
}
