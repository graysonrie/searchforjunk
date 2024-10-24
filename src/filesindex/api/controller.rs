use crate::filesindex::infrastructure::searchindex::{
    models::search_params_model::SearchParamsModel, service::SearchIndexService,
};
use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    web, App, Error, HttpResponse, Responder,
};
use std::sync::{Arc, Mutex};
use tantivy::Document;
use tokio::sync::mpsc::Sender;

use super::dtos::{
    input::{self, file_dto_input::FileDTOInput, query_dto_input::QueryDTOInput},
    output::file_dto_output::FileDTOOutput,
};

pub struct FilesIndexController {
    service: Arc<Mutex<SearchIndexService>>,
}

impl FilesIndexController {
    pub fn new(service: Arc<Mutex<SearchIndexService>>) -> Self {
        Self { service }
    }

    async fn index_files(
        self: Arc<Self>,
        files: web::Json<Vec<FileDTOInput>>,
        sender: Arc<Mutex<Sender<FileDTOInput>>>,
    ) -> impl Responder {
        let files = files.into_inner();

        for file in files.iter() {
            let sender = sender.lock().unwrap();
            let _ = sender.send(file.clone()).await;
        }

        HttpResponse::Ok()
    }

    // TODO: possibly remove
    async fn basic_query(self: Arc<Self>, dto: web::Json<QueryDTOInput>) -> impl Responder {
        println!("locking service");
        let service = match self.service.lock() {
            Ok(service) => service,
            Err(_) => {
                return HttpResponse::BadRequest().body("Failed to lock the service.");
            }
        };
        let result = match service.basic_query(&dto.search_term, &dto.query) {
            Ok(result) => result,
            Err(_) => {
                return HttpResponse::BadRequest().body("Failed to execute query.");
            }
        };
        println!("completed query");
        HttpResponse::Ok().json(result)
    }

    async fn query(self: Arc<Self>, dto: web::Json<SearchParamsModel>) -> impl Responder {
        println!("locking service");
        let service = match self.service.lock() {
            Ok(service) => service,
            Err(_) => {
                return HttpResponse::BadRequest().body("Failed to lock the service.");
            }
        };
        let result = match service.advanced_query(&dto) {
            Ok(result) => result,
            Err(_) => {
                return HttpResponse::BadRequest().body("Failed to execute query.");
            }
        };
        println!("completed query with {} results", result.len());
        HttpResponse::Ok().json(result)
    }

    pub fn map_routes<T>(self: Arc<Self>, app: App<T>) -> App<T>
    where
        T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
    {
        let sender = Arc::new(Mutex::new(
            self.service.lock().unwrap().set_up_queue_pipeline(),
        ));

        app.route(
            "/add-file",
            web::post().to({
                let self_clone = Arc::clone(&self); // Cloning the Arc to avoid moving
                let sender_clone = Arc::clone(&sender);
                move |files| self_clone.clone().index_files(files, sender_clone.clone())
            }),
        )
        .route(
            "/query",
            web::post().to({
                let self_clone = Arc::clone(&self); // Cloning the Arc to avoid moving
                move |dtos| self_clone.clone().query(dtos)
            }),
        )
    }
}
