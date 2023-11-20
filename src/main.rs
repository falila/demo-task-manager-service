use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use std::io;
mod models;
use crate::models::Status;

async fn status() -> impl Responder {
    HttpResponse::Ok().json(Status { status: "OK".to_string() })
}
#[actix_rt::main]
async fn main() -> io::Result<()> {
    print!("Server running at http://127.0.0.1:8080");
    HttpServer::new(|| App::new().route("/", web::get().to(status)))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
