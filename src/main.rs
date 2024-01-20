use actix_web::{web, App, HttpServer, HttpResponse, Result, ResponseError};
use std::fs;
use std::path::{Path, PathBuf};
use std::fmt;
use std::fmt::Formatter;
use std::io::Error;

#[derive(Debug)]
struct FileNotFoundError;

impl fmt::Display for FileNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "File not found")
    }
}

impl ResponseError for FileNotFoundError {}


fn get_project_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    Path::new(manifest_dir).join("src/pages")
}

fn path_control(filename: &str) -> Result<(), FileNotFoundError> {
    let path = get_project_root().join(filename);

    if !path.exists() {
        return Err(FileNotFoundError);
    }

    Ok(())
}

async fn serve_html(filename: &str) -> Result<HttpResponse, actix_web::Error> {
    path_control(filename)?;
    let path = get_project_root().join(filename);

    Ok(HttpResponse::Ok().content_type("text/html").body(fs::read_to_string(path)?))
}


#[actix_web::main]
async fn main() -> Result<(), Error> {
    HttpServer::new(|| {
        App::new()
            .route("/", web::get().to(|| serve_html("index.html")))
            .route("/page_two", web::get().to(|| serve_html("page_two.html")))
            .route("/page_three", web::get().to(|| serve_html("page_three.html")))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}