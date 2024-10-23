use std::fs::{self, DirEntry};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use tantivy::schema::*;
use tantivy::{doc, DateTime, Index};

fn system_time_to_tantivy_datetime(system_time: SystemTime) -> DateTime {
    let duration_since_epoch = system_time.duration_since(UNIX_EPOCH).unwrap();
    let timestamp_secs = duration_since_epoch.as_secs(); // Unix timestamp in seconds
    DateTime::from_timestamp_secs(timestamp_secs as i64)
}

// Function to read file metadata and index files
pub fn index_files_in_directory(
    index_writer: &mut tantivy::IndexWriter,
    schema: &Schema,
    dir_path: &str,
) -> tantivy::Result<()> {
    let file_name_field = schema.get_field("file_name").unwrap();
    let metadata_field = schema.get_field("metadata").unwrap();
    let date_created_field = schema.get_field("date_created").unwrap();
    let date_modified_field = schema.get_field("date_modified").unwrap();
    let file_content_field = schema.get_field("file_content").unwrap();

    // Iterate over files in the directory
    for entry in fs::read_dir(dir_path)? {
        let entry = entry?;
        let path = entry.path();

        if path.is_file() {
            // Get file metadata
            let metadata = fs::metadata(&path)?;
            let file_name = path.file_name().unwrap().to_string_lossy().to_string();
            let date_created =
                system_time_to_tantivy_datetime(metadata.created().unwrap_or(SystemTime::now()));
            let date_modified =
                system_time_to_tantivy_datetime(metadata.modified().unwrap_or(SystemTime::now()));

            // Optionally read file content (only if it's a text file, for example)
            let file_content = if let Some(extension) = path.extension() {
                if extension == "txt" {
                    fs::read_to_string(&path).unwrap_or_else(|_| "".to_string())
                } else {
                    "".to_string() // Only index text content for certain file types
                }
            } else {
                "".to_string()
            };

            // Add the document to the index
            let _ = index_writer.add_document(doc!(
                file_name_field => file_name,
                metadata_field => "Some custom metadata",
                date_created_field => date_created,
                date_modified_field => date_modified,
                file_content_field => file_content
            ));
            index_writer.commit()?;
        }
    }
    Ok(())
}
