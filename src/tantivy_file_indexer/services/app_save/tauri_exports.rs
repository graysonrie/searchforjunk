
/*use std::sync::Arc;
use tauri::State;

use super::service::AppSaveService;

#[tauri::command]
pub fn save_json_local(data: serde_json::Value, name:String, service: State<'_, Arc<AppSaveService>>) -> Result<(), String> {
    service.save(&name, data).map_err(|x| format!("Failed to save JSON locally: {}",x))
}

#[tauri::command]
pub fn load_json_local(name:String, service: State<'_, Arc<AppSaveService>>) -> Result<serde_json::Value, String> {
    service.load::<serde_json::Value>(&name).map_err(|x| format!("Failed to load JSON locally: {}",x))
}
    */