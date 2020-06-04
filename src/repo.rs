// Standard lib
use std::io;
// External crates - Primary
// External crates - Utilities

use tokio_pg_mapper::FromTokioPostgresRow;
// Other internal modules
use crate::model::{List, UpdateList};
// Const and type declarations
// Struct declarations


// Functions
pub async fn get_lists_db(client: &deadpool_postgres::Client) -> io::Result<Vec<List>> {
    let stmt = client.prepare("select * from lists order by id desc").await.unwrap();

    let lists = client.query(&stmt, &[])
        .await
        .expect("Failed to query lists")
        .iter()
        .map(|row| List::from_row_ref(row).unwrap())
        .collect::<Vec<List>>();

        Ok(lists)
}

pub async fn create_list_db(client: &deadpool_postgres::Client, title: &str, category: &str) -> io::Result<List> {
    let stmt = client.prepare("insert into lists(title,category) values($1,$2) returning id, title, category").await.unwrap();
    
    client.query(&stmt,&[&title.to_string(),&category.to_string()])
                    .await
                    .expect("Error creating todo")
                    .iter()
                    .map(|row| List::from_row_ref(row).unwrap())
                    .collect::<Vec<List>>()
                    .pop()
                    .ok_or(io::Error::new(io::ErrorKind::Other, "Error creating List"))
                    
}

pub async fn delete_list_db(client: &deadpool_postgres::Client, id: i32) -> io::Result<()> {
    let stmt = client.prepare("delete from lists where id = $1")
                .await
                .unwrap();

    client.execute(&stmt, &[&id])
        .await
        .expect("unable to delete");   

    Ok(())
}


pub async fn update_list_db(client: &deadpool_postgres::Client, id: i32, list: UpdateList ) -> io::Result<List> {
    let stmt1 = client.prepare("select * from lists where id = $1 ")
        .await.expect("Unable to co");
    let row = client.query_one(&stmt1,&[&id])
    .await
    .expect("error");
 
    let mut existing_list: List = List::from_row_ref(&row).unwrap();
    if let Some(title) = list.title {
        existing_list.title = title
    }
    if let Some(category) = list.category {
        existing_list.category = Some(category)
    }

    let stmt2 = client.prepare("update lists set title = $1, category = $2 where id = $3 returning id, title, category")
    .await
    .unwrap();

    client.query(&stmt2,&[&existing_list.title,&existing_list.category, &id])
                    .await
                    .expect("Error updating todo")
                    .iter()
                    .map(|row| List::from_row_ref(row).unwrap())
                    .collect::<Vec<List>>()
                    .pop()
                    .ok_or(io::Error::new(io::ErrorKind::Other, "Error creating List"))
    

    
}