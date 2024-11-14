use crate::tantivy_file_indexer::services::app_save::service::AppSaveService;

use super::tables::files::api::FilesTable;
use sqlx::{sqlite::SqlitePool, Pool, Sqlite};
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct SqlxService {
    pool: Arc<Mutex<Pool<Sqlite>>>,
    files_table: FilesTable,
}

impl SqlxService {
    // consider using a config here
    pub async fn new_async(save_service: &AppSaveService) -> Self {
        let db_path = save_service.create_path("file_index.db");
        let db_url = format!("sqlite://{}", db_path.to_string_lossy());

        let pool = Arc::new(Mutex::new(SqlitePool::connect(&db_url).await.unwrap()));
        let files_table = FilesTable::new_async(pool.clone()).await;

        Self { pool, files_table }
    }

    pub fn files_table(&self)->&FilesTable{
        &self.files_table
    }
    /**
     * Runs a command on the database to reclaim unused memory
     */
    pub async fn vacuum(&self) -> Result<(), String> {
        let pool = self.pool.lock().await;
        match sqlx::query("VACUUM").execute(&*pool).await {
            Ok(_) => Ok(()),
            Err(err) => Err(err.to_string()),
        }
    }
}
