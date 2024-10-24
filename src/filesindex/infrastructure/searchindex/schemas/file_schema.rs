use tantivy::schema::{Schema, INDEXED, STORED, TEXT};

pub fn create_schema() -> Schema {
    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field("file_id", TEXT | STORED); // UID
    
    schema_builder.add_text_field("name", TEXT | STORED); 
    schema_builder.add_date_field("date_modified", INDEXED | STORED); 
    schema_builder.add_text_field("metadata", TEXT| STORED);
    schema_builder.add_text_field("path", TEXT | STORED); 

    schema_builder.build()
}