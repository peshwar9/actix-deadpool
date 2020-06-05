// Standard lib
// External crates - Primary
use actix_web::{web, HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
// External crates - Utilities
// Other internal modules
use crate::model::{NewList, UpdateList};
use crate::repo::{create_list_db, delete_list_db, get_lists_db, update_list_db, get_one_list_db};
use crate::errors::ApiError;
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
pub async fn get_lists_handler(pool: web::Data<Pool>) -> Result<HttpResponse, ApiError> {
    let client: Client = pool.get().await?;

    get_lists_db(&client)
        .await
        .map(|lists| HttpResponse::Ok().json(lists))

}

// Create list handler
// http post localhost:12345/list title=shinylist category=work
pub async fn create_list_handler(
    pool: web::Data<Pool>,
    json: web::Json<NewList>,
) -> Result<HttpResponse, ApiError> {
    let client: Client = pool.get().await?;

    create_list_db(&client, json.into())
        .await
        .map(|list| HttpResponse::Ok().json(list))

}

// Update list handler
// http put localhost:12345/list/1 title=notsoshinylist
pub async fn update_list_handler(
    pool: web::Data<Pool>,
    path: web::Path<(i32,)>,
    json: web::Json<UpdateList>,
) -> Result<HttpResponse, ApiError> {
 
    let client: Client = pool.get().await?;

    update_list_db(&client, path.0, json.into())
    .await
    .map(|list| HttpResponse::Ok().json(list))

}

// Delete list handler
// http delete localhost:12345/list/1
pub async fn delete_list_handler(pool: web::Data<Pool>, path: web::Path<(i32,)>) -> Result<HttpResponse, ApiError> {
    
    let client: Client = pool.get().await?;

    delete_list_db(&client, path.0)
    .await
    .map(|_| HttpResponse::Ok().json("Deleted record"))

}

// Get one list handler
// http localhost:12345/list/1
pub async fn get_one_list_handler(pool: web::Data<Pool>, path: web::Path::<(i32,)>) -> Result<HttpResponse, ApiError> {
    let client: Client = pool.get().await?;

    get_one_list_db(&client,path.0)
    .await
    .map(|list| HttpResponse::Ok().json(list))
}


#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App,};
    use actix_web::{HttpRequest, web::Path, web::Json, HttpResponse, HttpMessage};
    use actix_web::http::{header, StatusCode};
    use lazy_static::lazy_static;
    use dotenv::dotenv;
    use crate::config::Config;
    use crate::handler;
    use crate::model;
    use serde_json::json;

fn get_pool() -> Pool{
    dotenv().ok();

    let config = Config::env().unwrap();
    let pool: Pool = config.db.create_pool(tokio_postgres::NoTls).unwrap();
    let address = format!("{}:{}", config.server.host, config.server.port);
    pool
}
    #[actix_rt::test]
    // To run all tests and use println: cargo test  -- --nocapture
    // --show-output flag can be used for summary of tests run
    async fn test_get_list() {

        let app_state: web::Data<Pool> = web::Data::new(get_pool());
        let req = test::TestRequest::with_header("content-type", "text/plain")
        .to_http_request();
        let resp = get_lists_handler(app_state).await.unwrap();
        println!("Response is {:?}", resp);
      //  println!("Response body is {:?}", resp.body());
        assert_eq!(resp.status(), StatusCode::OK);
       // assert_eq!(resp.body(),);
      //  let body = resp.to_string();
   
     //   let list_body: Result<Vec<model::List>, serde_json::error::Error> =
     //   serde_json::from_slice(body.to_string());
    }

    #[actix_rt::test]
    #[ignore]
    async fn test_create_list() {
     /*   let app_state: web::Data<Pool> = web::Data::new(get_pool());        

        let app = App::new()
        .data(app_state.clone())
        .route("/list", web::get().to(handler::get_lists_handler));

        let mut app = test::init_service(app).await;
        let req_data = Json(NewList {
            title: "test_title1".into(),
            category: Some("home".into())
        });
        let req = test::TestRequest::post()
        .uri("/list")
        .header("Content-Type","application/json")
        .set_payload(req_data.to_string())
        .to_request();

        let response = test::call_service(&mut app, req).await;

        assert_eq!(response.status(),200,"Status should be 200");

        let body = test::read_body(response).await;

        let created_list: Result<model::List,serde_json::error::Error> = 
        serde_json::from_slice(&body);
      //  println!("Created list is {}",body);
        assert!(created_list.is_ok(), "Response could not be parsed");

*/

        let app_state: web::Data<Pool> = web::Data::new(get_pool());        
        let req = test::TestRequest::with_header("content-type", "text/plain")
        .to_http_request();
        let req_data = Json(NewList {
            title: "test_title1".into(),
            category: Some("home".into())
        });
        let resp = create_list_handler(app_state,req_data).await.unwrap();
        println!("Response is {:?}", resp);
      //  println!("Response body is {:?}", resp.body());
        assert_eq!(resp.status(), StatusCode::OK);
        
    }

    #[actix_rt::test]
    async fn test_update_list() {

        let app_state: web::Data<Pool> = web::Data::new(get_pool());        
        let req = test::TestRequest::with_header("content-type", "text/plain")
        .to_http_request();
        let req_data = Json(UpdateList {
            title: Some("test_title2".into()),
            category: Some("office".into())
        });
        // Note the id of list has to be changed before test. Ensure id provided is there in database
        let id: web::Path::<(i32,)> = web::Path::from((22,));
        let resp = update_list_handler(app_state,id, req_data).await.unwrap();
        println!("Response is {:?}", resp);
      //  println!("Response body is {:?}", resp.body());
        assert_eq!(resp.status(), StatusCode::OK);

    }

    #[actix_rt::test]
    async fn test_getone_list() {

        let app_state: web::Data<Pool> = web::Data::new(get_pool());        
        let req = test::TestRequest::with_header("content-type", "text/plain")
        .to_http_request();

        // Note the id of list has to be changed before test. Ensure id provided is there in database
        let id: web::Path::<(i32,)> = web::Path::from((23,));
        let resp = get_one_list_handler(app_state,id).await.unwrap();
        println!("Response is {:?}", resp);
      //  println!("Response body is {:?}", resp.body());
        assert_eq!(resp.status(), StatusCode::OK);

    }

    #[actix_rt::test]

    async fn test_delete_list() {

        let app_state: web::Data<Pool> = web::Data::new(get_pool());        
        let req = test::TestRequest::with_header("content-type", "text/plain")
        .to_http_request();

        // Note the id of list has to be changed before test. Ensure id provided is there in database
        let id: web::Path::<(i32,)> = web::Path::from((21,));
        let resp = delete_list_handler(app_state,id).await.unwrap();
        println!("Response is {:?}", resp);
      //  println!("Response body is {:?}", resp.body());
        assert_eq!(resp.status(), StatusCode::OK);

    }


}

