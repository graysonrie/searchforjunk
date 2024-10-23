use std::fs::{self, DirEntry};
use std::time::{SystemTime, UNIX_EPOCH};
use tantivy::schema::*;
use tantivy::{doc, DateTime, Index};

use crate::filesindex::api::dtos::file_dto::FileDTO;


fn system_time_to_tantivy_datetime(system_time: SystemTime) -> DateTime {
    let duration_since_epoch = system_time.duration_since(UNIX_EPOCH).unwrap();
    let timestamp_secs = duration_since_epoch.as_secs(); // Unix timestamp in seconds
    DateTime::from_timestamp_secs(timestamp_secs as i64)
}

// Function to read file metadata and index files
pub fn index_files(
    index_writer: &tantivy::IndexWriter,
    schema: &Schema,
    files: Vec<&FileDTO>,
) -> tantivy::Result<()> {
    let file_name_field = schema.get_field("file_name").unwrap();
    //let metadata_field = schema.get_field("metadata").unwrap();
    //let date_created_field = schema.get_field("date_created").unwrap();
    //let date_modified_field = schema.get_field("date_modified").unwrap();
    //let file_content_field = schema.get_field("file_content").unwrap();

    for file in files.iter(){
        let _ = index_writer.add_document(doc!(
            file_name_field => file.name.clone(),
            //date_created_field => system_time_to_tantivy_datetime(file.date_created),
        ));
    }
      
    Ok(())
}