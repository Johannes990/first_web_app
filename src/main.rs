mod paths;
mod fileparser;

use actix_web::{web, App, HttpServer, HttpResponse, Result};
use std::fs;
use std::io::Error;
use crate::paths::{FilePath, path_control};
use log::{info, error};
use crate::fileparser::readFile;

#[derive(serde::Deserialize)]
struct TextForm {
    user_input: String,
}


async fn serve_html(filename: FilePath) -> Result<HttpResponse, actix_web::Error> {
    let path = path_control(filename)?;

    info!("Serving file from path: {:?}", path);

    Ok(HttpResponse::Ok().content_type("text/html").body(fs::read_to_string(&path)?))
}

async fn serve_text_input_page() -> HttpResponse {
    let path = FilePath::ThirdPage.get_full_path();

    info!("Posting to path: {:?}", path);

    match fs::read_to_string(&path) {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(err) => {
            error!("Failed to read file: {:?}", err);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

async fn process_text(form: web::Form<TextForm>) -> HttpResponse {
    let user_input = form.user_input.clone();
    let file_contents = fileparser::readFile(&user_input);

    info!("User entered text: {:?}", user_input);
    HttpResponse::Ok().body(format!("Processed text: \n{}", file_contents))
}

#[actix_web::main]
async fn main() -> Result<(), Error> {
    env_logger::Builder::from_default_env().filter_level(log::LevelFilter::Info).init();
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| serve_html(FilePath::Index)))
            .route("/page_two", web::get().to(|| serve_html(FilePath::SecondPage)))
            .route("/page_three", web::get().to(serve_text_input_page))
            .route("/process_text", web::post().to(process_text))
    })
        .bind("127.0.0.1:8085")?
        .run()
        .await
}
