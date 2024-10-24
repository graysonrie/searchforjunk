pub fn windows_path_to_facet(file_path: &str) -> String {
    // Replace backslashes with forward slashes
    let path_with_forward_slashes = file_path.replace("\\", "/");
    
    // Prepend a `/` to make the path start at the root
    if path_with_forward_slashes.starts_with('/') {
        path_with_forward_slashes
    } else {
        format!("/{}", path_with_forward_slashes)
    }
}