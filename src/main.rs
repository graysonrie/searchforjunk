use actix_web::rt::signal;
use actix_web::{App, HttpServer};
use filesindex::api::controller::FilesIndexController;
use filesindex::file_indexer_config::FileIndexerConfig;
use filesindex::infrastructure::searchindex::service::SearchIndexService;
use std::sync::Arc;
use tokio::sync::Mutex;
mod filesindex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let index_path = r"H:\tantivy-out";
    let buffer_size: usize = 50_000_000;
    let port = "127.0.0.1:8080";

    let config = FileIndexerConfig::new(index_path, buffer_size);
    let search_index_service = Arc::new(Mutex::new(SearchIndexService::new(&config)));
    let sender = Arc::new(Mutex::new(
        search_index_service.lock().await.set_up_queue_pipeline(),
    ));

    // Clone the controller to share across workers
    let controller = Arc::new(FilesIndexController::new(
        search_index_service.clone(),
        sender,
    ));

    // Start the server
    let server = HttpServer::new(move || {
        let controller_clone = controller.clone(); // Clone the controller for each worker

        App::new().configure(move |cfg| {
            controller_clone.map_routes(cfg); // This is now synchronous
        })
    })
    .bind(port)?
    .run();

    println!("setup finished. App is running on {}", port);

    let shutdown_signal = signal::ctrl_c();

    tokio::select! {
        _ = server => {},
        _ = shutdown_signal => {
            println!("Shutting down...");
        },
    }

    Ok(())
}
