use qdrant_client::qdrant::{
    CollectionOperationResponse, CreateCollectionBuilder, Distance, ScalarQuantizationBuilder,
    VectorParamsBuilder,
};
use qdrant_client::{Qdrant, QdrantError};

use super::builders::with_collection::WithCollectionBuilder;

pub struct QdrantApi {
    client: Qdrant,
}

impl QdrantApi {
    pub fn new(url: &str) -> Self {
        let client = Qdrant::from_url(url)
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .unwrap();
        Self { client }
    }

    pub async fn create_collection(
        &self,
        name: &str,
        num_features: u64,
    ) -> Result<WithCollectionBuilder, QdrantError> {
        self.client
            .create_collection(
                CreateCollectionBuilder::new(name)
                    .vectors_config(VectorParamsBuilder::new(num_features, Distance::Cosine))
                    .quantization_config(ScalarQuantizationBuilder::default()),
            )
            .await
            .map(|_| self.with_collection(name))
    }

    pub async fn delete_collections(&self, names:&Vec<&str>){
        for collection in names.iter(){
            self.delete_collection(&collection).await;
        }
    }

    pub async fn delete_collection(
        &self,
        name: &str,
    ) -> Result<CollectionOperationResponse, QdrantError> {
        println!("Deleting collection '{}'",name);
        self.client.delete_collection(name).await
    }

    pub fn with_collection(&self, collection: &str) -> WithCollectionBuilder {
        WithCollectionBuilder::new(&self.client, collection)
    }

    pub async fn list_collections(&self) -> Vec<String> {
        self
            .client
            .list_collections()
            .await
            .into_iter()
            .flat_map(|collection| collection.collections.into_iter())
            .map(|response| response.name)
            .collect()
    }
}
