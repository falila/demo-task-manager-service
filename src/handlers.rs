use crate::db;
use crate::erros::AppError;
use crate::models::{AppState, CreateTaskList, TaskItem, TaskList};
use crate::models::{ResultResponse, Status};
use actix_web::{web};
use actix_web::{HttpResponse, Responder};
use deadpool_postgres::{Client, Pool};
use slog::{crit, o, Logger, error};

pub async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status {
        status: "OK".to_string(),
    })
}

pub async fn get_client(pool: Pool, log: Logger) -> Result<Client, AppError> {
    pool.get().await.map_err(|error| {
        let temp_log = log.new(o!("caused"=> error.to_string()));
        crit!(temp_log, "Error while creating db pool client");
        AppError::db_error(error)
    })
}
pub fn log_error(log: Logger)-> Box<dyn Fn(AppError) -> AppError> { 
    Box::new(move |err| {
        let s_log = log.new(o!("caused"=> err.cause.clone()));
        error!(s_log, "{}", err.message());
        err
    })

}
pub async fn get_tasks(state: web::Data<AppState>) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler"=> "get_tasks"));
    let client: Client = get_client(state.pool.clone(), log.clone()).await?;
    let result: Result<Vec<TaskList>, AppError> = db::get_tasks(&client).await;

    result.map(|tasks| HttpResponse::Ok().json(tasks)).map_err(log_error(log))
}

pub async fn get_items(
    state: web::Data<AppState>,
    path: web::Path<(i32,)>,
) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler"=> "get_items"));

    let client: Client = get_client(state.pool.clone(), log.clone()).await?;
    let result: Result<Vec<TaskItem>, AppError> = db::get_task_items(&client, path.0).await;
    result.map(|items| HttpResponse::Ok().json(items)).map_err(log_error(log))
}

pub async fn create_task(
    state: web::Data<AppState>,
    json: web::Json<CreateTaskList>,
) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler"=> "create_task"));
    let client: Client = get_client(state.pool.clone(), log.clone()).await?;
    let result: Result<TaskList, AppError> = db::create_task(&client, json.title.clone()).await;

    result.map(|tasks| HttpResponse::Ok().json(tasks)).map_err(log_error(log))
}

pub async fn mark_item(
    state: web::Data<AppState>,
    path: web::Path<(i32, i32)>,
) -> Result<impl Responder, AppError> {
    let log = state.log.new(o!("handler"=> "mark_item"));
    let client: Client = get_client(state.pool.clone(), log.clone()).await?;
    let result = db::mark_item(&client, path.0, path.1).await;

    result.map(|updated| HttpResponse::Ok().json(ResultResponse { success: updated })).map_err(log_error(log))
}
