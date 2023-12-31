use deadpool_postgres::Pool;
use slog::Logger;
use serde::{Deserialize, Serialize};
use tokio_pg_mapper_derive::PostgresMapper;

#[derive(Serialize)]
pub struct Status {
    pub status: String,
}

#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "task_list")]
pub struct TaskList {
    pub id: i32,
    pub title: String,
}
#[derive(Serialize, Deserialize, PostgresMapper)]
#[pg_mapper(table = "task_item")]
pub struct TaskItem {
    pub id: i32,
    pub title: String,
    pub checked: bool,
    pub list_id: i32,
}

#[derive(Serialize, Deserialize)]
pub struct CreateTaskList {
    pub title: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResultResponse {
    pub success: bool,
}

pub struct AppState {
    pub pool: Pool,
    pub log: Logger
}