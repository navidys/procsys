use std::{fs, path::Path};

use serde::Serialize;
use walkdir::WalkDir;

use crate::error::{CollectResult, MetricError};

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

/// Clocksource contains a clocksource information read from '/sys/devices/system/clocksource'
#[derive(Debug, Serialize, Clone)]
pub struct Clocksource {
    pub name: String,
    pub available_clocksource: Vec<String>,
    pub current_clocksource: String,
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

/// collects clocksources information
/// # Example
/// ```
/// use procsys::sysfs::clocksource;
///
/// let clocksources = clocksource::collect().expect("clocksource information");
///
/// for clock_src in clocksources {
///     println!("name: {}", clock_src.name);
///     println!("available clocksource: {:?}", clock_src.available_clocksource);
///     println!("current clocksource: {}", clock_src.current_clocksource);
/// }
///
/// ```
pub fn collect() -> CollectResult<Vec<Clocksource>> {
    let clock_source_path = Path::new("/sys/devices/system/clocksource");
    collect_from(clock_source_path)
}

pub fn collect_from(base_path: &Path) -> CollectResult<Vec<Clocksource>> {
    let mut clock_sources = Vec::new();

    for clock_dev in WalkDir::new(base_path).into_iter().filter_map(|e| e.ok()) {
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
            base_path,
        )?
        .unwrap_or_default();

        let available_clocksource = collect_clocksource_info(
            ClocksourceInfo::AvailableClockSource,
            &clocksource_name,
            base_path,
        )?
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

    Ok(clock_sources)
}

fn collect_clocksource_info(
    info: ClocksourceInfo,
    name: &str,
    class_path: &Path,
) -> CollectResult<Option<String>> {
    let info_str = info.into_string();
    let info_path = Path::new(class_path).join(name).join(info_str);

    match fs::read_to_string(info_path.as_path()) {
        Ok(content) => Ok(Some(content.trim().to_string())),
        Err(err) => Err(MetricError::IOError(info_path, err)),
    }
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn clocksource_collect() {
        let clock_source_path = Path::new("test_data/fixtures/sys/devices/system/clocksource");
        let clock_sources =
            collect_from(clock_source_path).expect("collecting clock source information");

        assert!(clock_sources.len().eq(&1));
        assert!(clock_sources[0].name.eq("clocksource0"));
        assert!(clock_sources[0]
            .available_clocksource
            .eq(&["tsc", "hpet", "acpi_pm"]));
        assert!(clock_sources[0].current_clocksource.eq("tsc"));
    }
}
