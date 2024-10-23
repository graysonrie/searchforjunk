use actix_web::rt::signal;
use actix_web::{App, HttpServer};
use filesindex::api::controller::FilesIndexController;
use filesindex::file_indexer_config::FileIndexerConfig;
use filesindex::infrastructure::searchindex::service::SearchIndexService;
use std::sync::{Arc, Mutex};
mod filesindex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let index_path = r"D:\tantivy-out22";
    let buffer_size: usize = 50_000_000;

    let port = "127.0.0.1:8080";

    let config = FileIndexerConfig::new(index_path, buffer_size);
    let search_index_service = Arc::new(Mutex::new(SearchIndexService::new(&config)));

    // Pass the shared instance to each worker
    let server = HttpServer::new(move || {
        // Clone the shared Arc for each worker
        let app = App::new();
        let service_clone = search_index_service.clone();   
        let controller = Arc::new(FilesIndexController::new(service_clone));

        let app = controller.map_routes(app);
        println!("setup finished. App is running on {}", port);
        app
    })
    .bind(port)?
    .run();

    let shutdown_signal = signal::ctrl_c();

    tokio::select! {
        _ = server => {},
        _ = shutdown_signal => {
            println!("Shutting down...");
        },
    }

    Ok(())
}
