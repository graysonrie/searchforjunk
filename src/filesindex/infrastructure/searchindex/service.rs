use std::{fs, path::PathBuf};
use tantivy::{collector::TopDocs, query::FuzzyTermQuery, schema::Schema, Document, Index, IndexWriter, TantivyDocument, TantivyError};

use crate::filesindex::{api::dtos::file_dto::FileDTO, file_indexer_config::FileIndexerConfig};

use super::{file_indexer::index_file, models::query_result_model::QueryResult, schemas::file_schema::create_schema};

pub struct SearchIndexService {
    schema: Schema,
    tantivy_out_path: PathBuf,
    index: Index,
    index_writer: IndexWriter,
}

impl SearchIndexService {
    pub fn new(config: &FileIndexerConfig) -> Self {
        let schema = create_schema();
        let index_path = config.tantivy_out_path.clone();

        let index = if index_path.exists() {
            // If the index directory exists, open the existing index
            println!("Opening existing index at {:?}", index_path);
            Index::open_in_dir(index_path)
        } else {
            // If the index directory doesn't exist, create a new index
            println!("Creating a new index at {:?}", index_path);
            fs::create_dir_all(index_path.clone()); // Ensure the directory exists
            Index::create_in_dir(index_path, schema.clone())
        };
        let index = index.unwrap();
        let index_writer = index.writer(config.buffer_size).unwrap();

        Self {
            schema,
            tantivy_out_path: config.tantivy_out_path.clone(),
            index,
            index_writer,
        }
    }

    pub fn index_files(&mut self, files: Vec<&FileDTO>) {
        for file in files.iter() {
            index_file(&mut self.index_writer, &self.schema, file);
        }
    }

    pub fn query(&self, query_str:&str) -> Result<Vec<QueryResult>,TantivyError>{
        let reader = self.index.reader()?;
        let searcher = reader.searcher();

        // Fuzzy search query (max_distance 1)
        let field = self.schema.get_field("file_name").unwrap();
        let term = tantivy::Term::from_field_text(field, &query_str);

        let query = FuzzyTermQuery::new(term, 2, true);

        let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

        let results:Vec<QueryResult> = top_docs.iter().map(|x| QueryResult::new(searcher.doc(x.1).unwrap(),x.0)).collect();

        Ok(results)
    }

}
