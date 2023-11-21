use crate::db;
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

pub async fn get_tasks(db_pool: web::Data<Pool>) -> HttpResponse {
    let client: Client = db_pool.get().await.expect("Error database pool connection");
    let result: Result<Vec<TaskList>, io::Error> = db::get_tasks(&client).await;

    match result {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn get_items(db_pool: web::Data<Pool>, path: web::Path<(i32,)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error database pool connection");
    let result: Result<Vec<TaskItem>, io::Error> = db::get_task_items(&client, path.0).await;

    match result {
        Ok(items) => HttpResponse::Ok().json(items),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn create_task(
    db_pool: web::Data<Pool>,
    json: web::Json<CreateTaskList>,
) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error database pool connection");
    let result: Result<TaskList, io::Error> = db::create_task(&client, json.title.clone()).await;

    match result {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}

pub async fn mark_item(db_pool: web::Data<Pool>, path: web::Path<(i32, i32)>) -> impl Responder {
    let client: Client = db_pool.get().await.expect("Error database pool connection");
    let result = db::mark_item(&client, path.0, path.1).await;

    match result {
        Ok(()) => HttpResponse::Ok().json(ResultResponse { success: true }),
        Err(ref e) if e.kind() == Other => HttpResponse::Ok().json(ResultResponse {success: false}),
        Err(_) => HttpResponse::InternalServerError().into(),
    }
}
