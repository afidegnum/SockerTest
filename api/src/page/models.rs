use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
extern crate chrono;
use postgres_from_row::FromRow;
use utoipa::{IntoParams, ToResponse, ToSchema};
///// start of views

#[derive(Deserialize, Debug, Serialize, Clone, FromRow)]
pub struct ContentViews {
    pub title: String,
    pub url: String,
    pub media_id: String,
    pub type_name: String,
}

#[derive(Deserialize, Debug, FromRow, Serialize, Clone)]
pub struct BooksViews {
    pub Book_Title: Option<String>,
    pub ISBN: Option<String>,
    pub Author_Name: Option<String>,
    pub Country: Option<String>,
}

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct ContentViewsRow {
    pub title: String,
    pub url: String,
    pub media_id: String,
    pub type_name: String,
}

////////////////////// ENDOF VIEWS
//To be added based on special query
#[derive(
    Serialize, Debug, ToSchema, Clone, ToResponse, IntoParams, Deserialize, PostgresMapper, Default,
)]
#[schema(example = json!({"class": "post inline"}))]
#[response(description = "Category Lists")]
#[pg_mapper(table = "categories")]
pub struct Category {
    pub id: i32,
    pub name: String,
    pub slug: String,
    pub description: String,
}

#[derive(
    Serialize, Debug, ToSchema, Clone, ToResponse, IntoParams, Deserialize, PostgresMapper, Default,
)]
#[schema(example = json!({"class": "form inline"}))]
#[response(description = "Add a new category")]
#[pg_mapper(table = "categories")]
pub struct CreateCategory {
    pub name: String,
    pub slug: String,
    pub description: String,
}

/// Search todos Query

#[derive(Deserialize, Debug, Clone, IntoParams, PostgresMapper)]
#[pg_mapper(table = "categories")]
pub struct SearchCategory {
    /// Content that should be found from Todo's value field
    pub name: String,
}
