#[derive(serde::Deserialize)]
pub struct TextForm {
    file_path: String,
    handling_mode: String,
    contents: String,
}

impl TextForm {
    pub fn get_file_path(&self) -> &str {
        &self.file_path
    }
    pub fn get_handling_mode(&self) -> &str {
        &self.handling_mode
    }
    pub fn get_contents(&self) -> &str {
        &self.contents
    }
}

