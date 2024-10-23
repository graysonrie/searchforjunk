use std::{
    fs,
    future::Future,
    path::PathBuf,
    sync::{Arc, Mutex},
};
use tantivy::{
    collector::TopDocs, query::FuzzyTermQuery, schema::Schema, Index, IndexWriter,
    TantivyError,
};
use tokio::{sync::mpsc, task};

use crate::filesindex::{api::dtos::file_dto::FileDTO, file_indexer_config::FileIndexerConfig};

use super::{
    file_indexer::index_files, models::query_result_model::QueryResult, queue::index_worker,
    schemas::file_schema::create_schema,
};

pub struct SearchIndexService {
    schema: Schema,
    tantivy_out_path: PathBuf,
    index: Index,
    index_writer: Arc<Mutex<IndexWriter>>,
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
            fs::create_dir_all(index_path.clone()).expect("could not create output directory");
            Index::create_in_dir(index_path, schema.clone())
        };
        let index = index.unwrap();
        let index_writer = index.writer(config.buffer_size).unwrap();

        let writer_clone = Arc::new(Mutex::new(index_writer));

        Self {
            schema,
            tantivy_out_path: config.tantivy_out_path.clone(),
            index,
            index_writer: writer_clone,
        }
    }

    pub fn query(&self, query_str: &str) -> Result<Vec<QueryResult>, TantivyError> {
        let reader = self.index.reader()?;
        let searcher = reader.searcher();

        let field = self.schema.get_field("file_name").unwrap();
        let term = tantivy::Term::from_field_text(field, &query_str);

        let query = FuzzyTermQuery::new(term, 2, true);

        let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

        let results: Vec<QueryResult> = top_docs
            .iter()
            .map(|x| QueryResult::new(searcher.doc(x.1).unwrap(), x.0))
            .collect();

        Ok(results)
    }

    pub fn set_up_queue_pipeline(&self) -> mpsc::Sender<FileDTO> {
        let (sender, receiver) = mpsc::channel::<FileDTO>(32);
        let index_writer_clone = Arc::clone(&self.index_writer);
        let schema_clone = self.schema.clone();

        tokio::spawn(async move {
            index_worker::index_worker(receiver, index_writer_clone, schema_clone).await
        });
        sender
    }
}
