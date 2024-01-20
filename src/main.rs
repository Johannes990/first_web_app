use actix_web::{web, App, HttpServer, HttpResponse};
use std::io::{Result};
use std::fs;


async fn index() -> Result<HttpResponse> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "src/pages/index.html");
    Ok(HttpResponse::Ok().content_type("text/html").body(fs::read_to_string(path)?))
}

async fn page_two() -> Result<HttpResponse> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "src/pages/page_two.html");
    Ok(HttpResponse::Ok().content_type("text/html").body(fs::read_to_string(path)?))
}

async fn page_three() -> Result<HttpResponse> {
    let path = concat!(env!("CARGO_MANIFEST_DIR"), "src/pages/page_three.html");
    Ok(HttpResponse::Ok().content_type("text/html").body(fs::read_to_string(path)?))
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/page_two", web::get().to(page_two))
            .route("/page_three", web::get().to(page_three))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}