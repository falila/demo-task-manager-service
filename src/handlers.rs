use crate::db;
use crate::erros::{AppError, AppErrorType};
use crate::models::{CreateTaskList, TaskItem, TaskList};
use crate::models::{ResultResponse, Status};
use actix_web::web;
use actix_web::{HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use std::io;
use std::io::ErrorKind::Other;

pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "OK".to_string(),
    })
}

pub async fn get_tasks(db_pool: web::Data<Pool>) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.map_err(AppError::db_error)?;
    let result: Result<Vec<TaskList>, AppError> = db::get_tasks(&client).await;

    result.map(|tasks| HttpResponse::Ok().json(tasks))
}

pub async fn get_items(
    db_pool: web::Data<Pool>,
    path: web::Path<(i32,)>,
) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.map_err(AppError::db_error)?;
    let result: Result<Vec<TaskItem>, AppError> = db::get_task_items(&client, path.0).await;
    result.map(|items| HttpResponse::Ok().json(items))
}

pub async fn create_task(
    db_pool: web::Data<Pool>,
    json: web::Json<CreateTaskList>,
) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.map_err(AppError::db_error)?;
    let result: Result<TaskList, AppError> = db::create_task(&client, json.title.clone()).await;

    result.map(|tasks| HttpResponse::Ok().json(tasks))
}

pub async fn mark_item(
    db_pool: web::Data<Pool>,
    path: web::Path<(i32, i32)>,
) -> Result<impl Responder, AppError> {
    let client: Client = db_pool.get().await.map_err(AppError::db_error)?;
    let result = db::mark_item(&client, path.0, path.1).await;

    result.map(|updated| HttpResponse::Ok().json(ResultResponse { success: updated }))
}
