use std::{
    collections::{HashMap, HashSet},
    error::Error,
};

use walkdir::WalkDir;

pub async fn get_all_files(
    path: &str,
    excluded_files: &HashSet<String>,
) -> Result<HashMap<String, String>, Box<dyn Error>> {
    let mut all_files: HashMap<String, String> = HashMap::new();
    for entry in WalkDir::new(path).into_iter() {
        if let Ok(entry) = entry {
            let file_name = entry.file_name().to_string_lossy().to_string();
            if entry.path().is_file() && !excluded_files.contains(&file_name) {
                all_files.insert(file_name, entry.path().to_string_lossy().to_string());
            }
        }
    }
    Ok(all_files)
}
