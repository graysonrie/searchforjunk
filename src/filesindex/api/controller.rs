use crate::filesindex::infrastructure::searchindex::{
    models::search_params_model::SearchParamsModel, service::SearchIndexService,
};
use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    web::{self, ServiceConfig},
    App, Error, HttpResponse, Responder,
};
use futures::future;
use std::sync::Arc;
use tokio::sync::{mpsc::Sender, Mutex};

use super::dtos::{
    input::{self, file_dto_input::FileDTOInput, query_dto_input::QueryDTOInput},
    output::file_dto_output::FileDTOOutput,
};

pub struct FilesIndexController {
    service: Arc<Mutex<SearchIndexService>>,
    sender: Arc<Mutex<Sender<FileDTOInput>>>,
}

impl FilesIndexController {
    pub fn new(
        service: Arc<Mutex<SearchIndexService>>,
        sender: Arc<Mutex<Sender<FileDTOInput>>>,
    ) -> Self {
        Self { service, sender }
    }

    async fn index_files(
        self: Arc<Self>,
        files: web::Json<Vec<FileDTOInput>>,
        sender: Arc<Mutex<Sender<FileDTOInput>>>,
    ) -> impl Responder {
        let files = files.into_inner();
        let sender_clone = Arc::clone(&sender);

        // Spawn a separate task to process the files asynchronously
        tokio::spawn(async move {
            let mut send_futures = vec![];

            for file in files {
                let sender = Arc::clone(&sender_clone);

                // Spawn a task for each file to send it to the worker
                let send_future = async move {
                    let sender = sender.lock().await;
                    let _ = sender.send(file.clone()).await;
                };

                send_futures.push(send_future);
            }

            // Wait for all files to be sent (this runs in the background)
            future::join_all(send_futures).await;
        });

        // Return Ok immediately after spawning the task
        HttpResponse::Ok()
    }

    async fn query(self: Arc<Self>, dto: web::Json<SearchParamsModel>) -> impl Responder {
        println!("locking service");
        let service = self.service.lock().await;
        let result = match service.advanced_query(&dto) {
            Ok(result) => result,
            Err(err) => {
                println!("failed to execute query: {}", err);
                return HttpResponse::BadRequest().body("Failed to execute query.");
            }
        };
        println!("completed query with {} results", result.len());
        HttpResponse::Ok().json(result)
    }

    pub fn map_routes(self: Arc<Self>, cfg: &mut ServiceConfig) {
        cfg.route(
            "/index-files",
            web::post().to({
                let self_clone = Arc::clone(&self); // Cloning the Arc to avoid moving
                let sender_clone = Arc::clone(&self.sender);
                move |files| {
                    let self_clone = Arc::clone(&self_clone);
                    let sender_clone = Arc::clone(&sender_clone);
                    async move { self_clone.index_files(files, sender_clone).await }
                }
            }),
        )
        .route(
            "/query",
            web::post().to({
                let self_clone = Arc::clone(&self); // Cloning the Arc to avoid moving
                move |dtos| {
                    let self_clone = Arc::clone(&self_clone);
                    async move { self_clone.query(dtos).await }
                }
            }),
        );
    }
}
