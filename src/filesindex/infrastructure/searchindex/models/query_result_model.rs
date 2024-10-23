use tantivy::TantivyDocument;

pub struct QueryResult {
    document: TantivyDocument,
    score: f32,
}

impl QueryResult {
    pub fn new(document: TantivyDocument, score: f32) -> Self {
        Self { document, score }
    }
}
