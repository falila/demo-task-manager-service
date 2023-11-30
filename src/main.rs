mod config;
mod db;
mod erros;
mod handlers;
mod models;
use crate::handlers::*;
use crate::{config::Config, models::AppState};
use actix_web::web::Data;
use actix_web::{web, App, HttpServer};
use deadpool_postgres::Runtime;
use dotenv::dotenv;
use slog::{info, o, Drain, Logger};
use std::io;
use tokio_postgres::NoTls;

fn config_log() -> Logger {
    let decorator = slog_term::TermDecorator::new().build();
    let c_drain = slog_term::FullFormat::new(decorator).build().fuse();
    let asyn_c_drain = slog_async::Async::new(c_drain).build().fuse();
    slog::Logger::root(asyn_c_drain, o!("v" => env!("CARGO_PKG_VERSION")))
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    let log = config_log();
    info!(
        log,
        "Server running at http://{}:{}/", config.server.host, config.server.port
    );
    HttpServer::new(move || {
        App::new()
            .app_data(Data::new(AppState {
                pool: pool.clone(),
                log: log.clone(),
            }))
            .route("/", web::get().to(status))
            .route("/tasks", web::get().to(get_tasks))
            .route("/tasks", web::post().to(create_task))
            .route("/tasks/{list_id}/items", web::get().to(get_items))
            .route("/tasks/{list_id}/items/{item_id}", web::put().to(mark_item))
    })
    .bind(format!("{}:{}", config.server.host, config.server.port))?
    .run()
    .await
}
