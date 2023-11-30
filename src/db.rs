use crate::erros::{AppError, AppErrorType};
use crate::models::{TaskItem, TaskList};
use deadpool_postgres::Client;
use tokio_pg_mapper::FromTokioPostgresRow;

pub async fn get_tasks(client: &Client) -> Result<Vec<TaskList>, AppError> {
    let query_statement = client
        .prepare("SELECT * FROM task_list order by id desc limit 20")
        .await
        .map_err(AppError::db_error)?;
    let tasks = client
        .query(&query_statement, &[])
        .await
        .expect("Error executing query statement")
        .iter()
        .map(|row| TaskList::from_row_ref(row).unwrap())
        .collect::<Vec<TaskList>>();
    Ok(tasks)
}

pub async fn get_task_items(client: &Client, list_id: i32) -> Result<Vec<TaskItem>, AppError> {
    let query_statement = client
        .prepare("SELECT * FROM task_item where list_id = $1 order by id desc")
        .await
        .map_err(AppError::db_error)?;
    let items = client
        .query(&query_statement, &[&list_id])
        .await
        .expect("Error executing query statement")
        .iter()
        .map(|row| TaskItem::from_row_ref(row).unwrap())
        .collect::<Vec<TaskItem>>();
    Ok(items)
}

pub async fn create_task(client: &Client, title: String) -> Result<TaskList, AppError> {
    let statement = client
        .prepare("insert into task_list (title) values ($1) returning id, title")
        .await
        .map_err(AppError::db_error)?;
    client
        .query(&statement, &[&title])
        .await
        .expect("Error while creating task")
        .iter()
        .map(|row| TaskList::from_row_ref(row).unwrap())
        .collect::<Vec<TaskList>>()
        .pop()
        .ok_or(AppError {
            error_type: AppErrorType::DbError,
            message: Some("An error occured while attemping to create Task list".to_string()),
            cause: Some("unknown error".to_string()),
        })
}

pub async fn mark_item(client: &Client, list_id: i32, item_id: i32) -> Result<bool, AppError> {
    let query_statement = client
        .prepare("update task_item set checked = true where list_id = $1 and id = $2 and checked = false")
        .await
        .map_err(AppError::db_error)?;
    let result = client
        .execute(&query_statement, &[&list_id, &item_id])
        .await
        .expect("Error executing query statement");
    match result {
        ref update_value if *update_value == 1 => Ok(true),
        _ => Ok(false),
    }
}
