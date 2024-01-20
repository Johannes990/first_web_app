mod paths;

use actix_web::{web, App, HttpServer, HttpResponse, Result, ResponseError};
use std::fs;
use std::io::Error;
use crate::paths::{FilePath, path_control};
use env_logger;


async fn serve_html(filename: FilePath) -> Result<HttpResponse, actix_web::Error> {
    let path = path_control(filename)?;

    Ok(HttpResponse::Ok().content_type("text/html").body(fs::read_to_string(&path)?))
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    env_logger::init();
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| serve_html(FilePath::Index)))
            .route("/page_two", web::get().to(|| serve_html(FilePath::SecondPage)))
            .route("/page_three", web::get().to(|| serve_html(FilePath::ThirdPage)))
    })
        .bind("127.0.0.1:8085")?
        .run()
        .await
}
