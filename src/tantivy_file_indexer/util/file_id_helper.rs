

pub fn get_file_id(path: std::path::PathBuf) -> Result<String, String> {
   refind::get_id(path).map_err(|x|x.to_string())
}