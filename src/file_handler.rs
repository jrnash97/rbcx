use crate::config::Config;
use std::fs::ReadDir;
pub struct FileHandler {
    parent_dir: String,
    file_names: Vec<String>,
}

impl FileHandler {
    fn new(dir: ReadDir, config: &Config) -> FileHandler {
        todo!()
    }
}
