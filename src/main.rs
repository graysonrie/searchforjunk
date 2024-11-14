use actix_cors::Cors;
use actix_web::rt::signal;
use actix_web::{http, App, HttpServer};
use filesindex::api::controller::FilesIndexController;
use filesindex::file_indexer_config::FileIndexerConfig;
use filesindex::infrastructure::searchindex::service::SearchIndexService;
use tantivy_file_indexer::service_container::AppServiceContainer;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
mod filesindex;
mod tantivy_file_indexer;
mod shared;

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    initialize_services().await;

    Ok(())
}

async fn initialize_services() {

    let index_files = true;

    let service_container = AppServiceContainer::new_async().await;
    let crawler_service = service_container.crawler_service.clone();
    let db_service = service_container.sqlx_service.clone();

    if index_files {
        let sender = service_container
            .search_service
            .spawn_indexer(db_service, 128, 4);

        crawler_service.spawn_crawler(sender);
        crawler_service.load_or(vec!["C:\\"]).await;
    }

}