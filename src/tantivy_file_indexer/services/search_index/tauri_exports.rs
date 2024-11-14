/*
use std::sync::Arc;

use tauri::State;

use crate::{
    shared::dtos::file_dto::FileDTO,
    tantivy_file_indexer::models::search_params_model::SearchParamsModel,
};

use super::service::SearchIndexService;

#[tauri::command]
pub fn search_index_query(
    params: SearchParamsModel,
    service: State<'_, Arc<SearchIndexService>>,
) -> Result<Vec<FileDTO>, String> {
    match service.query(&params) {
        Ok(result) => Ok(result),
        Err(err) => Err(err.to_string()),
    }
}
*/