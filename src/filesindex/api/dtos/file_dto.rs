use std::time::SystemTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct FileDTO{
    pub name:String,
    pub date_created:SystemTime
}