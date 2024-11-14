use twox_hash::XxHash64;
use std::hash::{Hash, Hasher};

pub fn string_to_u64(s: &str) -> u64 {
    let mut hasher = XxHash64::default();
    s.hash(&mut hasher);
    hasher.finish()
}
