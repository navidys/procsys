use std::path::{Path, PathBuf};

use serde::Serialize;
use walkdir::WalkDir;

use crate::{error::CollectResult, utils};

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
#[derive(Debug, Serialize, Clone, Default)]
pub struct Cooling {
    pub name: String,
    pub cooling_type: String,
    pub max_state: i64,
    pub cur_state: i64,
}

impl Cooling {
    fn new() -> Self {
        Default::default()
    }
}

/// collects cooling devices information
/// # Example
/// ```
/// use procsys::sysfs::class_cooling;
///
/// let cooling_devices = class_cooling::collect().expect("cooling information");
/// let json_output = serde_json::to_string_pretty(&cooling_devices).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<Vec<Cooling>> {
    let cooling_class_path = Path::new("/sys/class/thermal/");
    collect_from(cooling_class_path)
}

fn collect_from(base_path: &Path) -> CollectResult<Vec<Cooling>> {
    let mut cooling_devs = Vec::new();

    for cdevice in utils::list_dir_content(base_path, "cooling_device", "thermal") {
        let mut cooling_device = Cooling::new();
        cooling_device.name = cdevice.to_string();

        let mut cdev_path = PathBuf::from(base_path);

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
                        utils::collect_info_string(&cdev_info_name, cdev_path.as_path())?
                    {
                        cooling_device.cooling_type = c;
                    }
                }
                CoolingInfo::MaxState => {
                    if let Some(c) = utils::collect_info_i64(&cdev_info_name, cdev_path.as_path())?
                    {
                        cooling_device.max_state = c;
                    }
                }
                CoolingInfo::CurState => {
                    if let Some(c) = utils::collect_info_i64(&cdev_info_name, cdev_path.as_path())?
                    {
                        cooling_device.cur_state = c;
                    }
                }
                CoolingInfo::Unknown => {}
            }
        }

        cooling_devs.push(cooling_device);
    }

    Ok(cooling_devs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cooling_devices() {
        let cooling_class_path = Path::new("test_data/fixtures/sys/class/thermal/");
        let cdev = collect_from(cooling_class_path).expect("collecting cooling information");
        assert!(cdev.len().eq(&2));
        assert!(cdev[0].name.eq("cooling_device0"));
        assert!(cdev[0].cur_state.eq(&0));
        assert!(cdev[0].max_state.eq(&50));
        assert!(cdev[0].cooling_type.eq("Processor"));

        assert!(cdev[1].name.eq("cooling_device1"));
        assert!(cdev[1].cur_state.eq(&-1));
        assert!(cdev[1].max_state.eq(&27));
        assert!(cdev[1].cooling_type.eq("intel_powerclamp"));
    }
}
