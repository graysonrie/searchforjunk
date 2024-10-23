use std::sync::{mpsc, Arc};
use tantivy::{doc, schema::Schema, Index};
use tokio::task;
use crate::filesindex::api::dtos::file_dto::FileDTO;

async fn index_worker(mut rx: mpsc::Receiver<FileDTO>, index_writer: &mut tantivy::IndexWriter, schema: &Schema) {
    while let Ok(dto) = rx.recv() {

        let _ = index_writer.add_document(doc!(
            schema.get_field("title").unwrap() => dto.name,
        ));

        index_writer.commit().unwrap(); // Commit changes after indexing
    }
}