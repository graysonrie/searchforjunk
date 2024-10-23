use tantivy::schema::{Schema, TEXT, STORED, INDEXED};

pub fn create_schema() -> Schema {
    let mut schema_builder = Schema::builder();
    
    // Add fields to the schema
    schema_builder.add_text_field("file_name", TEXT | STORED);   // File name
    schema_builder.add_text_field("metadata", TEXT | STORED);    // Custom metadata
    schema_builder.add_date_field("date_created", INDEXED | STORED); // Date created
    schema_builder.add_date_field("date_modified", INDEXED | STORED); // Date modified
    schema_builder.add_text_field("file_content", TEXT | STORED); // File content, optional

    schema_builder.build()
}