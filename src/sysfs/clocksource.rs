use std::{fs, path::Path};

use getset::Getters;
use serde::Serialize;
use walkdir::WalkDir;

use crate::error::MetricError;

enum ClocksourceInfo {
    AvailableClockSource,
    CurrentClockSource,
}

impl ClocksourceInfo {
    fn into_string(self) -> String {
        let info_str = match self {
            ClocksourceInfo::AvailableClockSource => "available_clocksource",
            ClocksourceInfo::CurrentClockSource => "current_clocksource",
        };

        info_str.to_string()
    }
}

/// Clocksource contains a clocksource information.
/// # Example
/// ```
/// use procsys::sysfs::clocksource;
///
/// let clocksources = clocksource::collect();
///
/// for clock_src in clocksources {
///     println!("name: {}", clock_src.name());
///     println!("available clocksource: {:?}", clock_src.available_clocksource());
///     println!("current clocksource: {}", clock_src.current_clocksource());
/// }
///
/// ```
#[derive(Debug, Serialize, Clone, Getters)]
pub struct Clocksource {
    #[getset(get = "pub")]
    name: String,

    #[getset(get = "pub")]
    available_clocksource: Vec<String>,

    #[getset(get = "pub")]
    current_clocksource: String,
}

impl Clocksource {
    fn new(
        name: String,
        available_clocksource: Vec<String>,
        current_clocksource: String,
    ) -> Clocksource {
        Self {
            name,
            available_clocksource,
            current_clocksource,
        }
    }
}

pub fn collect() -> Vec<Clocksource> {
    let mut clock_sources = Vec::new();
    let clock_source_path = Path::new("/sys/devices/system/clocksource");

    for clock_dev in WalkDir::new(clock_source_path)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if clock_dev.file_name() == "clocksource" {
            continue;
        }

        let clocksource_name = clock_dev
            .file_name()
            .to_str()
            .unwrap_or_default()
            .trim()
            .to_string();

        if clocksource_name.is_empty() {
            continue;
        }

        if !clocksource_name.starts_with("clocksource") {
            continue;
        }

        let current_clocksource = collect_clocksource_info(
            ClocksourceInfo::CurrentClockSource,
            &clocksource_name,
            clock_source_path,
        )
        .unwrap_or_default();

        let available_clocksource = collect_clocksource_info(
            ClocksourceInfo::AvailableClockSource,
            &clocksource_name,
            clock_source_path,
        )
        .unwrap_or_default()
        .split(' ')
        .map(|v| v.to_string())
        .collect::<Vec<String>>();

        clock_sources.push(Clocksource::new(
            clocksource_name,
            available_clocksource,
            current_clocksource,
        ));
    }

    clock_sources
}

fn collect_clocksource_info(
    info: ClocksourceInfo,
    name: &str,
    class_path: &Path,
) -> Option<String> {
    let info_str = info.into_string();
    let info_path = Path::new(class_path).join(name).join(info_str);

    match fs::read_to_string(info_path.as_path()) {
        Ok(content) => return Some(content.trim().to_string()),
        Err(err) => log::error!("{}", MetricError::IOError(info_path, err)),
    }

    None
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn clocksource_collect() {
        let clock_sources = collect();

        assert!(!clock_sources.is_empty());

        for clock_src in clock_sources {
            assert!(!clock_src.name().is_empty());
            assert!(!clock_src.available_clocksource().is_empty());
            assert!(!clock_src.current_clocksource().is_empty());
        }
    }
}
