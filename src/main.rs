use crate::filesindex::file_indexer_core;
use actix_web::rt::signal;
use actix_web::{App, HttpServer};
use filesindex::file_indexer_config::FileIndexerConfig;
mod filesindex;
mod filesindexer;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let index_path = r"D:\tantivy-out";
    let buffer_size: usize = 50_000_000;

    let config = FileIndexerConfig::new(index_path, buffer_size);

    let server = HttpServer::new(move || file_indexer_core::init(App::new(), &config))
        .bind("127.0.0.1:8080")?
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
