use crate::filesindex::api::dtos::file_dto::FileDTO;
use std::sync::{Arc, Mutex};
use tantivy::{doc, schema::Schema, IndexWriter};
use tokio::{sync::mpsc, task};

pub async fn index_worker(
    mut rx: mpsc::Receiver<FileDTO>,
    index_writer: Arc<Mutex<IndexWriter>>,
    schema: Schema,
) {
    let mut batch_on: u32 = 0;
    let batch_size: u32 = 16;

    while let Some(dto) = rx.recv().await {
        {
            // Lock the writer once per loop iteration
            let mut writer = index_writer.lock().unwrap();

            writer
                .add_document(doc!(
                    schema.get_field("file_name").unwrap() => dto.name,
                ))
                .unwrap(); // Consider proper error handling here
            batch_on += 1;

            if batch_on >= batch_size {
                println!("writer commit");
                writer.commit().unwrap();
                batch_on = 0;
            }
        }
    }

    if batch_on > 0 {
        println!("writer commit");
        index_writer.lock().unwrap().commit().unwrap();
    }
}
