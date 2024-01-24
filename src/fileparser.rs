use std::fs::File;
use std::io::prelude::*;
use std::path::Path;

pub fn readFile(filename: &str) -> String {
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(error) => panic!("Couldn't open {}: {}", display, error),
        Ok(file) => file,
    };

    let mut s = String::new();
    match file.read_to_string(&mut s) {
        Err(error) => panic!("Couldn't read {}: {}", display, error),
        Ok(_) => { return s; }
    }
}
