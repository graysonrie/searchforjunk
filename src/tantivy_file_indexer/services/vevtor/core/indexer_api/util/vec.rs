use std::sync::Arc;

use tokio::sync::RwLock;

pub async fn extract_vec_from_lock<T>(shared_vec: Arc<RwLock<Vec<T>>>) -> Vec<T> {
    let mut vec_guard = shared_vec.write().await;
    std::mem::take(&mut *vec_guard)
}