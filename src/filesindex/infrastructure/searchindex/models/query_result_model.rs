use tantivy::TantivyDocument;
use serde::{Deserialize, Serialize};


#[derive(Clone, Deserialize, Serialize)]
pub struct QueryResult {
    pub document: TantivyDocument,
    pub score: f32,
}


impl QueryResult {
    pub fn new(document: TantivyDocument, score: f32) -> Self {
        Self { document, score }
    }
}
