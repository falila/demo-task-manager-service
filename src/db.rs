use crate::models::{TaskList};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;
use std::io;

pub async fn get_tasks(client: &Client) -> Result<Vec<TaskList>, io::Error> {
    let query_statement =  client.prepare("SELECT * FROM task_list").await.unwrap();
    let tasks = client.query(&query_statement, &[])
        .await
        .expect("Error executing query statement")
        .iter()
        .map(|row| TaskList::from_row_ref(row).unwrap())
        .collect::<Vec<TaskList>>();
    Ok(tasks)
}