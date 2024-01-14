use crate::contents::db;
use crate::contents::models::CreateContent;
use std::io;

use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use io::ErrorKind::NotFound;

/// Get list of contents.
///
/// List contentss from in-memory content store.
///
/// One could call the api endpoint with following curl.
/// ```text
/// curl localhost:8080/contents
/// ```
#[utoipa::path(
    context_path = "/contents",
    responses(
        (status = 200, description = "Content lists", body = [Content]),
    )
)]
#[get("/")]
pub async fn contents(db_pool: web::Data<Pool>) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::content_list(&client).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

/// Create new Content to shared in-memory storage.
///
/// Content a new `Todo` in request body as json to store it. Api will return
/// created `Todo` on success or `ErrorResponse::Conflict` if todo with same id already exists.
///
/// One could call the api with.
/// ```text
/// curl localhost:8080/todo -d '{"id": 1, "value": "Buy movie ticket", "checked": false}'
/// ```
#[utoipa::path(
     context_path = "/contents",
    request_body = CreateContent,
    responses(
        (status = 201, description = "Category Successfully added", body = Content),
        (status = 409, description = "Category with id already exists", body = ServiceError)
    )
)]
#[post("/")]
pub async fn add_contents(
    local_object: web::Json<CreateContent>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::content_add(&client, local_object.clone()).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

/// Get Category by given todo id.
///
/// Return found `Category` with status 200 or 404 not found if `Category` is not found from db.
#[utoipa::path(
    context_path = "/contents",
    responses(
        (status = 200, description = "Content", body = Content),
        (status = 404, description = "Content not found by id", body = ServiceError)
    ),
    params(
        ("id", description = "Unique Content Id")
    )
)]
#[get("/{id}")]
pub async fn get_contents(
    id_contents: web::Path<(i32,)>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::content_id(&client, id_contents.0).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

/// Delete Content by given path variable id.
///
/// This endpoint needs `api_key` authentication in order to call. Api key can be found from README.md.
///
/// Api will delete todo from shared in-memory storage by the provided id and return success 200.
/// If storage does not contain `Todo` with given id 404 not found will be returned.
#[utoipa::path(
     context_path = "/contents",
    responses(
        (status = 200, description = "Content deleted successfully"),
        (status = 401, description = "Unauthorized to delete Content", body = ServiceError),
        (status = 404, description = "Content not found by id", body = ServiceError)
    ),
    params(
        ("id", description = "Unique storage id of Category")
    ),
    security(
        ("api_key" = [])
    )
)]
#[delete("/{id}")]
pub async fn delete_contents(
    contents_id: web::Path<(i32,)>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::content_delete(&client, contents_id.0).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

/// Update Todo with given id.
///
/// This endpoint supports optional authentication.
///
/// Tries to update `Todo` by given id as path variable. If todo is found by id values are
/// updated according `TodoUpdateRequest` and updated `Todo` is returned with status 200.
/// If todo is not found then 404 not found is returned.
#[utoipa::path(
     context_path = "/contents",
    request_body = CreateContent,
    responses(
        (status = 200, description = "Category updated successfully", body = Content),
        (status = 404, description = "Category not found by id", body = ServiceError)
    ),
    params(
        ("id", description = "Unique storage id of Category")
    ),
    security(
        (),
        ("api_key" = [])
    )
)]
#[patch("/{id}")]
pub async fn update_contents(
    id_contents: web::Path<(i32,)>,
    local_object: web::Json<CreateContent>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::content_update(&client, id_contents.0, local_object.clone()).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(contents);
    cfg.service(add_contents);
    cfg.service(update_contents);
    cfg.service(get_contents);
    cfg.service(delete_contents);
}

// #[delete("/{id}")]
// pub async fn delete_author(id_author: web::Path<(i32,)>,  db_pool: web::Data<Pool>) -> impl Responder {
//     let res = format!("{:?},", id_author.0);
//     println!("{:#?}", res);
//     res
// }
