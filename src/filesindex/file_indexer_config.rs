use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct FileIndexerConfig{
    pub tantivy_out_path:PathBuf,
    pub buffer_size:usize
}

impl FileIndexerConfig{
    pub fn new(tantivy_out_path:&str, buffer_size:usize)->Self{
        Self { tantivy_out_path: Path::new(tantivy_out_path).to_path_buf(), buffer_size }
    }
}
