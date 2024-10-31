use actix_cors::Cors;
use actix_web::rt::signal;
use actix_web::{http, App, HttpServer};
use filesindex::api::controller::FilesIndexController;
use filesindex::file_indexer_config::FileIndexerConfig;
use filesindex::infrastructure::searchindex::service::SearchIndexService;
use std::path::Path;
use std::sync::Arc;
use tokio::sync::Mutex;
mod filesindex;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let index_path = r"D:\tantivy-out";
    let buffer_size: usize = 50_000_000;
    let indexer_batch_size: usize = 128;

    let port = "127.0.0.1:8080";

    let config = FileIndexerConfig {
        tantivy_out_path: Path::new(index_path).to_path_buf(),
        buffer_size,
        indexer_batch_size,
    };

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

        App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:1420")
                    .allow_any_method()
                    .allowed_headers(vec![http::header::CONTENT_TYPE, http::header::ACCEPT])
                    .supports_credentials(), // Allow cookies if needed
            )
            .configure(move |cfg| {
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
