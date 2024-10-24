use serde::{Deserialize, Serialize};
use tantivy::DateTime;

#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct FileDTOOutput {
    pub name: String,
    pub file_path: String,
    pub metadata: String,
    pub date_modified: String,
    pub score: f32,
}
