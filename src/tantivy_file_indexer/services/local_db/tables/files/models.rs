#[derive(Debug, sqlx::FromRow)]
pub struct FileModel {
    pub path: String,
    pub parent_path: Option<String>
}