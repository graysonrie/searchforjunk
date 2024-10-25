pub fn apply_popularity(existing_score: f32, popularity_score: f64) -> f64 {
    (existing_score as f64) + popularity_score.log(10.0)
}