use std::{fs, future::Future, path::PathBuf, sync::Arc};
use tantivy::{
    collector::TopDocs,
    query::{BooleanQuery, FuzzyTermQuery, Occur, Query, QueryParser, RangeQuery, TermQuery},
    schema::{Facet, Schema, Value},
    DateTime, DocId, Index, IndexReader, IndexWriter, Score, SegmentReader, TantivyDocument, Term,
};
use tokio::sync::{mpsc, Mutex};

use crate::filesindex::{
    api::dtos::{input::file_dto_input::FileDTOInput, output::file_dto_output::FileDTOOutput},
    file_indexer_config::FileIndexerConfig,
};

use super::{
    converters::doc_to_dto::doc_to_dto, models::search_params_model::SearchParamsModel,
    queue::index_worker, schemas::file_schema::create_schema,
    scorers::pop_scorer::apply_popularity,
};

pub struct SearchIndexService {
    schema: Schema,
    index_writer: Arc<Mutex<IndexWriter>>,
    index_reader: IndexReader,
    config: FileIndexerConfig,
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
        let index_reader = index.reader().unwrap();

        Self {
            config: config.clone(),
            schema,
            index_writer: writer_clone,
            index_reader,
        }
    }

    pub fn advanced_query(
        &self,
        search_params: &SearchParamsModel, // Struct holding the user's search criteria
    ) -> tantivy::Result<Vec<FileDTOOutput>> {
        let schema = &self.schema;
        let searcher = self.index_reader.searcher();

        let mut queries: Vec<(Occur, Box<dyn Query>)> = Vec::new();

        if let Some(file_path) = &search_params.file_path {
            let field = schema.get_field("path").unwrap();
            let query_parser = QueryParser::for_index(&searcher.index(), vec![field]);
            let query = query_parser.parse_query(&file_path)?;
            queries.push((Occur::Should, Box::new(query)));
        }

        if let Some(query_str) = &search_params.name {
            let field = schema.get_field("name").unwrap();
            let query_parser = QueryParser::for_index(&searcher.index(), vec![field]);
            let query = query_parser.parse_query(query_str)?;
            queries.push((Occur::Should, Box::new(query)));
        }

        if let Some(date_range) = &search_params.date_range {
            let start_date = DateTime::from_utc(date_range.start);
            let end_date = DateTime::from_utc(date_range.end);
            let query = RangeQuery::new_date("date_modified".to_string(), start_date..end_date);
            queries.push((Occur::Must, Box::new(query)));
        }

        if let Some(metadata) = &search_params.metadata {
            let field = schema.get_field("metadata").unwrap();
            let term = Term::from_field_text(field, metadata);
            let query = TermQuery::new(term, tantivy::schema::IndexRecordOption::Basic);
            queries.push((Occur::Must, Box::new(query)));
        }

        // Combine all the queries into a BooleanQuery
        let boolean_query = BooleanQuery::new(queries);

        // Execute the query and collect the results
        let top_docs = searcher.search(
            &boolean_query,
            &TopDocs::with_limit(10).tweak_score(|segment_reader: &SegmentReader| {
                let popularity_field = segment_reader
                    .fast_fields()
                    .f64("popularity")
                    .expect("Failed to access popularity field");
                move |doc: DocId, original_score: Score| {
                    // Default to 1 if no popularity
                    let pop_score = popularity_field.first(doc).unwrap_or(1.0);
                    let tweaked = apply_popularity(original_score, pop_score);
                    tweaked
                }
            }),
        )?;

        let results: Vec<FileDTOOutput> = top_docs
            .into_iter()
            .map(|(_score, doc_address)| {
                let doc: TantivyDocument = searcher.doc(doc_address).unwrap();
                doc_to_dto(doc, &schema, _score)
            })
            .collect();

        Ok(results)
    }

    pub fn set_up_queue_pipeline(&self) -> mpsc::Sender<FileDTOInput> {
        let (sender, receiver) = mpsc::channel::<FileDTOInput>(32);
        let index_writer_clone = Arc::clone(&self.index_writer);
        let schema_clone = self.schema.clone();
        let batch_size = self.config.indexer_batch_size;

        tokio::spawn(async move {
            index_worker::index_worker(receiver, index_writer_clone, schema_clone, batch_size).await
        });
        sender
    }
}
