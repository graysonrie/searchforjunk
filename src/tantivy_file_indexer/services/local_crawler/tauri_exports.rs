/* 
use std::sync::Arc;
use tauri::State;

use super::{core::crawler_queue::Priority, service::FileCrawlerService};

#[tauri::command]
pub async fn add_dirs_to_crawler_queue(
    directories: Vec<(&str, Priority)>,
    service: State<'_, Arc<FileCrawlerService>>,
) -> Result<(), ()> {
    service.push_dirs(directories).await;
    Ok(())
}
*/