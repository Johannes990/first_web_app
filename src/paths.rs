use std::fmt;
use std::fmt::Formatter;
use std::path::{Path, PathBuf};
use actix_web::ResponseError;
use log::info;


#[derive(Debug)]
pub struct FileNotFoundError;

impl fmt::Display for FileNotFoundError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "File not found")
    }
}

impl ResponseError for FileNotFoundError {}

pub enum FilePath {
    Index,
    SecondPage,
    ThirdPage,
}

impl FilePath {
    pub fn get_full_path(&self) -> PathBuf {
        let base_path = get_project_root().join("src/pages");

        info!("Base path for {:?} received.", base_path);

        match self {
            FilePath::Index => base_path.join("index.html"),
            FilePath::SecondPage => base_path.join("page_two.html"),
            FilePath::ThirdPage => base_path.join("page_three.html"),
        }
    }
}

fn get_project_root() -> PathBuf {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    Path::new(manifest_dir).into()
}

pub fn path_control(file_path: FilePath) -> Result<PathBuf, FileNotFoundError> {
    let path = file_path.get_full_path();

    if !path.exists() {
        println!("File not found: {:?}", path);
        return Err(FileNotFoundError);
    }

    Ok(path)
}
