use {
    crate::{
        models::{
            book::{Book, BookRequest, BookSharing, BookSharingRaw},
            session::Session,
            user::User,
        },
        utils::snowflake::Snowflake,
    },
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

    pub async fn fetch_user(&self, user_id: Snowflake) -> Option<User> {
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

        self.fetch_user(user_id).await
    }

    pub async fn fetch_book(&self, book_id: Snowflake) -> Option<Book> {
        sqlx::query_as!(Book, "SELECT * FROM books WHERE id = $1", book_id.0)
            .fetch_optional(&self.pool)
            .await
            .ok()?
    }

    pub async fn fetch_book_by_isbn(&self, isbn: &str) -> Option<Book> {
        sqlx::query_as!(Book, "SELECT * FROM books WHERE isbn = $1", isbn)
            .fetch_optional(&self.pool)
            .await
            .ok()?
    }

    pub async fn fetch_sharing(&self, sharing_id: Snowflake) -> Option<BookSharing> {
        let raw = sqlx::query_as!(
            BookSharingRaw,
            "SELECT * FROM books_sharing WHERE id = $1",
            sharing_id.0
        )
        .fetch_optional(&self.pool)
        .await
        .ok()??;

        self.fetch_book(raw.book_id).await.map(|book| BookSharing {
            id: raw.id,
            comment: raw.comment,
            condition: raw.condition.into(),
            holder_id: raw.holder_id,
            book,
        })
    }

    pub async fn fetch_sharings_by_book_and_holder_id(
        &self, holder_id: Snowflake, book_id: Snowflake,
    ) -> Option<Vec<BookSharing>> {
        let raws = sqlx::query_as!(
            BookSharingRaw,
            r#"SELECT * FROM books_sharing WHERE holder_id = $1 AND book_id = $2"#,
            holder_id.0,
            book_id.0
        )
        .fetch_all(&self.pool)
        .await
        .ok()?;

        let mut sharings = Vec::with_capacity(raws.len());

        for raw in raws {
            if let Some(book) = self.fetch_book(raw.book_id).await {
                sharings.push(BookSharing {
                    id: raw.id,
                    book,
                    holder_id: raw.holder_id,
                    comment: raw.comment,
                    condition: raw.condition.into(),
                });
            }
        }

        Some(sharings)
    }

    pub async fn fetch_all_unique_shared_books(&self, holder_id: Snowflake) -> Option<Vec<Book>> {
        let unique_book_ids_raw: Vec<(i64,)> =
            sqlx::query_as(r#"SELECT DISTINCT book_id FROM books_sharing WHERE holder_id = $1"#)
                .bind(holder_id.0)
                .fetch_all(&self.pool)
                .await
                .ok()?;

        let mut books = Vec::with_capacity(unique_book_ids_raw.len());

        for (book_id,) in unique_book_ids_raw {
            if let Some(book) = self.fetch_book(book_id.into()).await {
                books.push(book);
            }
        }

        Some(books)
    }

    pub async fn fetch_holders_by_book_id(&self, book_id: Snowflake) -> Option<Vec<User>> {
        let unique_holder_ids_raw: Vec<(i64,)> =
            sqlx::query_as(r#"SELECT DISTINCT holder_id FROM books_sharing WHERE book_id = $1"#)
                .bind(book_id.0)
                .fetch_all(&self.pool)
                .await
                .ok()?;

        let mut holders = Vec::with_capacity(unique_holder_ids_raw.len());

        for (holder_id,) in unique_holder_ids_raw {
            if let Some(user) = self.fetch_user(holder_id.into()).await {
                holders.push(user);
            }
        }

        Some(holders)
    }

    pub async fn fetch_user_requested_books(
        &self, holder_id: Snowflake,
    ) -> Option<Vec<BookRequest>> {
        sqlx::query_as!(
            BookRequest,
            r#"
            SELECT
                br.id,
                bs.id AS "book_sharing_id",
                br.book_id,
                br.borrower_id,
                br.is_accepted,
                br.accepted_at,
                br.borrowed_at,
                br.return_at
            FROM
                books_requests AS br
            JOIN
                books_sharing AS bs ON br.book_sharing_id = bs.id
            WHERE
                bs.holder_id = $1
        "#,
            holder_id.0
        )
        .fetch_all(&self.pool)
        .await
        .ok()
    }
}
