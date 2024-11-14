/*
use std::sync::Arc;
 
use tauri::State;

use crate::tantivy_file_indexer::services::local_db::service::SqlxService;

#[tauri::command]
pub async fn get_num_stored_files(service: State<'_, Arc<SqlxService>>) -> Result<i64, String> {
    match service.files_table().count_files().await {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    }
}
*/