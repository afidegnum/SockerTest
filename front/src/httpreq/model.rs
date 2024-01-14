use serde::{Deserialize, Serialize};

#[derive(Serialize, Debug, Clone, Deserialize, Default)]
pub struct ContentList {
    pub id: i32,
    pub title: String,
    pub media_id: Option<String>,
}

#[derive(Deserialize, PartialEq, Debug, Serialize, Clone)]
pub struct ContentViews {
    pub title: String,
    pub url: String,
    pub media_id: String,
    pub type_name: String,
}

#[derive(Deserialize, Debug, Clone, Serialize, PartialEq)]
pub struct BooksViews {
    pub Book_Title: String,
    pub ISBN: String,
    pub Author_Name: String,
    pub Country: String,
}
