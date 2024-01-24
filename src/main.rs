mod paths;
mod file_parser;
mod data_structs;


use actix_web::{web, App, HttpServer, HttpResponse, Result};
use actix_multipart::Multipart;
use futures_util::stream::StreamExt;
use std::fs;
use std::io;
use crate::paths::{FilePath, path_control};
use log::{info, error};
use crate::file_parser::{read_file, write_file, append_file};
use crate::data_structs::TextForm;


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

async fn serve_file_upload_page() -> HttpResponse {
    let path = FilePath::Upload.get_full_path();

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
    let file_path = form.get_file_path();
    let handling_mode = form.get_handling_mode();
    let contents = form.get_contents();

    info!("User entered text: {:?}/n{:?}/n{:?}", file_path, handling_mode, contents);

    let response = match handling_mode {
        "r" => HttpResponse::Ok().body(format!("Read contents:\n{}", read_file(file_path))),
        "w" | "a" => {
            let operation = match handling_mode {
                "w" => "write",
                "a" => "append",
                _ => "unknown"
            };
            if handling_mode == "w" {
                write_file(file_path, contents)
            } else {
                append_file(file_path, contents)
            }
            HttpResponse::Ok().body(format!("File operation '{}' successful for file: {}", operation, file_path))
        }
        _ => HttpResponse::Ok().body(format!("No such file handling mode available\nPlease use 'r' to read, 'w' to write or 'a' to append"))
    };
    response
}

async fn upload_file(mut payload: Multipart) -> Result<HttpResponse, actix_web::Error> {
    let mut buffer = Vec::new();
    while let Some(item) = payload.next().await {
        let mut field = item?;

        while let Some(chunk) = field.next().await {
            buffer.extend_from_slice(&chunk?)
        }
    }
    Ok(HttpResponse::Ok().body(format!("read chunk:\n{:?}", std::str::from_utf8(&*buffer))))
}

#[actix_web::main]
async fn main() -> Result<(), io::Error> {
    env_logger::Builder::from_default_env().filter_level(log::LevelFilter::Info).init();
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| serve_html(FilePath::Index)))
            .route("/page_two", web::get().to(|| serve_html(FilePath::SecondPage)))
            .route("/page_three", web::get().to(serve_text_input_page))
            .route("/process_text", web::post().to(process_text))
            .route("/upload_file_page", web::get().to(serve_file_upload_page))
            .route("/upload_file", web::post().to(upload_file))
    })
        .bind("127.0.0.1:8085")?
        .run()
        .await
}
