use fastembed::{EmbeddingModel, Error, InitOptions, TextEmbedding};

type Embeddings = Vec<f32>;

pub struct EmbeddingsGenerator {
    model: TextEmbedding,
    pub embedding_dim_len: u64,
}

impl EmbeddingsGenerator {
    pub fn new() -> Self {
        let model = TextEmbedding::try_new(
            InitOptions::new(EmbeddingModel::AllMiniLML6V2).with_show_download_progress(true),
        )
        .unwrap();
        Self {
            model,
            embedding_dim_len: 384,
        }
    }

    pub fn embed(&self, document: &str) -> Result<Embeddings, String> {
        let embeddings = self
            .embed_many(vec![document])
            .map_err(|err| format!("Error generating embedding: {}", err))?;

        match embeddings.first() {
            Some(val) => Ok(val.clone()),
            None => Err("Embeddings have no embeddings... what?".to_string()),
        }
    }

    pub fn embed_many(&self, documents: Vec<&str>) -> Result<Vec<Embeddings>, Error> {
        let embeddings = self.model.embed(documents, None)?;
        // Extra validation
        for embedding in embeddings.iter() {
            assert_eq!(
                embedding.len(),
                self.embedding_dim_len as usize,
                "Embedding should be {} dims but is actually {}",
                self.embedding_dim_len,
                embedding.len()
            );
        }
        Ok(embeddings)
    }

    pub fn embed_named(&self, documents: Vec<&str>) -> Result<Vec<(Embeddings, String)>, Error> {
        self.embed_many(documents.clone()).map(|embeddings| {
            embeddings
                .into_iter()
                .zip(documents.into_iter().map(|x| x.to_string()))
                .collect()
        })
    }
}
