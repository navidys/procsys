use std::path::{Path, PathBuf};

use serde::Serialize;
use walkdir::WalkDir;

use crate::{error::CollectResult, utils};

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
#[derive(Debug, Serialize, Clone, Default)]
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
    fn new() -> Self {
        Default::default()
    }
}

/// collects watchdog devices information.
/// # Example
/// ```
/// use procsys::sysfs::class_watchdog;
///
/// let watchdog_devices = class_watchdog::collect().expect("watchdog information");
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
pub fn collect() -> CollectResult<Vec<Watchdog>> {
    let watchdog_class_path = Path::new("/sys/class/watchdog/");
    collect_from(watchdog_class_path)
}

fn collect_from(base_path: &Path) -> CollectResult<Vec<Watchdog>> {
    let mut devices = Vec::new();

    for device in utils::list_dir_content(base_path, "", "watchdog") {
        let mut watchdog_dev = Watchdog::new();
        watchdog_dev.name = device.to_string();

        let mut wdev_path = PathBuf::from(base_path);
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
                        utils::collect_info_i64(&wdev_filename, wdev_path.as_path())?;
                }
                WatchdogInfo::Options => {
                    watchdog_dev.options =
                        utils::collect_info_string(&wdev_filename, wdev_path.as_path())?;
                }
                WatchdogInfo::FwVersion => {
                    watchdog_dev.fw_version =
                        utils::collect_info_i64(&wdev_filename, wdev_path.as_path())?;
                }
                WatchdogInfo::Identity => {
                    watchdog_dev.identity =
                        utils::collect_info_string(&wdev_filename, wdev_path.as_path())?;
                }
                WatchdogInfo::Nowayout => {
                    watchdog_dev.nowayout =
                        utils::collect_info_i64(&wdev_filename, wdev_path.as_path())?;
                }
                WatchdogInfo::State => {
                    watchdog_dev.state =
                        utils::collect_info_string(&wdev_filename, wdev_path.as_path())?;
                }
                WatchdogInfo::Status => {
                    watchdog_dev.status =
                        utils::collect_info_string(&wdev_filename, wdev_path.as_path())?;
                }
                WatchdogInfo::Timeleft => {
                    watchdog_dev.timeleft =
                        utils::collect_info_i64(&wdev_filename, wdev_path.as_path())?;
                }
                WatchdogInfo::Timeout => {
                    watchdog_dev.timeout =
                        utils::collect_info_i64(&wdev_filename, wdev_path.as_path())?;
                }
                WatchdogInfo::MinTimeout => {
                    watchdog_dev.min_timeout =
                        utils::collect_info_i64(&wdev_filename, wdev_path.as_path())?;
                }
                WatchdogInfo::MaxTimeout => {
                    watchdog_dev.max_timeout =
                        utils::collect_info_i64(&wdev_filename, wdev_path.as_path())?;
                }
                WatchdogInfo::Pretimeout => {
                    watchdog_dev.pretimeout =
                        utils::collect_info_i64(&wdev_filename, wdev_path.as_path())?;
                }
                WatchdogInfo::PretimeoutGovernor => {
                    watchdog_dev.pretimeout_governor =
                        utils::collect_info_string(&wdev_filename, wdev_path.as_path())?;
                }
                WatchdogInfo::AccessCs0 => {
                    watchdog_dev.access_cs0 =
                        utils::collect_info_i64(&wdev_filename, wdev_path.as_path())?;
                }
                WatchdogInfo::Unknown => {}
            }
        }

        devices.push(watchdog_dev);
    }

    Ok(devices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn watchdog_devices() {
        let watchdog_class_path = Path::new("test_data/fixtures/sys/class/watchdog/");
        let wdev =
            collect_from(watchdog_class_path).expect("collecting system watchdog information");
        assert!(wdev.len().eq(&2));

        assert!(wdev[0].name.eq("watchdog0"));
        assert!(wdev[0].boot_status.unwrap().eq(&1));
        assert!(wdev[0].options.clone().unwrap().eq("0x8380"));
        assert!(wdev[0].fw_version.unwrap().eq(&2));
        assert!(wdev[0].identity.clone().unwrap().eq("Software Watchdog"));
        assert!(wdev[0].nowayout.unwrap().eq(&0));
        assert!(wdev[0].state.clone().unwrap().eq("active"));
        assert!(wdev[0].status.clone().unwrap().eq("0x8000"));
        assert!(wdev[0].timeleft.unwrap().eq(&300));
        assert!(wdev[0].timeout.unwrap().eq(&60));
        assert!(wdev[0].min_timeout.unwrap().eq(&120));
        assert!(wdev[0].max_timeout.unwrap().eq(&65535));
        assert!(wdev[0].pretimeout.unwrap().eq(&120));
        assert!(wdev[0].pretimeout_governor.clone().unwrap().eq("noop"));
        assert!(wdev[0].access_cs0.unwrap().eq(&0));

        assert!(wdev[1].name.eq("watchdog1"));
        assert!(wdev[1].boot_status.is_none());
        assert!(wdev[1].options.is_none());
        assert!(wdev[1].fw_version.is_none());
        assert!(wdev[1].identity.is_none());
        assert!(wdev[1].nowayout.is_none());
        assert!(wdev[1].state.is_none());
        assert!(wdev[1].status.is_none());
        assert!(wdev[1].timeleft.is_none());
        assert!(wdev[1].timeout.is_none());
        assert!(wdev[1].min_timeout.is_none());
        assert!(wdev[1].max_timeout.is_none());
        assert!(wdev[1].pretimeout.is_none());
        assert!(wdev[1].pretimeout_governor.is_none());
        assert!(wdev[1].access_cs0.is_none());
    }
}
