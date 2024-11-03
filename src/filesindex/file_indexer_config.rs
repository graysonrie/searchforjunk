use std::path::{Path, PathBuf};

#[derive(Clone)]
pub struct FileIndexerConfig {
    pub tantivy_out_path: PathBuf,
    pub buffer_size: usize,
    pub indexer_batch_size: usize,
}
