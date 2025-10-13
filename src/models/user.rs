use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i64,
    pub username: String,
    pub first_name: String,
    pub second_name: String,
    #[serde(default, skip)]
    pub password_hash: String,
    pub school_name: Option<String>,
    pub permissions: i64,
}

impl User {
    pub async fn new() -> Option<Self> {
        return None;
    }
}
