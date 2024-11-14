use std::{collections::HashMap, path::PathBuf, sync::Arc, time::Instant};

use chrono::Utc;
use tokio::sync::RwLock;

use crate::{
    shared::collections::popularity_set::PopularitySet,
    tantivy_file_indexer::services::app_save::service::AppSaveService,
};

type NumFiles = u32;
type RecentlyAddedDir = (PathBuf, NumFiles, chrono::DateTime<Utc>);

const DEFAULT_PRIORITY: Priority = 1;
const SAVE_NAME: &str = "files_queue";
pub struct CrawlerQueue {
    queue: Arc<RwLock<PopularitySet<PathBuf>>>,
    // Keep track of recently indexed directories so that if they get added again, they can be ignored
    recently_indexed: Arc<RwLock<Vec<RecentlyAddedDir>>>,
    save_service: Arc<AppSaveService>,
}

pub type Priority = u32;
impl CrawlerQueue {
    /**
     * An 'iteration' is one file being processed
     */
    pub async fn new_async(directories: Vec<PathBuf>, save_service: Arc<AppSaveService>) -> Self {
        let queue = Arc::new(RwLock::new(PopularitySet::<PathBuf>::new()));
        for item in directories {
            queue.write().await.insert(item, DEFAULT_PRIORITY);
        }
        Self {
            save_service,
            recently_indexed: Arc::new(RwLock::new(Vec::new())),
            queue,
        }
    }

    pub async fn push(&self, directory: PathBuf, priority: Priority) {
        self.queue.write().await.insert(directory, priority);
    }

    pub async fn push_default(&self, directory: PathBuf) {
        self.queue.write().await.insert(directory, DEFAULT_PRIORITY);
    }

    pub async fn pop(&self) -> Option<PathBuf> {
        self.queue.write().await.pop()
    }

    pub async fn get_len(&self) -> usize {
        self.queue.read().await.len()
    }

    pub async fn save(&self) -> Result<(), std::io::Error> {
        self.save_service.save(SAVE_NAME, self.queue_as_vec().await)
    }

    /**
     * Fallback in case the save path doesn't exist
     */
    pub async fn load_or(&self, fallback_directories: Vec<PathBuf>) {
        if self.load().await.is_err() {
            self.populate_queue(
                fallback_directories
                    .iter()
                    .map(|x| (x.clone(), DEFAULT_PRIORITY))
                    .collect(),
            )
            .await;
        }
    }

    pub async fn load(&self) -> Result<(), std::io::Error> {
        let entries = self
            .save_service
            .load::<Vec<(PathBuf, Priority)>>(SAVE_NAME)?;
        self.populate_queue(entries).await;
        Ok(())
    }

    async fn populate_queue(&self, entries: Vec<(PathBuf, Priority)>) {
        self.queue
            .write()
            .await
            .insert_many(entries.into_iter().map(|x| (x.0, x.1)).collect());
    }

    async fn queue_as_vec(&self) -> Vec<(PathBuf, Priority)> {
        self.queue.read().await.as_vec()
    }

    async fn add_to_recents(&self, dir:&PathBuf, num_files:NumFiles){
        let mut recents = self.recently_indexed.write().await;
        recents.push((dir.to_path_buf(),num_files, Utc::now()));
    }
    /* 
    async fn remove_from_recents(&self, dir:&PathBuf){
        let mut recents = self.recently_indexed.write().await;
        let index_of = recents.iter().position(|x| &x.0 == &dir);
        if let Some(index) = index_of{
            recents.remove(index);
        }
    }

    async fn processed_recently(&self, dir:&PathBuf):bool{
        let recent: Option<&RecentlyAddedDir> = self.recently_indexed.read().await.iter().find(|x| &x.0 == dir);
        match recent{
            Some(recent)=>{
                let now = Utc::now();

                true
            },
            None => false
        }
    }
    */
}
