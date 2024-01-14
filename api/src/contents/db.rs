use crate::{
    contents::{Content, CreateContent},
    errors::ServiceError,
};
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

use super::ContentList;

// CORE CRUD

//TODO configure .env for db shema name

// Decide wether to return id or return all fields from insert sql query . if return ID, insert id in function argument.
// shift id in db tables to the top so we can skip it when not needed

pub async fn content_add(client: &Client, selfobj: CreateContent) -> Result<Content, io::Error> {
    let statement = client
        .prepare(
            "INSERT INTO public.contents
   (title, slug, summary, content)
    VALUES ($0, $1, $2, $3) RETURNING slug, title, summary, content",
        )
        .await
        .unwrap();

    client
        .query(
            &statement,
            &[
                &selfobj.slug,
                &selfobj.title,
                &selfobj.summary,
                &selfobj.details,
            ],
        )
        .await
        .expect("Error creating content")
        .iter()
        .map(|row| Content::from_row_ref(row).unwrap())
        .collect::<Vec<Content>>()
        .pop()
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Error creating content tables",
        ))
}

// TODO populate fields

pub async fn content_list(client: &Client) -> Result<Vec<ContentList>, ServiceError> {
    let statement = client
        .prepare("select id, title, media_id from public.contents order by id desc")
        .await
        .unwrap();

    let content_list = client
        .query(&statement, &[])
        .await
        .expect("Error getting author lists")
        .iter()
        .map(|row| ContentList::from_row_ref(row).unwrap())
        .collect::<Vec<ContentList>>();

    Ok(content_list)
}

pub async fn content_id(client: &Client, id_content: i32) -> Result<Content, io::Error> {
    let statement = client
        .prepare("select * from public.contents where id = $1")
        .await
        .unwrap();

    let maybe_content = client
        .query_opt(&statement, &[&id_content])
        .await
        .expect("Error fetching content ")
        .map(|row| Content::from_row_ref(&row).unwrap());

    match maybe_content {
        Some(content) => Ok(content),
        None => Err(io::Error::new(io::ErrorKind::NotFound, "Not found")),
    }
}

pub async fn content_search(
    client: &Client,
    content_search: String,
) -> Result<Vec<Content>, io::Error> {
    let statement = client
        .prepare("select * from public.contents where title LIKE %$1%")
        .await
        .unwrap();

    let maybe_content = client
        .query(&statement, &[&content_search])
        .await
        .expect("Error fetching content ")
        .iter()
        .map(|row| Content::from_row_ref(&row).unwrap())
        .collect::<Vec<Content>>();
    Ok(maybe_content)
}

//TODO take into account ID position

pub async fn content_update(client: &Client, id: i32, mdl: CreateContent) -> Result<(), io::Error> {
    let statement = client.prepare("update public.contents set (slug, title, summary, content) = ($1, $2, $3, $4) where id = $5").await.unwrap();

    let result = client
        .execute(
            &statement,
            &[&mdl.slug, &mdl.title, &mdl.summary, &mdl.details],
        )
        .await
        .expect("Error updating content");

    match result {
        ref updated if *updated == 1 => Ok(()),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Failed to check list")),
    }
}

pub async fn content_delete(client: &Client, content_id: i32) -> Result<(), io::Error> {
    let statement = client
        .prepare("DELETE FROM public.contents WHERE id = $1")
        .await
        .unwrap();

    client.execute(&statement, &[&content_id]).await.unwrap();
    Ok(())
}

// END OF CORE CRUD
