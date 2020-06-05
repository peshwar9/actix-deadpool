// Standard lib

// External crates - Primary
// External crates - Utilities

use tokio_pg_mapper::FromTokioPostgresRow;
// Other internal modules
use crate::errors::ApiError;
use crate::model::{List, NewList, UpdateList};
// Const and type declarations
// Struct declarations

// Functions
pub async fn get_lists_db(client: &deadpool_postgres::Client) -> Result<Vec<List>, ApiError> {
    let stmt = client
        .prepare("select * from lists order by id desc")
        .await?;

    let lists = client
        .query(&stmt, &[])
        .await?
        .iter()
        .map(|row| List::from_row_ref(row).unwrap())
        .collect::<Vec<List>>();

    Ok(lists)
}


pub async fn create_list_db(
    client: &deadpool_postgres::Client,
    newlist: NewList
) -> Result<List, ApiError> {
    let stmt = client
        .prepare("insert into lists(title,category) values($1,$2) returning id, title, category")
        .await?;


    client
        .query(&stmt, &[&newlist.title, &newlist.category])
        .await?
        .iter()
        .map(|row| List::from_row_ref(row).unwrap())
        .collect::<Vec<List>>()
        .pop()
        .ok_or(ApiError::DBError(
            "Error in returning inserted value".into(),
        ))
}


pub async fn delete_list_db(client: &deadpool_postgres::Client, id: i32) -> Result<(), ApiError> {
    let stmt1 = client.prepare("select * from lists where id = $1").await?;
    let res1 = client.query_one(&stmt1, &[&id]).await;
    //     .map_err(|_| ApiError::NotFound("Item for deletion not found".into()));

    if let Err(_) = res1 {
        return Err(ApiError::NotFound("Item for deletion not found".into()));
    }

    let stmt = client.prepare("delete from lists where id = $1").await?;

    client
        .execute(&stmt, &[&id])
        .await
        .map_err(|err| ApiError::DBError(err.to_string()))?;

    Ok(())
}

pub async fn get_one_list_db(client: &deadpool_postgres::Client, id: i32) -> Result<List, ApiError> {
    let stmt = client
                .prepare("select * from lists where id = $1")
                .await?;

    let res1 = client
                .query_one(&stmt,&[&id])
                .await;
    
    if let Err(_) = res1 {
        return Err(ApiError::NotFound("Item not found".into()));
    }
    Ok(List::from_row_ref(&res1.unwrap()).unwrap())
    
        
}

pub async fn update_list_db(
    client: &deadpool_postgres::Client,
    id: i32,
    list: UpdateList,
) -> Result<List, ApiError> {
    let stmt1 = client.prepare("select * from lists where id = $1 ").await?;

    let res1 = client.query_one(&stmt1, &[&id]).await;

    if let Err(_) = res1 {
        return Err(ApiError::NotFound("Item for modification not found".into()));
    }

    let mut existing_list: List = List::from_row_ref(&res1.unwrap()).unwrap();
    if let Some(title) = list.title {
        existing_list.title = title
    }
    if let Some(category) = list.category {
        existing_list.category = Some(category)
    }

    let stmt2 = client.prepare("update lists set title = $1, category = $2 where id = $3 returning id, title, category")
    .await?;

    client
        .query(
            &stmt2,
            &[&existing_list.title, &existing_list.category, &id],
        )
        .await?
        .iter()
        .map(|row| List::from_row_ref(row).unwrap())
        .collect::<Vec<List>>()
        .pop()
        .ok_or(ApiError::DBError("Error in returning updated value".into()))
}
