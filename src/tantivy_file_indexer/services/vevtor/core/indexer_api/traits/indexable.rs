use std::collections::HashMap;

use qdrant_client::qdrant::Value;

pub trait Indexable: Send + Sync + 'static {
    type Output;

    fn as_map(&self) -> HashMap<String, Value>;

    fn from_qdrant_payload(
        payload: &std::collections::HashMap<String, Value>,
        collection: String,
    ) -> Result<Self::Output, String>;

    fn get_id(&self) -> u64;

    fn collection(&self) -> String;

    fn embed_label(&self) -> &str;
}
