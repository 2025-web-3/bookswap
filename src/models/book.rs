use {
    crate::{
        routes::{HttpError, Result as HttpResult},
        utils::{openlibrary::get_open_library_books, snowflake::Snowflake},
    },
    chrono::NaiveDateTime,
    serde::{Deserialize, Serialize},
    serde_repr::*,
    sqlx::{sqlite::SqliteValueRef, Decode, Sqlite, SqliteExecutor},
};

#[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug, Clone, Copy)]
#[repr(i64)]
pub enum BookCondition {
    /// The book is in perfect condition, as if new
    MintCondition = 0,
    /// The book is nearly perfect, with minimal signs of use
    NearPerfect = 1,
    /// The book has been gently used, showing minimal wear
    GentlyUsed = 2,
    /// The book shows signs of use, such as abrasions or lightly torn pages
    ShowsSomeWear = 3,
    /// The book is significantly damaged, with medium to heavy tears (even if repaired)
    HeavilyUsed = 4,
}

impl Into<i64> for BookCondition {
    fn into(self) -> i64 {
        self as i64
    }
}

impl From<i64> for BookCondition {
    fn from(value: i64) -> BookCondition {
        match value {
            0 => BookCondition::MintCondition,
            1 => BookCondition::NearPerfect,
            2 => BookCondition::GentlyUsed,
            3 => BookCondition::ShowsSomeWear,
            4 => BookCondition::HeavilyUsed,
            _ => BookCondition::MintCondition,
        }
    }
}

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

    pub async fn from_isbn(id: Snowflake, isbn: &str) -> HttpResult<Self> {
        let isbnf = format!("ISBN:{}", &isbn);

        let mut book_map =
            get_open_library_books(isbnf.clone()).await.map_err(|_| HttpError::UnknownBook)?;
        let book = book_map.remove(&isbnf).ok_or_else(|| HttpError::UnknownBook)?;

        let title = book.title.as_deref().unwrap_or("No Title");
        let description = book
            .excerpts
            .as_ref()
            .and_then(|e| e.first())
            .map(|e| e.text.as_str())
            .unwrap_or("No Description");

        let author_name = book
            .authors
            .as_ref()
            .and_then(|a| a.first())
            .map(|a| a.name.as_str())
            .unwrap_or("Unknown Author");

        let subject_name = book.subjects.and_then(|s| s.first().map(|s| s.name.clone()));
        let cover_url = book.cover.and_then(|cover| cover.medium);

        Ok(Book::new(
            id,
            Some(isbn.to_string()),
            title,
            description,
            author_name,
            subject_name,
            book.number_of_pages,
            cover_url,
            None,
        ))
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
pub struct BookSharingRaw {
    pub id: Snowflake,
    pub book_id: Snowflake,
    pub comment: Option<String>,
    pub holder_id: Snowflake,
    pub condition: i64,
}

#[derive(Serialize)]
pub struct BookSharing {
    pub id: Snowflake,
    pub book: Book,
    pub comment: Option<String>,
    pub holder_id: Snowflake,
    pub condition: BookCondition,
}

impl BookSharing {
    pub fn new(
        id: Snowflake, book: Book, comment: Option<String>, holder_id: Snowflake,
        condition: BookCondition,
    ) -> Self {
        Self { id, book, comment, holder_id, condition }
    }

    pub async fn save<'a, E: SqliteExecutor<'a>>(self, executor: E) -> HttpResult<Self> {
        let condition: i64 = self.condition.into();

        sqlx::query!(r#"INSERT INTO books_sharing(id, book_id, comment, holder_id, condition) VALUES ($1, $2, $3, $4, $5)"#,
            self.id.0, self.book.id.0, self.comment, self.holder_id.0, condition)
            .execute(executor).await
            .map(|_| self)
            .map_err(HttpError::Database)
    }
}

#[derive(Serialize)]
pub struct BookRequest {
    pub id: Snowflake,
    pub book_sharing_id: Snowflake,
    pub book_id: Snowflake,
    pub borrower_id: Snowflake,
    pub is_accepted: Option<bool>,
    pub accepted_at: Option<NaiveDateTime>,
    pub borrowed_at: Option<NaiveDateTime>,
    pub return_at: Option<NaiveDateTime>,
}

impl BookRequest {
    pub fn new(
        id: Snowflake, book_sharing_id: Snowflake, book_id: Snowflake, borrower_id: Snowflake,
    ) -> Self {
        Self {
            id,
            book_sharing_id,
            book_id,
            borrower_id,
            is_accepted: None,
            accepted_at: None,
            borrowed_at: None,
            return_at: None,
        }
    }

    pub async fn save<'a, E: SqliteExecutor<'a>>(self, executor: E) -> HttpResult<Self> {
        sqlx::query!(r#"INSERT INTO books_requests(id, book_sharing_id, book_id, borrower_id) VALUES ($1, $2, $3, $4)"#,
            self.id.0, self.book_sharing_id.0, self.book_id.0, self.borrower_id.0
        )
            .execute(executor).await
            .map(|_| self)
            .map_err(HttpError::Database)
    }
}

impl Decode<'_, Sqlite> for Book {
    fn decode(
        value: SqliteValueRef<'_>,
    ) -> Result<Self, Box<dyn std::error::Error + 'static + Send + Sync>> {
        let s: sqlx::types::Json<Book> = sqlx::Decode::<'_, Sqlite>::decode(value)?;
        Ok(s.0)
    }
}
