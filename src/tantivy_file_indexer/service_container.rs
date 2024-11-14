use super::services::{
    app_save::service::{AppSavePath, AppSaveService},
    local_crawler::service::FileCrawlerService,
    local_db::service::SqlxService,
    search_index::service::SearchIndexService,
};
use std::sync::Arc;

use super::configs::file_indexer_config::FileIndexerConfig;

pub struct AppServiceContainer {
    pub search_service: Arc<SearchIndexService>,
    pub sqlx_service: Arc<SqlxService>,
    pub crawler_service: Arc<FileCrawlerService>,
}

impl AppServiceContainer {
    pub async fn new_async() -> Self {
        let app_name = "DesktopSearch";

        // AppSavePath::Other("D:\\DSearch".to_string())
        let app_save_service = Self::initialize_app_save_service(AppSavePath::AppData, app_name);

        let config = Self::create_file_indexer_config(&app_save_service);
        let search_service = Self::initialize_search_service(&config);

        let sqlx_service = Self::initialize_sqlx_service(&app_save_service).await;
        let crawler_service = Self::initialize_crawler_service(
            8,
            512,
            search_service.clone(),
            sqlx_service.clone(),
            app_save_service.clone(),
        )
        .await;

        Self {
            search_service,
            sqlx_service,
            crawler_service,
        }
    }

    fn create_file_indexer_config(app_save_service: &Arc<AppSaveService>) -> FileIndexerConfig {
        FileIndexerConfig {
            buffer_size: 50_000_000,
            indexer_batch_size: 256,
            app_path: app_save_service.save_dir.clone(),
        }
    }

    fn initialize_search_service(config: &FileIndexerConfig) -> Arc<SearchIndexService> {
        Arc::new(SearchIndexService::new(config))
    }

    fn initialize_app_save_service(save_dir: AppSavePath, app_name: &str) -> Arc<AppSaveService> {
        Arc::new(AppSaveService::new(save_dir, app_name))
    }

    async fn initialize_sqlx_service(app_save_service: &Arc<AppSaveService>) -> Arc<SqlxService> {
        Arc::new(SqlxService::new_async(app_save_service).await)
    }

    async fn initialize_crawler_service(
        max_concurrent: usize,
        save_after_iters: usize,
        search_service: Arc<SearchIndexService>,
        sqlx_service: Arc<SqlxService>,
        save_service: Arc<AppSaveService>,
    ) -> Arc<FileCrawlerService> {
        Arc::new(
            FileCrawlerService::new_async(
                max_concurrent,
                save_after_iters,
                search_service,
                sqlx_service,
                save_service,
            )
            .await,
        )
    }
}
