use serde::{Deserialize, Serialize};
use tantivy::time::OffsetDateTime;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SearchParamsModel {
    pub name: Option<String>,
    pub metadata: Option<String>,
    pub date_range: Option<DateRange>,
    pub file_path: Option<String>,
}