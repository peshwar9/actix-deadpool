use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
use actix_web::web;


#[derive(Serialize, Deserialize,PostgresMapper)]
#[pg_mapper(table = "lists")]
pub struct List {
    pub id:i32,
    pub title: String,
    pub category: Option<String>,
}

#[derive(Serialize)]
pub struct ListId {
    pub id: i32,
}

#[derive(Deserialize)]
pub struct NewList {
    pub title: String,
    pub category: Option<String>,
}

#[derive(Deserialize)]
pub struct UpdateList {
    pub title: Option<String>,
    pub category: Option<String>,
}

impl From<web::Json<UpdateList>> for UpdateList {
    fn from(json: web::Json<UpdateList>) -> Self {
        UpdateList {
            title: match &json.title {
                Some(title) => Some(title.to_string()),
                None => None
            },
            category: match &json.category {
                Some(category) => Some(category.to_string()),
                None => None
            }
        }
    }
}