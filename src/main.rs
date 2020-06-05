use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::io;


// Module declarations

mod handler;
mod model;
mod config;
mod repo;
mod errors;

use crate::config::Config;
use handler::{index_handler,create_list_handler, delete_list_handler, update_list_handler};

#[actix_rt::main]
async fn main() -> io::Result<()> {

    // Load environment variables from .env file
    dotenv().ok();

    let config = Config::env().unwrap();
    let pool = config.db.create_pool(tokio_postgres::NoTls).unwrap();
    let address = format!("{}:{}", config.server.host, config.server.port);
    
    HttpServer::new( move || {
        App::new()
        .data(pool.clone())
        .route("/",web::get().to(index_handler))
        .route("/",web::post().to(create_list_handler))
        .route("/{task_id}", web::delete().to(delete_list_handler))
        .route("/{task_id}", web::put().to(update_list_handler))
    })
    .bind(address)?
    .run()
    .await


}
