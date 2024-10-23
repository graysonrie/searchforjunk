use std::sync::{Arc, Mutex};

use actix_web::{dev::{ServiceFactory, ServiceRequest}, App, Error};
use super::{api::controller::FilesIndexController, file_indexer_config::FileIndexerConfig, infrastructure::searchindex::service::SearchIndexService};

pub fn init<T>(app:App<T>, config:&FileIndexerConfig) -> App<T> 
where T: ServiceFactory<ServiceRequest, Config = (), Error = Error, InitError = ()> {
    let search_index_service = SearchIndexService::new(&config);
    let controller = Arc::new(FilesIndexController::new(Arc::new(Mutex::new(search_index_service))));

    let app = controller.map_routes(app);
    app
}