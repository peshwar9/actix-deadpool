// Standard lib
use std::fmt;
// External crates - Primary
use actix_web::web;
// External crates - Utilities
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;
// Other internal modules
// Const and type declarations
// Struct declarations

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "lists")]
pub struct List {
    pub id: i32,
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

// Functions

// Convert incoming json data type for update to <UpdateList>

impl From<web::Json<UpdateList>> for UpdateList {
    fn from(json: web::Json<UpdateList>) -> Self {
        UpdateList {
            title: match &json.title {
                Some(title) => Some(title.to_string()),
                None => None,
            },
            category: match &json.category {
                Some(category) => Some(category.to_string()),
                None => None,
            },
        }
    }
}

// Convert incoming json data type for create to <NewList>

impl From<web::Json<NewList>> for NewList {
    fn from(json: web::Json<NewList>) -> Self {
        NewList {
            title: json.title.clone(),
            category: match &json.category {
                Some(category) => Some(category.to_string()),
                None => None,
            },
        }
    }
}

// Utility function to print out NewList. This is needed to do *new_list.to_string()* elsewhere in program
impl fmt::Display for NewList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        write!(f, "{}", self)
    }
}