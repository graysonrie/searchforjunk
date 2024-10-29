use std::path::Path;

use chrono::Utc;
use tantivy::{
    schema::{OwnedValue, Schema},
    DateTime, Document, TantivyDocument,
};

use crate::filesindex::api::dtos::output::file_dto_output::FileDTOOutput;

pub fn doc_to_dto(doc: TantivyDocument, schema: &Schema, score: f64) -> FileDTOOutput {
    let mut name = String::new();
    let mut file_path = String::new();
    let mut metadata = String::new();
    let mut date_modified: Option<String> = None;

    // Iterate through the document fields and populate the DTO fields
    for (field, value) in doc.iter_fields_and_values() {
        let field_name = schema.get_field_name(field);

        match field_name {
            "name" => {
                if let OwnedValue::Str(text) = value {
                    name = text.to_string();
                }
            }
            "path" => {
                if let OwnedValue::Str(text) = value {
                    file_path = text.to_string();
                }
            }
            "metadata" => {
                if let OwnedValue::Str(text) = value {
                    metadata = text.to_string();
                }
            }
            "date_modified" => {
                if let OwnedValue::Date(date) = value {
                    date_modified = Some(date.into_utc().to_string());
                }
            }
            _ => {}
        }
    }
    let is_dir: bool = Path::new(file_path.as_str()).is_dir();
    // Construct and return the DTO
    FileDTOOutput {
        name,
        file_path,
        metadata,
        date_modified: date_modified.unwrap_or_else(|| Utc::now().to_string()),
        score,
        is_directory: is_dir,
    }
}
