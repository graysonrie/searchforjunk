use serde::{Deserialize, Serialize};


#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileDTOInput {
    pub file_id: String,
    pub name: String,
    pub file_path: String,
    pub metadata: String,
    pub date_modified: u64, // UNIX timestamp
    pub popularity:f64,
}

