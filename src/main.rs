// Standard lib
use std::io;
// External crates - Primary
use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
// External crates - Utilities
use dotenv::dotenv;
// Other internal modules
use crate::config::Config;
use crate::auth::validator;
use handler::{
    create_list_handler, 
    delete_list_handler, 
    health_handler, 
    get_lists_handler, 
    update_list_handler, 
    get_one_list_handler,
    get_token,    
};

// Const and type declarations
// Struct declarations

// Module declarations

mod config;
mod errors;
mod handler;
mod model;
mod repo;
mod auth;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    // Load environment variables from .env file
    dotenv().ok();

    let config = Config::env().unwrap();
    let pool = config.db.create_pool(tokio_postgres::NoTls).unwrap();
    let address = format!("{}:{}", config.server.host, config.server.port);
    let auth = HttpAuthentication::bearer(validator);

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .route("/health", web::get().to(health_handler))
            .route("/auth", web::get().to(get_token))
            .service(
                web::scope("/list")
                .wrap(auth.clone())
                .route("/", web::get().to(get_lists_handler))
                .route("/{list_id}", web::get().to(get_one_list_handler))
                .route("/", web::post().to(create_list_handler))
                .route("/{list_id}", web::delete().to(delete_list_handler))
                .route("/{list_id}", web::put().to(update_list_handler))
    )})
    .bind(address)?
    .run()
    .await
}
