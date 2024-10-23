use crate::filesindexer::indexer::index_files_in_directory;
use crate::filesindexer::schemacreator::create_schema;
use std::fs;
use std::path::Path;
use tantivy::collector::TopDocs;
use tantivy::query::{FuzzyTermQuery, QueryParser};
use tantivy::schema::*;
use tantivy::{doc, Index, Result};
mod filesindexer;

fn main() -> Result<()> {
    // Define the schema
    let schema = create_schema();
    let dir_path = r"C:\Users\grays\Downloads";
    let query_str = "test";

    let index_path = Path::new(r"D:\tantivy-out");

    // Check if the index directory exists
    let index = if index_path.exists() {
        // If the index directory exists, open the existing index
        println!("Opening existing index at {:?}", index_path);
        Index::open_in_dir(index_path)?
    } else {
        // If the index directory doesn't exist, create a new index
        println!("Creating a new index at {:?}", index_path);
        fs::create_dir_all(index_path)?; // Ensure the directory exists
        Index::create_in_dir(index_path, schema.clone())?
    };

    // Create or open the index writer
    let mut index_writer = index.writer(50_000_000)?;

    // Only index files if we're creating the index for the first time
    if !index_path.exists() {
        let _ = index_files_in_directory(&mut index_writer, &schema, dir_path);
        index_writer.commit()?;
    }

    // Create a searcher
    let reader = index.reader()?;
    let searcher = reader.searcher();

    // Fuzzy search query (max_distance 1)
    let field = schema.get_field("file_name").unwrap();
    let term = tantivy::Term::from_field_text(field, &query_str);

    let query = FuzzyTermQuery::new(term, 2, true);

    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;

    for (_score, doc_address) in top_docs {
        let retrieved_doc: TantivyDocument = searcher.doc(doc_address)?;
        println!("{}", retrieved_doc.to_json(&schema));
    }

    Ok(())
}
