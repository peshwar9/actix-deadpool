#[cfg(test)]
mod tests {
    use super::*;


// Standard lib
// External crates - Primary
use actix_web::{test, web, App};
// External crates - Utilities
use lazy_static::lazy_static;
use dotenv::dotenv;
use crate::config::Config;
use crate::handler;
use crate::model;
// Other internal modules
// Const and type declarations
// Struct declarations
// Functions

lazy_static! {
    static ref APP_STATE: Pool  = {
        dotenv().ok();

        let config = Config::env().unwrap();
        let pool = config.db.create_pool(tokio_postgres::NoTls).unwrap();
        let address = format!("{}:{}", config.server.host, config.server.port);
        pool
    }
}

#[actix_rt::test]
async fn test_get_todos() {
    let app = App::new()
        .data(APP_STATE.clone())
        .route("/list",web::get().to(handler::get_lists_handler));

    let mut app = test::init_service(app).await;   
    let req = test::TestRequest::get().uri("/list").to_request();

    let response = test::call_service(&mut app,req).await;

    assert_eq!(response.status(), 200, "GET /list should return 200");

}

}