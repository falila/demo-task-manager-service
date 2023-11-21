mod config;
mod models;
mod handlers;
mod db;
use crate::config::Config;
use crate::handlers::*;
use actix_web::{web, App, HttpServer};
use deadpool_postgres::Runtime;
use dotenv::dotenv;
use std::io;
use tokio_postgres::NoTls;
use actix_web::web::Data;



#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();
    let config = Config::from_env().unwrap();
    let pool = config.pg.create_pool(Some(Runtime::Tokio1), NoTls).unwrap();
    
    //let host = std::env::var("SERVER.HOST").expect("SERVER.HOST must be set.");
    //let port = std::env::var("SERVER.PORT").expect("SERVER.PORT must be set.");
    print!(
        "Server running at http://{}:{}/",
        config.server.host, config.server.port
    );
    HttpServer::new(move ||{App::new()
        .app_data(Data::new(pool.clone()))
        .route("/", web::get().to(status))
        .route("/tasks", web::get().to(get_tasks))
        })
        .bind(format!("{}:{}", config.server.host, config.server.port))?
        .run()
        .await
}
