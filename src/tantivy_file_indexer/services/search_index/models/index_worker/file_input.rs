use std::path::PathBuf;

use crate::tantivy_file_indexer::dtos::file_dto_input::FileDTOInput;

pub struct FileInputModel {
    pub dtos: Vec<FileDTOInput>,
    pub directory_from: PathBuf,
}
