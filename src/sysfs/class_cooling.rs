use std::path::{Path, PathBuf};

use getset::Getters;
use serde::Serialize;
use walkdir::WalkDir;

use crate::utils;

enum CoolingInfo {
    CoolingType,
    MaxState,
    CurState,
    Unknown,
}

impl CoolingInfo {
    fn from(name: &str) -> CoolingInfo {
        match name {
            "type" => CoolingInfo::CoolingType,
            "max_state" => CoolingInfo::MaxState,
            "cur_state" => CoolingInfo::CurState,
            _ => CoolingInfo::Unknown,
        }
    }
}

/// Cooling contains a cooling device information from files in /sys/class/thermal/cooling_device[0-9]*
/// # Example
/// ```
/// use procsys::sysfs::class_cooling;
///
/// let cooling_devices = class_cooling::collect();
/// let json_output = serde_json::to_string_pretty(&cooling_devices).unwrap();
/// println!("{}", json_output);
///
/// ```
#[derive(Debug, Serialize, Clone, Getters)]
pub struct Cooling {
    #[getset(get = "pub")]
    name: String,

    #[getset(get = "pub")]
    cooling_type: String,

    #[getset(get = "pub")]
    max_state: i64,

    #[getset(get = "pub")]
    cur_state: i64,
}

impl Cooling {
    fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            cooling_type: String::new(),
            max_state: i64::from(0),
            cur_state: i64::from(0),
        }
    }
}

pub fn collect() -> Vec<Cooling> {
    let mut cooling_devs = Vec::new();
    let cooling_class_path = Path::new("/sys/class/thermal/");

    for cdevice in utils::list_dir_content(cooling_class_path, "cooling_device", "thermal") {
        let mut cooling_device = Cooling::new(&cdevice);
        let mut cdev_path = PathBuf::from(cooling_class_path);

        cdev_path.push(&cdevice);

        for cdev_info in WalkDir::new(&cdev_path).into_iter().filter_map(|e| e.ok()) {
            let cdev_info_name = cdev_info
                .file_name()
                .to_str()
                .unwrap_or_default()
                .to_string();

            if cdev_info_name == cdevice {
                continue;
            }

            match CoolingInfo::from(&cdev_info_name) {
                CoolingInfo::CoolingType => {
                    if let Some(c) =
                        utils::collect_info_string(&cdev_info_name, cdev_path.as_path())
                    {
                        cooling_device.cooling_type = c;
                    }
                }
                CoolingInfo::MaxState => {
                    if let Some(c) = utils::collect_info_i64(&cdev_info_name, cdev_path.as_path()) {
                        cooling_device.max_state = c;
                    }
                }
                CoolingInfo::CurState => {
                    if let Some(c) = utils::collect_info_i64(&cdev_info_name, cdev_path.as_path()) {
                        cooling_device.cur_state = c;
                    }
                }
                CoolingInfo::Unknown => {}
            }
        }

        cooling_devs.push(cooling_device);
    }

    cooling_devs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cooling_devices() {
        let cdev = collect();
        assert!(!cdev.is_empty())
    }
}
