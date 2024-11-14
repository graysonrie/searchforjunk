use std::path::PathBuf;

#[derive(Clone)]
pub struct FileIndexerConfig {
    pub buffer_size: usize,
    pub indexer_batch_size: usize,
    pub app_path: PathBuf,
}
