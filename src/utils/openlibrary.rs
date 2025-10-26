use reqwest;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Deserialize, Clone)]
pub struct Author {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Publisher {
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct PublishPlace {
    pub name: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Subject {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Excerpt {
    pub comment: String,
    pub text: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Link {
    pub url: String,
    pub title: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Cover {
    pub small: Option<String>,
    pub medium: Option<String>,
    pub large: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ebook {
    pub preview_url: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Identifiers {
    #[serde(flatten)]
    pub ids: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Classifications {
    #[serde(flatten)]
    pub classes: HashMap<String, Vec<String>>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct BookData {
    pub url: Option<String>,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub authors: Option<Vec<Author>>,
    pub identifiers: Option<Identifiers>,
    pub classifications: Option<Classifications>,
    pub subjects: Option<Vec<Subject>>,
    pub subject_places: Option<Vec<Subject>>,
    pub subject_people: Option<Vec<Subject>>,
    pub subject_times: Option<Vec<Subject>>,
    pub publishers: Option<Vec<Publisher>>,
    pub publish_places: Option<Vec<PublishPlace>>,
    pub publish_date: Option<String>,
    pub excerpts: Option<Vec<Excerpt>>,
    pub links: Option<Vec<Link>>,
    pub cover: Option<Cover>,
    pub ebooks: Option<Vec<Ebook>>,
    pub number_of_pages: Option<i64>,
    pub weight: Option<String>,
}

type OpenLibraryResponse = HashMap<String, BookData>;

pub async fn get_open_library_books(isbn: String) -> Result<OpenLibraryResponse, reqwest::Error> {
    const BASE_URL: &str = "https://openlibrary.org/api/books";

    let url = format!("{}?bibkeys={}&format=json&jscmd=data", BASE_URL, isbn);

    let client = reqwest::Client::new();

    let response: OpenLibraryResponse =
        client.get(&url).send().await?.error_for_status()?.json().await?;

    Ok(response)
}
