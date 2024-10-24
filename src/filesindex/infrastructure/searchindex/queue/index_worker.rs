use crate::filesindex::{
    api::dtos::input::file_dto_input::FileDTOInput,
    infrastructure::searchindex::converters::date_converter::unix_time_to_tantivy_datetime,
};

use std::{sync::Arc, time::Duration};
use tantivy::{doc, schema::Schema, IndexWriter, TantivyError};
use tokio::{
    sync::{mpsc, Mutex},
    task,
};

pub async fn index_worker(
    mut rx: mpsc::Receiver<FileDTOInput>,
    index_writer: Arc<Mutex<IndexWriter>>,
    schema: Schema,
) {
    let mut batch_on: u32 = 0;
    let batch_size: u32 = 16;

    while let Some(dto) = rx.recv().await {
        {
            // Lock the writer once per loop iteration
            let file_removed =
                remove_file_from_index(index_writer.clone(), &schema, &dto.file_path).await;
            match file_removed {
                Ok(_) => {}
                Err(err) => {
                    println!("Error when trying to remove file: {}", err)
                }
            }

            // Lock the writer only while adding the document
            {
                let writer = index_writer.lock().await;

                writer
                .add_document(doc!(
                    schema.get_field("file_id").unwrap() => dto.file_id,
                    schema.get_field("name").unwrap() => dto.name,
                    schema.get_field("date_modified").unwrap() => unix_time_to_tantivy_datetime(dto.date_modified),
                    schema.get_field("path").unwrap() => dto.file_path,
                    schema.get_field("metadata").unwrap() => dto.metadata,
                ))
                .unwrap(); // Consider proper error handling here

                batch_on += 1;
            } // Release the lock after adding the document

            if batch_on >= batch_size {
                let _ = commit_and_retry(index_writer.clone()).await;
                batch_on = 0;
            }
        }
    }

    if batch_on > 0 {
        let result = commit_and_retry(index_writer.clone()).await;
        if let Err(e) = result {
            println!("Final writer commit attempt failed: {}", e)
        }
    }
}

async fn commit_and_retry(writer: Arc<Mutex<IndexWriter>>) -> Result<(), TantivyError> {
    let retry_attempts = 3;
    for attempt in 1..=retry_attempts {
        match writer.lock().await.commit() {
            Ok(_) => break, // Success, exit the loop
            Err(e) if attempt < retry_attempts => {
                println!("Commit failed on attempt {}, retrying: {:?}", attempt, e);
                tokio::time::sleep(Duration::from_millis(500)).await; // Add delay
            }
            Err(e) => {
                println!("Commit failed after {} attempts: {:?}", retry_attempts, e);
                return Err(e);
            }
        }
    }
    println!("writer commit");
    return Ok(());
}

async fn remove_file_from_index(
    index_writer: Arc<Mutex<IndexWriter>>,
    schema: &Schema,
    file_path: &str,
) -> tantivy::Result<()> {
    let index_writer = index_writer.lock().await;
    match schema.get_field("file_id") {
        Ok(field) => {
            index_writer.delete_term(tantivy::Term::from_field_text(field, file_path));
            Ok(())
        }
        Err(e) => Err(e),
    }
}
