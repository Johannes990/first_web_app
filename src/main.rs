use actix_web::{web, App, HttpServer, HttpResponse};
use std::io::{Result};


async fn index() -> HttpResponse {
    HttpResponse::Ok().body("Hello, rust web services!!!")
}

async fn page_two() -> HttpResponse {
    HttpResponse::Ok().body("site numero dos")
}

#[actix_web::main]
async fn main() -> Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(index))
            .route("/page_two", web::get().to(page_two))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}