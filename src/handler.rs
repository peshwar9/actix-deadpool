// Standard lib
// External crates - Primary
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
// External crates - Utilities
// Other internal modules
use crate::model::{NewList, UpdateList};
use crate::repo::{create_list_db, delete_list_db, get_lists_db, update_list_db};
// Const and type declarations
// Struct declarations
// Functions

// Index Handler
// http localhost:12345/health
pub async fn health_handler(_pool: web::Data<Pool>) -> impl Responder {

 HttpResponse::Ok().json("Feeling bright and shining.")
}

// Get Lists Handler
// http localhost:12345/lists
pub async fn get_lists_handler(pool: web::Data<Pool>) -> impl Responder {
    let client: Client = pool.get().await?;

    get_lists_db(&client)
        .await
        .map(|lists| HttpResponse::Ok().json(lists))

}

pub async fn create_list_handler(
    pool: web::Data<Pool>,
    json: web::Json<NewList>,
) -> impl Responder {
    let client: Client = pool.get().await?;

    create_list_db(&client, json.into())
        .await
        .map(|list| HttpResponse::Ok().json(list))

}

pub async fn update_list_handler(
    pool: web::Data<Pool>,
    path: web::Path<(i32,)>,
    json: web::Json<UpdateList>,
) -> impl Responder {
 
    let client: Client = pool.get().await?;

    update_list_db(&client, path.0, json.into())
    .await
    .map(|list| HttpResponse::Ok().json(list))

}

pub async fn delete_list_handler(pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    
    let client: Client = pool.get().await?;

    delete_list_db(&client, path.0)
    .await
    .map(|_| HttpResponse::Ok())

}
