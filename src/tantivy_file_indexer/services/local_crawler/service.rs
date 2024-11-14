use tokio::sync::mpsc;

use crate::tantivy_file_indexer::services::app_save::service::AppSaveService;
use crate::tantivy_file_indexer::services::local_db::service::SqlxService;
use crate::tantivy_file_indexer::services::search_index::models::index_worker::file_input::FileInputModel;
use crate::tantivy_file_indexer::services::search_index::service::SearchIndexService;
use std::path::{Path, PathBuf};
use std::sync::Arc;

use super::core::crawler_queue::{CrawlerQueue, Priority};

pub struct FileCrawlerService {
    max_concurrent_tasks: usize,
    crawler_save_after_iters: usize,
    queue: Arc<CrawlerQueue>,
    search_service: Arc<SearchIndexService>,
    db_service: Arc<SqlxService>,
}

impl FileCrawlerService {
    pub async fn new_async(
        max_concurrent_tasks: usize,
        crawler_save_after_iters: usize,
        search_service: Arc<SearchIndexService>,
        db_service: Arc<SqlxService>,
        app_save_service: Arc<AppSaveService>,
    ) -> Self {
        let queue = Arc::new(CrawlerQueue::new_async(vec![], app_save_service.clone()).await);
        Self {
            max_concurrent_tasks,
            crawler_save_after_iters,
            search_service,
            db_service,
            queue,
        }
    }

    pub fn spawn_crawler(&self, sender: mpsc::Sender<FileInputModel>) {
        let queue = self.queue.clone();
        let max_concurrent_tasks = self.max_concurrent_tasks;
        let crawler_save_after_iters = self.crawler_save_after_iters;

        tokio::task::spawn(async move {
            super::core::crawler_worker::spawn_worker(
                sender,
                max_concurrent_tasks,
                crawler_save_after_iters,
                queue,
            )
            .await;
        });
    }

    pub async fn push_dirs(&self, paths: Vec<(&str, Priority)>) {
        let dirs = paths
            .iter()
            .map(|x| (Path::new(x.0).to_path_buf(), x.1))
            .collect();
        self.process_dirs(dirs).await;
    }

    pub async fn load_or(&self, fallback_directories: Vec<&str>) {
        let dirs: Vec<PathBuf> = fallback_directories
            .iter()
            .map(|x| Path::new(x).to_path_buf())
            .collect();
        self.queue.load_or(dirs).await;
    }

    async fn process_dirs(&self, paths: Vec<(PathBuf, Priority)>) {
        for path in paths {
            self.queue.push(path.0, path.1).await;
        }
    }
}
