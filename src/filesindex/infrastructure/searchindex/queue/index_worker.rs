use crate::filesindex::{
    api::dtos::input::file_dto_input::FileDTOInput,
    infrastructure::searchindex::converters::{
        date_converter::unix_time_to_tantivy_datetime, path_to_facet::windows_path_to_facet,
    },
};

use std::{path::Path, sync::Arc, time::Duration};
use tantivy::{
    doc,
    schema::{Facet, Schema},
    Document, IndexWriter, TantivyDocument, TantivyError,
};
use tokio::{
    sync::{mpsc, Mutex},
    task,
};

pub async fn index_worker(
    mut rx: mpsc::Receiver<FileDTOInput>,
    index_writer: Arc<Mutex<IndexWriter>>,
    schema: Schema,
    batch_size:usize,
) {
    let mut batch_on: usize = 0;

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

                //let formatted_facet_path = windows_path_to_facet(&dto.file_path);

                writer
                .add_document(doc!(
                    schema.get_field("file_id").unwrap() => dto.file_id,
                    schema.get_field("name").unwrap() => dto.name,
                    schema.get_field("date_modified").unwrap() => unix_time_to_tantivy_datetime(dto.date_modified),
                    schema.get_field("path").unwrap() => dto.file_path,
                    schema.get_field("metadata").unwrap() => dto.metadata,
                    schema.get_field("popularity").unwrap() => dto.popularity,
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

fn build_facet_from_file_path(path: &str) -> Facet {
    let path = Path::new(path);
    let mut facet_path = String::from("/");
    for component in path.components() {
        facet_path.push_str(&component.as_os_str().to_string_lossy());
        facet_path.push('/');
    }
    Facet::from(&facet_path)
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
