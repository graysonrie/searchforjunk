use std::sync::{Arc, Mutex};

use actix_web::{dev::{ServiceFactory, ServiceRequest}, web, App, Error, HttpResponse, Responder};
use crate::filesindex::infrastructure::searchindex::service::SearchIndexService;
use super::dtos::file_dto::FileDTO;

pub struct FilesIndexController {
    service: Arc<Mutex<SearchIndexService>>,
}

impl FilesIndexController {
    pub fn new(service: Arc<Mutex<SearchIndexService>>) -> Self {
        Self { service }
    }

    async fn index_files(self: Arc<Self>, files: web::Json<Vec<FileDTO>>) -> impl Responder {
        let files = files.into_inner();

        let mut service = self.service.lock().unwrap();
        service.index_files(files.iter().collect());

        HttpResponse::Ok()
    }

    pub fn map_routes<T>(self: Arc<Self>, app: App<T>) -> App<T>
    where
        T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()>,
    {
        app.route("/add-file", web::post().to(move |files| self.clone().index_files(files)))
    }
}

