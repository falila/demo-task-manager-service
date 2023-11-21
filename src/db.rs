use crate::models::{TaskItem, TaskList};
use actix_web::dev::always_ready;
use deadpool_postgres::Client;
use std::io;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_tasks(client: &Client) -> Result<Vec<TaskList>, io::Error> {
    let query_statement = client
        .prepare("SELECT * FROM task_list order by id desc")
        .await
        .unwrap();
    let tasks = client
        .query(&query_statement, &[])
        .await
        .expect("Error executing query statement")
        .iter()
        .map(|row| TaskList::from_row_ref(row).unwrap())
        .collect::<Vec<TaskList>>();
    Ok(tasks)
}

pub async fn get_task_items(client: &Client, list_id: i32) -> Result<Vec<TaskItem>, io::Error> {
    let query_statement = client
        .prepare("SELECT * FROM task_item where list_id = $1 order by id desc")
        .await
        .unwrap();
    let items = client
        .query(&query_statement, &[&list_id])
        .await
        .expect("Error executing query statement")
        .iter()
        .map(|row| TaskItem::from_row_ref(row).unwrap())
        .collect::<Vec<TaskItem>>();
    Ok(items)
}

pub async fn create_task(client: &Client, title: String) -> Result<TaskList, io::Error> {
    let statement = client
        .prepare("insert into task_list (title) values ($1) returning id, title")
        .await
        .unwrap();
    client
        .query(&statement, &[&title])
        .await
        .expect("Error while creating task")
        .iter()
        .map(|row| TaskList::from_row_ref(row).unwrap())
        .collect::<Vec<TaskList>>()
        .pop()
        .ok_or(io::Error::new(
            io::ErrorKind::Other,
            "Error creating taskList",
        ))
}

pub async fn mark_item(client: &Client, list_id: i32, item_id: i32) -> Result<(), io::Error> {
    let query_statement = client
        .prepare("update task_item set checked = true where list_id = $1 and id = $2 and checked = false")
        .await
        .unwrap();
    let result = client
        .execute(&query_statement, &[&list_id, &item_id])
        .await
        .expect("Error executing query statement");
    match result {
        ref update_value if *update_value == 1 => Ok(()),
        _ => Err(io::Error::new(io::ErrorKind::Other, "Error updating")),
    }
}
