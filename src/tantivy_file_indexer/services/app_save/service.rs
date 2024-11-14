use std::path::{Path, PathBuf};

use dirs::data_dir;
use serde::{de::DeserializeOwned, Serialize};

use crate::tantivy_file_indexer::services::app_save::core::helper::{
    create_path, get_path, load, save,
};

pub enum AppSavePath {
    AppData,
    Other(String),
}
pub struct AppSaveService {
    pub save_dir: PathBuf,
}

impl AppSaveService {
    pub fn new(save_dir: AppSavePath, app_name: &str) -> Self {
        Self {
            save_dir: AppSaveService::get_save_path(save_dir, app_name),
        }
    }
    pub fn get_path(&self, path: &str) -> PathBuf {
        get_path(&self.save_dir, path)
    }
    pub fn create_path(&self, path: &str) -> PathBuf {
        create_path(&self.save_dir, path)
    }

    pub fn save<T>(&self, name: &str, data: T) -> Result<(), std::io::Error>
    where
        T: Serialize,
    {
        save(&self.save_dir, name, data)
    }

    /**
     * Note: do not include '.json' when you pass in a value for `name`
     */
    pub fn load<T>(&self, name: &str) -> Result<T, std::io::Error>
    where
        T: DeserializeOwned,
    {
        load(&self.save_dir, name)
    }

    fn get_save_path(save_dir: AppSavePath, app_name: &str) -> PathBuf {
        let save_path = match save_dir {
            AppSavePath::AppData => data_dir().expect("Could not find AppData directory"),
            AppSavePath::Other(path) => Path::new(&path).to_path_buf(),
        };
        save_path.join(app_name)
    }
}
