use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileDTO{
    pub name:String,
}