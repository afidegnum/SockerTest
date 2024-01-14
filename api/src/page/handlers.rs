// use crate::category::models::CreateCategory;
// use crate::page::{db, SearchCategory};

use crate::page::db;

use crate::errors::ServiceError;
use std::io;

use actix_web::web::Query;
use actix_web::{delete, get, patch, post, web, HttpRequest, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use io::ErrorKind::NotFound;

////// Views
#[get("/books")]
pub async fn content_views(db_pool: web::Data<Pool>, req: HttpRequest) -> impl Responder {
    //  log::info!("HttpRequest {req:?}");
    let newr = format!("HttpReq: {:#?}", req);
    println!("{}", newr);
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::fetch_books_views(&client).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

#[get("/{id_category}/{limit}")]
pub async fn filterred_views(
    path: web::Path<(String, i64)>,
    db_pool: web::Data<Pool>,
) -> impl Responder {
    let (type_name, limit) = path.into_inner();
    let client: Client = db_pool
        .get()
        .await
        .expect("Error connecting to the database");

    let result = db::filter_content_views(&client, type_name, limit).await;

    match result {
        Ok(object) => HttpResponse::Ok().json(object),
        Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

///// Views end

//     match result {
//         Ok(object) => HttpResponse::Ok().json(object),
//         Err(_) => HttpResponse::InternalServerError().into(),
//     }
// }

// /// Create new Todo to shared in-memory storage.
// ///
// /// Post a new `Todo` in request body as json to store it. Api will return
// /// created `Todo` on success or `ErrorResponse::Conflict` if todo with same id already exists.
// ///
// /// One could call the api with.
// /// ```text
// /// curl localhost:8080/todo -d '{"id": 1, "value": "Buy movie ticket", "checked": false}'
// /// ```
// #[utoipa::path(
//      context_path = "/categories",
//     request_body = CreateCategory,
//     responses(
//         (status = 201, description = "Category Successfully added", body = Category),
//         (status = 409, description = "Category with id already exists", body = ServiceError)
//     )
// )]
// #[post("/")]
// pub async fn add_category(
//     local_object: web::Json<CreateCategory>,
//     db_pool: web::Data<Pool>,
// ) -> impl Responder {
//     let client: Client = db_pool
//         .get()
//         .await
//         .expect("Error connecting to the database");

//     let result = db::category_add(&client, local_object.clone()).await;

//     match result {
//         Ok(object) => HttpResponse::Ok().json(object),
//         Err(_) => HttpResponse::InternalServerError().into(),
//     }
// }

// /// Get Category by given todo id.
// ///
// /// Return found `Category` with status 200 or 404 not found if `Category` is not found from db.
// #[utoipa::path(
//      context_path = "/categories",
//     responses(
//         (status = 200, description = "Category", body = Category),
//         (status = 404, description = "Category not found by id", body = ServiceError)
//     ),
//     params(
//         ("id", description = "Unique Category Id")
//     )
// )]
// #[get("/{id}")]
// pub async fn get_category(
//     id_category: web::Path<(i32,)>,
//     db_pool: web::Data<Pool>,
// ) -> impl Responder {
//     let client: Client = db_pool
//         .get()
//         .await
//         .expect("Error connecting to the database");

//     let result = db::category_id(&client, id_category.0).await;

//     match result {
//         Ok(object) => HttpResponse::Ok().json(object),
//         Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
//         Err(_) => HttpResponse::InternalServerError().into(),
//     }
// }

// /// Search Todos with by value
// ///
// /// Perform search from `Todo`s present in in-memory storage by matching Todo's value to
// /// value provided as query parameter. Returns 200 and matching `Todo` items.
// #[utoipa::path(
//      context_path = "/categories",
//     params(
//         SearchCategory
//     ),
//     responses(
//         (status = 200, description = "Search Todos did not result error", body = [Category]),
//     )
// )]
// #[get("/{name}")]
// pub async fn search_category(
//     query: Query<SearchCategory>,
//     db_pool: web::Data<Pool>,
// ) -> impl Responder {
//     // let qr = query.clone();
//     let client: Client = db_pool
//         .get()
//         .await
//         .expect("Error connecting to the database");

//     let result = db::category_search(&client, &query.name).await;

//     match result {
//         Ok(object) => HttpResponse::Ok().json(object),
//         Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
//         Err(_) => HttpResponse::InternalServerError().into(),
//     }
// }

// /// Delete Category by given path variable id.
// ///
// /// This endpoint needs `api_key` authentication in order to call. Api key can be found from README.md.
// ///
// /// Api will delete todo from shared in-memory storage by the provided id and return success 200.
// /// If storage does not contain `Todo` with given id 404 not found will be returned.
// #[utoipa::path(
//     context_path = "/categories",
//     responses(
//         (status = 200, description = "Category deleted successfully"),
//         (status = 401, description = "Unauthorized to delete Category", body = ServiceError),
//         (status = 404, description = "Category not found by id", body = ServiceError)
//     ),
//     params(
//         ("id", description = "Unique storage id of Category")
//     ))]
// #[delete("/{id}")]
// pub async fn delete_category(
//     category_id: web::Path<(i32,)>,
//     db_pool: web::Data<Pool>,
// ) -> impl Responder {
//     let client: Client = db_pool
//         .get()
//         .await
//         .expect("Error connecting to the database");

//     let result = db::category_delete(&client, category_id.0).await;

//     match result {
//         Ok(object) => HttpResponse::Ok().json(object),
//         Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
//         Err(_) => HttpResponse::InternalServerError().into(),
//     }
// }

// /// Update Todo with given id.
// ///
// /// This endpoint supports optional authentication.
// ///
// /// Tries to update `Todo` by given id as path variable. If todo is found by id values are
// /// updated according `TodoUpdateRequest` and updated `Todo` is returned with status 200.
// /// If todo is not found then 404 not found is returned.
// #[utoipa::path(
//      context_path = "/categories",
//     request_body = TodoUpdateRequest,
//     responses(
//         (status = 200, description = "Category updated successfully", body = CreateCategory),
//         (status = 404, description = "Category not found by id", body = ServiceError)
//     ),
//     params(
//         ("id", description = "Unique storage id of Category")
//     )
// )]
// #[patch("/{id}")]
// pub async fn update_category(
//     id_category: web::Path<(i32,)>,
//     local_object: web::Json<CreateCategory>,
//     db_pool: web::Data<Pool>,
// ) -> impl Responder {
//     let client: Client = db_pool
//         .get()
//         .await
//         .expect("Error connecting to the database");

//     let result = db::category_update(&client, id_category.0, local_object.clone()).await;

//     match result {
//         Ok(object) => HttpResponse::Ok().json(object),
//         Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
//         Err(_) => HttpResponse::InternalServerError().into(),
//     }
// }

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(content_views);
    cfg.service(filterred_views);
}

// #[delete("/{id}")]
// pub async fn delete_author(id_author: web::Path<(i32,)>,  db_pool: web::Data<Pool>) -> impl Responder {
//     let res = format!("{:?},", id_author.0);
//     println!("{:#?}", res);
//     res
// }
//         .expect("Error connecting to the database");

//     let result = db::category_update(&client, id_category.0, local_object.clone()).await;

//     match result {
//         Ok(object) => HttpResponse::Ok().json(object),
//         Err(ref e) if e.kind() == NotFound => HttpResponse::NotFound().into(),
//         Err(_) => HttpResponse::InternalServerError().into(),
//     }
// }

// #[delete("/{id}")]
// pub async fn delete_author(id_author: web::Path<(i32,)>,  db_pool: web::Data<Pool>) -> impl Responder {
//     let res = format!("{:?},", id_author.0);
//     println!("{:#?}", res);
//     res
// }
