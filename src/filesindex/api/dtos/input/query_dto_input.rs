use serde::{Deserialize, Serialize};


#[derive(Clone, Deserialize, Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct QueryDTOInput {
    pub search_term:String,
    pub query:String
}
