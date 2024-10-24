use crate::filesindex::{
    api::dtos::input::file_dto_input::FileDTOInput,
    infrastructure::searchindex::converters::date_converter::unix_time_to_tantivy_datetime,
};

use std::sync::{Arc, Mutex};
use tantivy::{doc, schema::Schema, IndexWriter};
use tokio::{sync::mpsc, task};

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
                remove_file_from_index(index_writer.clone(), &schema, &dto.file_path);
            match file_removed {
                Ok(_) => {}
                Err(err) => {
                    println!("Error when trying to remove file: {}", err)
                }
            }

            let mut writer = index_writer.lock().unwrap();

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

fn remove_file_from_index(
    index_writer: Arc<Mutex<IndexWriter>>,
    schema: &Schema,
    file_path: &str,
) -> tantivy::Result<()> {
    let index_writer = index_writer.lock()?;
    let file_path_field = schema.get_field("file_id").unwrap();
    index_writer.delete_term(tantivy::Term::from_field_text(file_path_field, file_path));
    Ok(())
}
