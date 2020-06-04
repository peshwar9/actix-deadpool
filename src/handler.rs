// Standard lib
// External crates - Primary
use actix_web::{Responder, web, HttpResponse};
use deadpool_postgres::{Client,Pool};
// External crates - Utilities
// Other internal modules
use crate::repo::{create_list_db, get_lists_db,update_list_db, delete_list_db};
use crate::model::{NewList, UpdateList};
// Const and type declarations
// Struct declarations
// Functions


pub async fn index_handler(pool: web::Data<Pool>) -> impl Responder {

    let client: Client = pool.get()
                    .await
                    .expect("Unable to connect to database");

let result = get_lists_db(&client)
                    .await;
    match result {
        Ok(lists) => HttpResponse::Ok().json(lists),
        Err(_) => HttpResponse::NotFound().into()
    }
}


pub async fn create_list_handler(pool: web::Data<Pool>, json: web::Json<NewList>) -> impl Responder {
    let client: Client = pool.get()
                        .await
                        .expect("Unable to connect to database");
    let cat = match  &json.category {
        Some(c) => c,
        None => ""
    };
    let result = create_list_db(&client,&json.title.clone(), cat.clone())
                .await;
            
    match result {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(_) => HttpResponse::NotFound().into()
    }

}

pub async fn update_list_handler(pool: web::Data<Pool>, path: web::Path::<(i32,)>, json: web::Json<UpdateList>) -> impl Responder {
    let client: Client = pool.get()
                .await
                .expect("Unable to connect to database");

    let result = update_list_db(&client,path.0, json.into() )
                .await;
            
    match result {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(_) => HttpResponse::NotFound().into()
    }

}

pub async fn delete_list_handler(pool: web::Data<Pool>, path: web::Path::<(i32,)>) -> impl Responder {
    let client: Client = pool.get()
                .await
                .expect("Unable to connect to database");
    let result =delete_list_db(&client, path.0)
            .await;
    match result {
        Ok(_) => HttpResponse::Ok(),
        Err(_) => HttpResponse::InternalServerError().into()
    }
}