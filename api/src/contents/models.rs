use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

use utoipa::ToSchema;
//To be added based on special query

#[derive(Serialize, Debug, Clone, Deserialize, ToSchema, PostgresMapper, Default)]
#[pg_mapper(table = "contents")]
pub struct Content {
    pub id: i32,
    pub title: String,
    pub slug: Option<String>,
    pub summary: String,
    pub details: String,
    // pub date_created: chrono::DateTime<Utc>,
    // pub date_published: chrono::DateTime<Utc>,
}

#[derive(Serialize, Debug, Clone, Deserialize, ToSchema, PostgresMapper, Default)]
#[pg_mapper(table = "contents")]
pub struct CreateContent {
    pub title: String,
    pub slug: String,
    pub summary: String,
    pub details: String,
}

#[derive(Serialize, Debug, Clone, Deserialize, ToSchema, PostgresMapper, Default)]
#[pg_mapper(table = "contents")]
pub struct ContentList {
    pub id: i32,
    pub title: String,
    pub media_id: Option<String>,
}
