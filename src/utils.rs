use std::{fs, path::Path};

use walkdir::WalkDir;

use crate::error::MetricError;

pub fn collect_info_string(filename: &str, dir_path: &Path) -> Option<String> {
    if filename.is_empty() {
        return None;
    }

    let info_path = Path::new(dir_path).join(filename);

    match fs::read_to_string(info_path.as_path()) {
        Ok(c) => return Some(c.trim().to_string()),
        Err(err) => log::error!("{}", MetricError::IOError(info_path, err)),
    }

    None
}

pub fn collect_info_i64(filename: &str, dir_path: &Path) -> Option<i64> {
    if let Some(c) = collect_info_string(filename, dir_path) {
        if !c.is_empty() {
            match c.parse::<i64>() {
                Ok(i) => return Some(i),
                Err(err) => log::error!("{}", err),
            }
        }
    }

    None
}

pub fn list_dir_content(
    dir_path: &Path,
    include_pattern: &str,
    exclude_pattern: &str,
) -> Vec<String> {
    let mut content = Vec::new();

    for dir_item in WalkDir::new(dir_path).into_iter().filter_map(|e| e.ok()) {
        if dir_item.file_name() == exclude_pattern {
            continue;
        }

        let dir_item_name = dir_item.file_name().to_str().unwrap_or_default();

        if include_pattern.is_empty() {
            content.push(dir_item_name.to_string());

            continue;
        }

        if dir_item_name.starts_with(include_pattern) {
            content.push(dir_item_name.to_string());
        }
    }

    content
}
