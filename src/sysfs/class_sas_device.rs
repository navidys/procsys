use regex::Regex;
use serde::Serialize;
use std::{collections::HashMap, path::PathBuf};

use crate::{
    error::{CollectResult, MetricError},
    utils,
};

#[derive(Debug, Serialize, Clone, Default)]
pub struct SASDevice {
    /// /sys/class/sas_device/<Name>/sas_address
    pub sas_address: String,

    /// /sys/class/sas_device/<Name>/device/phy-*
    pub sas_phys: Vec<String>,

    /// /sys/class/sas_device/<Name>/device/ports-*
    pub sas_ports: Vec<String>,

    /// /sys/class/sas_device/<Name>/device/target*/*/block/*
    pub block_devices: Vec<String>,
}

impl SASDevice {
    fn new() -> Self {
        Default::default()
    }
}

/// collects sas devices information
/// # Example
/// ```
/// use procsys::sysfs::class_sas_device;
///
/// let sasdevices = class_sas_device::collect().expect("sas devices information");
/// let json_output = serde_json::to_string_pretty(&sasdevices).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<HashMap<String, SASDevice>> {
    collect_from("/sys/class/sas_device/")
}

fn collect_from(dirname: &str) -> CollectResult<HashMap<String, SASDevice>> {
    let re_phy = match Regex::new(r"^phy-[0-9:]+$") {
        Ok(r) => r,
        Err(err) => return Err(MetricError::RegexError(err)),
    };

    let re_port = match Regex::new(r"^port-[0-9:]+$") {
        Ok(r) => r,
        Err(err) => return Err(MetricError::RegexError(err)),
    };

    let re_target = match Regex::new(r"^target[0-9:]+$") {
        Ok(r) => r,
        Err(err) => return Err(MetricError::RegexError(err)),
    };

    let re_subdevice = match Regex::new(r"[0-9]+:.*") {
        Ok(r) => r,
        Err(err) => return Err(MetricError::RegexError(err)),
    };

    let mut sasdevices: HashMap<String, SASDevice> = HashMap::new();

    let sas_devices_path = PathBuf::from(dirname);
    for sdevice in utils::list_dir_content(&sas_devices_path, "", "sas_device") {
        let mut sasdevice_path = sas_devices_path.clone();
        sasdevice_path.push(&sdevice);

        let mut sasdevice = SASDevice::new();
        sasdevice.sas_address =
            utils::collect_info_string("sas_address", &sasdevice_path)?.unwrap_or_default();

        let mut sas_device_path_device = sasdevice_path.clone();
        sas_device_path_device.push("device");

        for item in utils::list_dir_content(&sas_device_path_device, "", "") {
            if re_phy.is_match(&item) {
                sasdevice.sas_phys.push(item);
            } else if re_port.is_match(&item) {
                sasdevice.sas_ports.push(item);
            } else if re_target.is_match(&item) {
                let item_target = item.clone();
                let mut item_target_path = sas_device_path_device.clone();
                item_target_path.push(&item_target);

                for sub_target in utils::list_dir_content(&item_target_path, "", &item_target) {
                    if !re_subdevice.is_match(&sub_target) {
                        continue;
                    }

                    let mut sub_target_path = item_target_path.clone();
                    sub_target_path.push(sub_target);
                    sub_target_path.push("block");

                    for block in utils::list_dir_content(&sub_target_path, "", "block") {
                        sasdevice.block_devices.push(block);
                    }
                }
            }
        }

        sasdevices.insert(sdevice, sasdevice);
    }

    Ok(sasdevices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sas_devices_information() {
        let sasdevices = collect_from("test_data/fixtures/sys/class/sas_device/")
            .expect("sas device information");

        for (sasdev_name, sasdev_info) in sasdevices {
            match sasdev_name.as_str() {
                "end_device-11:0:0" => {
                    assert_eq!(sasdev_info.sas_address, "0x5000ccab02009402");
                    assert_eq!(sasdev_info.sas_phys.is_empty(), true);
                    assert_eq!(sasdev_info.sas_ports.is_empty(), true);
                    assert_eq!(sasdev_info.block_devices[0], "sdv");
                }
                "end_device-11:0:2" => {
                    assert_eq!(sasdev_info.sas_address, "0x5000ccab02009406");
                    assert_eq!(sasdev_info.sas_phys.is_empty(), true);
                    assert_eq!(sasdev_info.sas_ports.is_empty(), true);
                    assert_eq!(sasdev_info.block_devices[0], "sdx");
                }
                "end_device-11:2" => {
                    assert_eq!(sasdev_info.sas_address, "0x5000cca0506b5f1d");
                    assert_eq!(sasdev_info.sas_phys.is_empty(), true);
                    assert_eq!(sasdev_info.sas_ports.is_empty(), true);
                    assert_eq!(sasdev_info.block_devices[0], "sdp");
                }
                "expander-11:0" => {
                    assert_eq!(sasdev_info.sas_address, "0x5000ccab0200947e");
                    assert_eq!(sasdev_info.sas_phys.len(), 12);
                    assert_eq!(sasdev_info.sas_ports.len(), 3);
                    assert_eq!(sasdev_info.block_devices.is_empty(), true);
                }
                "expander-11:1" => {
                    assert_eq!(sasdev_info.sas_address, "0x5003048001e8967f");
                    assert_eq!(sasdev_info.sas_phys.is_empty(), true);
                    assert_eq!(sasdev_info.sas_ports.is_empty(), true);
                    assert_eq!(sasdev_info.block_devices.is_empty(), true);
                }
                "end_device-11:0:1" => {
                    assert_eq!(sasdev_info.sas_address, "0x5000cca26128b1f5");
                    assert_eq!(sasdev_info.sas_phys.is_empty(), true);
                    assert_eq!(sasdev_info.sas_ports.is_empty(), true);
                    assert_eq!(sasdev_info.block_devices[0], "sdw");
                }
                _ => panic!("invalid sas device name: {}", sasdev_name),
            }
        }
    }
}
