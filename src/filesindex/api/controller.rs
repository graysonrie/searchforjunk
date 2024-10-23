use super::dtos::file_dto::FileDTO;
use crate::filesindex::infrastructure::searchindex::service::SearchIndexService;
use actix_web::{
    dev::{ServiceFactory, ServiceRequest},
    web, App, Error, HttpResponse, Responder,
};
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::Sender;

pub struct FilesIndexController {
    service: Arc<Mutex<SearchIndexService>>,
}

impl FilesIndexController {
    pub fn new(service: Arc<Mutex<SearchIndexService>>) -> Self {
        Self { service }
    }

    async fn index_files(
        self: Arc<Self>,
        files: web::Json<Vec<FileDTO>>,
        sender: Arc<Mutex<Sender<FileDTO>>>,
    ) -> impl Responder {
        let files = files.into_inner();

        for file in files.iter() {
            let sender = sender.lock().unwrap();
            let _ = sender.send(file.clone()).await;
        }

        HttpResponse::Ok()
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
            web::post().to(move |files| self.clone().index_files(files, sender.clone())),
        )
    }
}
