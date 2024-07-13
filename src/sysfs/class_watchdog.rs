use std::path::{Path, PathBuf};

use serde::Serialize;
use walkdir::WalkDir;

use crate::utils;

enum WatchdogInfo {
    BootStatus,
    Options,
    FwVersion,
    Identity,
    Nowayout,
    State,
    Status,
    Timeleft,
    Timeout,
    MinTimeout,
    MaxTimeout,
    Pretimeout,
    PretimeoutGovernor,
    AccessCs0,
    Unknown,
}

impl WatchdogInfo {
    fn from(name: &str) -> WatchdogInfo {
        match name {
            "bootstatus" => WatchdogInfo::BootStatus,
            "options" => WatchdogInfo::Options,
            "fw_version" => WatchdogInfo::FwVersion,
            "identity" => WatchdogInfo::Identity,
            "nowayout" => WatchdogInfo::Nowayout,
            "state" => WatchdogInfo::State,
            "status" => WatchdogInfo::Status,
            "timeleft" => WatchdogInfo::Timeleft,
            "timeout" => WatchdogInfo::Timeout,
            "min_timeout" => WatchdogInfo::MinTimeout,
            "max_timeout" => WatchdogInfo::MaxTimeout,
            "pretimeout" => WatchdogInfo::Pretimeout,
            "pretimeout_governor" => WatchdogInfo::PretimeoutGovernor,
            "access_cs0" => WatchdogInfo::AccessCs0,
            _ => WatchdogInfo::Unknown,
        }
    }
}

/// Watchdog contains a watchdog device stat information from files in /sys/class/watchdog
#[derive(Debug, Serialize, Clone)]
pub struct Watchdog {
    pub name: String,
    pub boot_status: Option<i64>,
    pub options: Option<String>,
    pub fw_version: Option<i64>,
    pub identity: Option<String>,
    pub nowayout: Option<i64>,
    pub state: Option<String>,
    pub status: Option<String>,
    pub timeleft: Option<i64>,
    pub timeout: Option<i64>,
    pub min_timeout: Option<i64>,
    pub max_timeout: Option<i64>,
    pub pretimeout: Option<i64>,
    pub pretimeout_governor: Option<String>,
    pub access_cs0: Option<i64>,
}

impl Watchdog {
    fn new(name: String) -> Self {
        Self {
            name,
            boot_status: None,
            options: None,
            fw_version: None,
            identity: None,
            nowayout: None,
            state: None,
            status: None,
            timeleft: None,
            timeout: None,
            min_timeout: None,
            max_timeout: None,
            pretimeout: None,
            pretimeout_governor: None,
            access_cs0: None,
        }
    }
}

/// collects watchdog devices information.
/// # Example
/// ```
/// use procsys::sysfs::class_watchdog;
///
/// let watchdog_devices = class_watchdog::collect();
///
/// for wdev in &watchdog_devices {
///     println!("name: {}", wdev.name);
///     println!("timeout: {}", wdev.timeout.unwrap_or_default());
///     println!("min_timeout: {}", wdev.min_timeout.unwrap_or_default());
///     println!("max_timeout: {}", wdev.max_timeout.unwrap_or_default());
/// }
///
/// // print all watchdog devices information in json format
/// let json_output = serde_json::to_string_pretty(&watchdog_devices).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> Vec<Watchdog> {
    let mut devices = Vec::new();
    let watchdog_class_path = Path::new("/sys/class/watchdog/");
    for device in utils::list_dir_content(watchdog_class_path, "", "watchdog") {
        let mut watchdog_dev = Watchdog::new(device.to_owned());

        let mut wdev_path = PathBuf::from(watchdog_class_path);
        wdev_path.push(&device);

        for dev_info in WalkDir::new(&wdev_path).into_iter().filter_map(|e| e.ok()) {
            if *dev_info
                .file_name()
                .to_str()
                .unwrap_or_default()
                .to_string()
                == device
            {
                continue;
            }

            let wdev_filename = dev_info
                .file_name()
                .to_str()
                .unwrap_or_default()
                .to_string();

            match WatchdogInfo::from(wdev_filename.as_str()) {
                WatchdogInfo::BootStatus => {
                    watchdog_dev.boot_status =
                        utils::collect_info_i64(&wdev_filename, wdev_path.as_path());
                }
                WatchdogInfo::Options => {
                    watchdog_dev.options =
                        utils::collect_info_string(&wdev_filename, wdev_path.as_path());
                }
                WatchdogInfo::FwVersion => {
                    watchdog_dev.fw_version =
                        utils::collect_info_i64(&wdev_filename, wdev_path.as_path());
                }
                WatchdogInfo::Identity => {
                    watchdog_dev.identity =
                        utils::collect_info_string(&wdev_filename, wdev_path.as_path());
                }
                WatchdogInfo::Nowayout => {
                    watchdog_dev.nowayout =
                        utils::collect_info_i64(&wdev_filename, wdev_path.as_path());
                }
                WatchdogInfo::State => {
                    watchdog_dev.state =
                        utils::collect_info_string(&wdev_filename, wdev_path.as_path());
                }
                WatchdogInfo::Status => {
                    watchdog_dev.status =
                        utils::collect_info_string(&wdev_filename, wdev_path.as_path());
                }
                WatchdogInfo::Timeleft => {
                    watchdog_dev.timeleft =
                        utils::collect_info_i64(&wdev_filename, wdev_path.as_path());
                }
                WatchdogInfo::Timeout => {
                    watchdog_dev.timeout =
                        utils::collect_info_i64(&wdev_filename, wdev_path.as_path());
                }
                WatchdogInfo::MinTimeout => {
                    watchdog_dev.min_timeout =
                        utils::collect_info_i64(&wdev_filename, wdev_path.as_path());
                }
                WatchdogInfo::MaxTimeout => {
                    watchdog_dev.max_timeout =
                        utils::collect_info_i64(&wdev_filename, wdev_path.as_path());
                }
                WatchdogInfo::Pretimeout => {
                    watchdog_dev.pretimeout =
                        utils::collect_info_i64(&wdev_filename, wdev_path.as_path());
                }
                WatchdogInfo::PretimeoutGovernor => {
                    watchdog_dev.pretimeout_governor =
                        utils::collect_info_string(&wdev_filename, wdev_path.as_path());
                }
                WatchdogInfo::AccessCs0 => {
                    watchdog_dev.access_cs0 =
                        utils::collect_info_i64(&wdev_filename, wdev_path.as_path());
                }
                WatchdogInfo::Unknown => {}
            }
        }

        devices.push(watchdog_dev);
    }

    devices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn watchdog_devices() {
        let wdev = collect();
        assert!(!wdev.is_empty());
    }
}
