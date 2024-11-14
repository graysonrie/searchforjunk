use crossbeam::queue::SegQueue;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::Instant;
use std::{path::PathBuf, sync::Arc, time::UNIX_EPOCH};
use tokio::time::{self, Duration};
use tokio::{
    sync::{mpsc, Semaphore},
    task::JoinSet,
};

use crate::tantivy_file_indexer::{
    dtos::file_dto_input::FileDTOInput,
    services::search_index::models::index_worker::file_input::FileInputModel, util::file_id_helper,
};

use super::crawler_queue::CrawlerQueue;

// Note that the crawler does not handle database operations
pub async fn spawn_worker(
    sender: mpsc::Sender<FileInputModel>,
    max_concurrent_tasks: usize,
    save_queue_after: usize,
    queue: Arc<CrawlerQueue>,
) {
    let dir_entries = Arc::new(SegQueue::new());
    let semaphore = Arc::new(Semaphore::new(max_concurrent_tasks));
    let mut tasks = JoinSet::new();
    let files_processed = Arc::new(AtomicUsize::new(0));
    let worker_queue = Arc::clone(&queue);

    loop {
        if let Some(path) = queue.pop().await {
            let dir_entries = Arc::clone(&dir_entries);
            let semaphore = Arc::clone(&semaphore);
            let sender = sender.clone();
            let queue = Arc::clone(&worker_queue);
            let files_processed = Arc::clone(&files_processed);

            tasks.spawn(async move {
                let _permit = semaphore
                    .acquire_owned()
                    .await
                    .expect("Failed to acquire semaphore");

                if path.is_dir() {
                    if let Ok(mut dir) = tokio::fs::read_dir(&path).await {
                        while let Ok(Some(entry)) = dir.next_entry().await {
                            let entry_path = entry.path();
                            if entry_path.is_dir() {
                                //let time = Instant::now();

                                queue.push_default(entry_path).await;

                                //println!("dir queue push took {:?}", time.elapsed());
                            }
                            if let Ok(dto) = create_dto(&entry).await {
                                //let time = Instant::now();

                                dir_entries.push(dto);
                                let count = files_processed.fetch_add(1, Ordering::Relaxed) + 1;
                                if count >= save_queue_after {
                                    files_processed.store(0, Ordering::Relaxed);
                                    if let Err(err) = queue.save().await {
                                        eprintln!("Failed to save queue: {}", err);
                                    }
                                }

                                //println!("create dto took {:?}", time.elapsed());
                            }
                        }
                    }
                }

                let model = create_model(path, &dir_entries).await;
                //let time = Instant::now();
                // Apparent bottleneck:
                if let Err(err) = sender.send(model).await {
                    eprintln!("Error sending FileInputModel to indexer: {}", err);
                }
                //println!("files send {:?}", time.elapsed());
            });
        } else {
            if tasks.is_empty() {
                println!("Worker done processing");
                break;
            }
            time::sleep(Duration::from_millis(100)).await;
        }

        while let Some(result) = tasks.join_next().await {
            if let Err(err) = result {
                eprintln!("Task error: {:?}", err);
            }
        }
    }
}

async fn create_model(directory_from: PathBuf, entries: &SegQueue<FileDTOInput>) -> FileInputModel {
    let mut dtos = Vec::<FileDTOInput>::new();
    while let Some(entry) = entries.pop() {
        dtos.push(entry);
    }
    FileInputModel {
        dtos,
        directory_from,
    }
}

// This function takes around 60ms to complete so look at this
async fn create_dto(entry: &tokio::fs::DirEntry) -> Result<FileDTOInput, String> {
    let metadata = entry.metadata().await.map_err(|x| x.to_string())?;

    let modified_time = metadata.modified().map_err(|x| x.to_string())?;

    let unix_timestamp = modified_time
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();

    let file_id = if entry.path().is_dir() {
        //for directories, use the directory path since getting their ID is more difficult
        entry.path().to_string_lossy().to_string()
    } else {
        file_id_helper::get_file_id(entry.path().to_path_buf())?
    };

    let dto = FileDTOInput {
        file_id,
        name: entry.file_name().to_string_lossy().to_string(),
        file_path: entry.path().to_string_lossy().to_string(),
        metadata: "test metadata".to_string(),
        date_modified: unix_timestamp,
        popularity: 1.0,
    };
    Ok(dto)
}
