
use crate::models::Status;
use crate::models::{TaskList};
use std::io;
use deadpool_postgres::{Pool, Client};
use actix_web::{Responder,HttpResponse};
use actix_web::web;
use crate::db;


pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "OK".to_string(),
    })
}

pub async fn get_tasks(db_pool: web::Data<Pool>) -> HttpResponse {

    let client: Client = db_pool.get().await.expect("Error database pool connection");
    let result:Result<Vec<TaskList>, io::Error>  = db::get_tasks(&client).await;

    match result {
        Ok(tasks) => HttpResponse::Ok().json(tasks),
        Err(_) => HttpResponse::InternalServerError().into()       
    }
}