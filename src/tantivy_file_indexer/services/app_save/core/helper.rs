use std::{
    fs::{self, File},
    path::{Path, PathBuf},
};

use serde::{de::DeserializeOwned, Serialize};
use serde_json::{from_reader, to_writer_pretty};

pub fn get_path(app_path: &Path, file: &str) -> PathBuf {
    let new_path = app_path.join(file);

    if !app_path.exists() {
        fs::create_dir_all("DesktopSearch").expect("could not create DesktopSearch directory");
    }

    new_path
}
pub fn create_path(app_path: &Path, path: &str) -> PathBuf {
    let new_path = get_path(app_path, path);

    if !new_path.exists() {
        if let Some(parent) = Path::new(&new_path).parent() {
            fs::create_dir_all(parent).expect("failed to create directories");
        }
        fs::File::create(&new_path).expect("failed to create path");
    }
    new_path
}

pub fn save<T>(app_path: &Path, name: &str, data: T) -> Result<(), std::io::Error>
where
    T: Serialize,
{
    let path = get_path(app_path, format!("{}.json", name).as_str());
    let file = File::create(path)?;
    to_writer_pretty(file, &data)?;
    Ok(())
}

/**
 * Note: do not include '.json' when you pass in a value for `name`
 */
pub fn load<T>(app_path: &Path, name: &str) -> Result<T, std::io::Error>
where
    T: DeserializeOwned,
{
    let path = get_path(app_path, format!("{}.json", name).as_str());
    let file = File::open(path)?;
    let data: T = from_reader(file)?;
    Ok(data)
}
