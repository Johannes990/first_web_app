use std::fs::OpenOptions;
use std::io::prelude::*;
use std::path::Path;

pub fn read_file(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match OpenOptions::new().read(true).open(path) {
        Err(error) => panic!("Couldn't open {}: {}", display, error),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(error) => panic!("Couldn't read {}: {}", display, error),
        Ok(_) => { return s; }
    }
}

pub fn write_file(filename: &str, contents: &str) {
    let path = Path::new(&filename);
    let display = path.display();

    let mut file = match OpenOptions::new().write(true).create(true).open(path) {
        Err(error) => panic!("Couldn't create or overwrite {}: {}", display, error),
        Ok(file) => file,
    };

    match file.write(contents.as_bytes()) {
        Err(error) => panic!("Couldn't write {}: {}", display, error),
        Ok(_) => {}
    }
}

pub fn append_file(filename: &str, contents: &str) {
    let path = Path::new(&filename);
    let display = path.display();

    let mut file = match OpenOptions::new().append(true).open(path) {
        Err(error) => panic!("Couldn't open {}: {}", display, error),
        Ok(file) => file,
    };

    match file.write(contents.as_bytes()) {
        Err(error) => panic!("Couldn't write {}: {}", display, error),
        Ok(_) => {}
    }
}
