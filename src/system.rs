use walkdir::WalkDir;

/// Provide timestamp as dash-separated String to be used in identifiers
///
/// # Returns
///
/// String with timestamp in format YYYY-MM-DD-HH-MM-SS
pub fn add_time_id() -> String {
    let now = chrono::Utc::now();
    let timestamp = format!("{}", now.format("%Y-%m-%d-%H-%M-%S"));
    timestamp
}

/// Find all files at the first level of a given folder and put them in a Vector
///
/// # Arguments
///
/// * `path` - Path to the folder to be searched
///
/// # Returns
///
/// Vector of Strings containing the paths to all first-level files in the given folder
pub async fn files_in_folder(path: &str) -> Vec<String> {
    let mut files = Vec::new();
    for entry in std::fs::read_dir(&path).expect(
        format!(
            "The data has not been gathered to and cannot be read from {}",
            &path
        )
        .as_str(),
    ) {
        let entry = entry.expect("Failed to get entry");
        let path = entry.path();
        if path.is_file() {
            files.push(path.to_str().unwrap().to_string());
        }
    }
    files
}

/// Recursively list all files in a folder using absolute paths
///
/// # Arguments
///
/// * `root_path` - Path to root folder
///
/// # Returns
///
/// Vector of Strings with absolute paths to files
pub async fn create_file_index(root_path: String) -> Vec<String> {
    let file_list = WalkDir::new(root_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .map(|e| e.path().to_str().unwrap().to_string())
        .collect::<Vec<String>>();
    return file_list;
}

/// Add trailing slash to a given path if it does not end with one
///
/// # Arguments
///
/// * `path` - String that should end with a slash
///
/// # Returns
///
/// String with trailing slash
pub fn check_slash(path: &str) -> String {
    if path.ends_with("/") {
        return path.to_string();
    } else {
        return format!("{}/", path);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    /// Checks that the resulting timestamp string from add_time_id() does not contain spaces
    fn test_add_time_id() {
        let result = add_time_id();
        assert!(!result.contains(" "));
    }

    #[test]
    /// Check if path given to check_slash is returned with a trailing slash if it does not have one
    fn test_check_slash() {
        let result = check_slash("path");
        assert!(result == "path/");
    }
}
