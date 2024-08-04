use std::path::{Path, PathBuf};

use serde::Serialize;
use walkdir::WalkDir;

use crate::{error::CollectResult, utils};

enum ThermalZoneInfo {
    ZoneType,
    Temp,
    Policy,
    Mode,
    Passive,
    Unknown,
}

impl ThermalZoneInfo {
    fn from(name: &str) -> ThermalZoneInfo {
        match name {
            "type" => ThermalZoneInfo::ZoneType,
            "temp" => ThermalZoneInfo::Temp,
            "policy" => ThermalZoneInfo::Policy,
            "mode" => ThermalZoneInfo::Mode,
            "passive" => ThermalZoneInfo::Passive,
            _ => ThermalZoneInfo::Unknown,
        }
    }
}

/// ThermalZone contains info from files in /sys/class/thermal/thermal_zoneX.
#[derive(Debug, Serialize, Clone, Default)]
pub struct ThermalZone {
    pub name: String,
    pub zone_type: String,
    pub policy: String,
    pub temp: i64,
    pub mode: Option<bool>,
    pub passive: Option<u64>,
}

impl ThermalZone {
    fn new() -> Self {
        Default::default()
    }
}

/// collects thermal devices information.
/// #Example
/// ```
/// use procsys::sysfs::class_thermal;
///
///let thermal_devices = class_thermal::collect().expect("thermal information");
///
/// for tdev in &thermal_devices {
///     println!("name: {}", tdev.name);
///     println!("temperature: {}", tdev.temp);
///     println!("type: {}", tdev.zone_type);
///     println!("policy: {}", tdev.zone_type);
/// }
///
/// // print all thermal devices information in json format
/// let json_output = serde_json::to_string_pretty(&thermal_devices).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<Vec<ThermalZone>> {
    let thermal_zone_class_path = Path::new("/sys/class/thermal/");
    collect_from(thermal_zone_class_path)
}

fn collect_from(base_path: &Path) -> CollectResult<Vec<ThermalZone>> {
    let mut thermal_zone_devices = Vec::new();

    for tdevice in utils::list_dir_content(base_path, "thermal_zone", "thermal") {
        let mut thermal_device = ThermalZone::new();
        thermal_device.name = tdevice.to_string();

        let mut tdev_path = PathBuf::from(base_path);

        tdev_path.push(&tdevice);

        for tdev_info in WalkDir::new(&tdev_path).into_iter().filter_map(|e| e.ok()) {
            let tdev_info_name = tdev_info
                .file_name()
                .to_str()
                .unwrap_or_default()
                .to_string();

            if tdev_info_name == tdevice {
                continue;
            }

            match ThermalZoneInfo::from(&tdev_info_name) {
                ThermalZoneInfo::Mode => {
                    if let Some(c) =
                        utils::collect_info_string(&tdev_info_name, tdev_path.as_path())?
                    {
                        match c.as_str() {
                            "enabled" => thermal_device.mode = Some(true),
                            "disabled" => thermal_device.mode = Some(false),
                            _ => thermal_device.mode = None,
                        }
                    }
                }
                ThermalZoneInfo::Temp => {
                    if let Some(c) = utils::collect_info_i64(&tdev_info_name, tdev_path.as_path())?
                    {
                        thermal_device.temp = c;
                    }
                }
                ThermalZoneInfo::Passive => {
                    thermal_device.passive =
                        utils::collect_info_u64(&tdev_info_name, tdev_path.as_path())?;
                }
                ThermalZoneInfo::Policy => {
                    if let Some(c) =
                        utils::collect_info_string(&tdev_info_name, tdev_path.as_path())?
                    {
                        thermal_device.policy = c;
                    }
                }
                ThermalZoneInfo::ZoneType => {
                    if let Some(c) =
                        utils::collect_info_string(&tdev_info_name, tdev_path.as_path())?
                    {
                        thermal_device.zone_type = c;
                    }
                }
                ThermalZoneInfo::Unknown => {}
            }
        }

        thermal_zone_devices.push(thermal_device);
    }

    Ok(thermal_zone_devices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thermal_devices() {
        let thermal_zone_class_path = Path::new("test_data/fixtures/sys/class/thermal/");
        let tdev =
            collect_from(thermal_zone_class_path).expect("collecting system thermal information");
        assert!(tdev.len().eq(&2));
        assert!(tdev[0].name.eq("thermal_zone0"));
        assert!(tdev[0].zone_type.eq("bcm2835_thermal"));
        assert!(tdev[0].policy.eq("step_wise"));
        assert!(tdev[0].mode.is_none());
        assert!(tdev[0].temp.eq(&49925));
        assert!(tdev[0].passive.is_none());

        assert!(tdev[1].name.eq("thermal_zone1"));
        assert!(tdev[1].zone_type.eq("acpitz"));
        assert!(tdev[1].policy.eq("step_wise"));
        assert!(tdev[1].mode.unwrap());
        assert!(tdev[1].temp.eq(&-44000));
        assert!(tdev[1].passive.unwrap().eq(&0));
    }
}
