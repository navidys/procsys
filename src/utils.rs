use std::{fs, path::Path};

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
