use std::collections::HashMap;

use qdrant_client::qdrant::{DeletePointsBuilder, PointStruct, PointsIdsList, UpsertPointsBuilder};
use qdrant_client::qdrant::{PointsOperationResponse, SearchPointsBuilder};
use qdrant_client::{Qdrant, QdrantError};

type EmbeddingResult = (HashMap<String, qdrant_client::qdrant::Value>, f32);
pub type Embeddings = Vec<f32>;
pub struct WithCollectionBuilder<'a> {
    client: &'a Qdrant,
    collection: String,
}

impl<'a> WithCollectionBuilder<'a> {
    pub fn new(client: &'a Qdrant, collection: &str) -> Self {
        Self {
            client,
            collection: collection.to_string(),
        }
    }

    pub async fn insert_many<T>(&self, data: Vec<(Embeddings, T, u64)>)
    where
        T: std::convert::Into<qdrant_client::Payload>,
    {
        for (embedding, payload, id) in data.into_iter() {
            _ = self.insert(embedding, payload, id).await;
        }
    }

    pub async fn insert<T>(
        &self,
        embeddings: Embeddings,
        payload: T,
        id: u64,
    ) -> Result<PointsOperationResponse, QdrantError>
    where
        T: std::convert::Into<qdrant_client::Payload>,
    {
        let points = vec![PointStruct::new(
            id,         // Uniqe point ID
            embeddings, // Vector to upsert
            // Attached payload
            payload,
        )];
        self.client
            .upsert_points(UpsertPointsBuilder::new(&self.collection, points))
            .await
    }

    pub async fn remove(&self, id:u64)->Result<PointsOperationResponse, QdrantError>{
        self.remove_many(vec![id]).await
    }

    pub async fn remove_many(&self, ids:Vec<u64>)->Result<PointsOperationResponse, QdrantError>{
        self.client.delete_points(DeletePointsBuilder::new(&self.collection).points(PointsIdsList{
            ids: ids.into_iter().map(|x|x.into()).collect()
        })).await
    }

    pub async fn search(
        &self,
        embedding: Embeddings,
        top_k: u64,
    ) -> Result<Vec<EmbeddingResult>, QdrantError> {
        let search_request =
            SearchPointsBuilder::new(&self.collection, embedding, top_k).with_payload(true);

        self.client
            .search_points(search_request)
            .await
            .map(|response| {
                response
                    .result
                    .into_iter()
                    .map(|result| {
                        let score = result.score;
                        (result.payload, score)
                    })
                    .collect()
            })
    }
}
