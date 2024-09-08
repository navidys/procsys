use std::{
    fs::{self, File},
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
};

use walkdir::WalkDir;

use crate::error::{CollectResult, MetricError};

pub fn collect_info_string(filename: &str, dir_path: &Path) -> CollectResult<Option<String>> {
    if filename.is_empty() {
        return Ok(None);
    }

    let info_path = Path::new(dir_path).join(filename);

    if !info_path.exists() {
        return Ok(None);
    }

    match fs::read_to_string(info_path.as_path()) {
        Ok(c) => {
            let value = c.trim().to_string();
            if value.is_empty() {
                return Ok(None);
            }

            Ok(Some(c.trim().to_string()))
        }
        Err(err) => Err(MetricError::IOError(info_path, err)),
    }
}

pub fn collect_info_i64(filename: &str, dir_path: &Path) -> CollectResult<Option<i64>> {
    if let Some(c) = collect_info_string(filename, dir_path)? {
        if !c.is_empty() {
            match c.parse::<i64>() {
                Ok(i) => return Ok(Some(i)),
                Err(err) => return Err(MetricError::ParseIntError(filename.to_string(), err)),
            }
        }
    }

    Ok(None)
}

pub fn collect_info_u64(filename: &str, dir_path: &Path) -> CollectResult<Option<u64>> {
    if let Some(c) = collect_info_string(filename, dir_path)? {
        if !c.is_empty() {
            match c.parse::<u64>() {
                Ok(i) => return Ok(Some(i)),
                Err(err) => return Err(MetricError::ParseIntError(filename.to_string(), err)),
            }
        }
    }

    Ok(None)
}

pub fn list_dir_content(
    dir_path: &Path,
    include_pattern: &str,
    exclude_pattern: &str,
) -> Vec<String> {
    let mut content = Vec::new();

    for dir_item in WalkDir::new(dir_path)
        .max_depth(1)
        .into_iter()
        .filter_map(|e| e.ok())
    {
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

pub fn read_file_lines(filename: &str) -> CollectResult<Vec<String>> {
    let mut result = Vec::new();

    match File::open(filename) {
        Ok(file) => {
            let reader = BufReader::new(file);
            for line_result in reader.lines() {
                match line_result {
                    Ok(line) => result.push(line),
                    Err(err) => return Err(MetricError::IOError(PathBuf::from(filename), err)),
                }
            }
        }
        Err(err) => return Err(MetricError::IOError(PathBuf::from(filename), err)),
    }

    Ok(result)
}

pub fn convert_to_bytes(num: u64, unit: &str) -> CollectResult<Option<u64>> {
    match unit {
        "B" => Ok(Some(num)),
        "KiB" | "kiB" | "kB" | "KB" => Ok(Some(num * 1024)),
        "MiB" | "miB" | "MB" | "mB" => Ok(Some(num * 1024 * 1024)),
        "GiB" | "giB" | "GB" | "gB" => Ok(Some(num * 1024 * 1024 * 1024)),
        _ => Err(MetricError::ByteConvertError(unit.to_string())),
    }
}

pub fn convert_str_to_i64(value: &str) -> CollectResult<i64> {
    match value.parse::<i64>() {
        Ok(c) => Ok(c),
        Err(err) => Err(MetricError::ParseIntError(value.to_string(), err)),
    }
}

pub fn convert_str_to_u64(value: &str) -> CollectResult<u64> {
    match value.parse::<u64>() {
        Ok(c) => Ok(c),
        Err(err) => Err(MetricError::ParseIntError(value.to_string(), err)),
    }
}
