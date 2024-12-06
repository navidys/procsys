use regex::Regex;
use serde::Serialize;
use std::{collections::HashMap, path::PathBuf};

use crate::{
    error::{CollectResult, MetricError},
    utils,
};

#[derive(Debug, Serialize, Clone, Default)]
pub struct SASPort {
    /// /sys/class/sas_device/<Name>/device/phy-*
    pub sas_phys: Vec<String>,

    /// /sys/class/sas_port/<Name>/device/expander-*
    pub expanders: Vec<String>,

    /// /sys/class/sas_port/<Name>/device/end_device-*
    pub end_devices: Vec<String>,
}

impl SASPort {
    fn new() -> Self {
        Default::default()
    }
}

/// collects sas ports information
/// # Example
/// ```
/// use procsys::sysfs::class_sas_port;
///
/// let sasports = class_sas_port::collect().expect("sas ports information");
/// let json_output = serde_json::to_string_pretty(&sasports).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<HashMap<String, SASPort>> {
    collect_from("/sys/class/sas_port/")
}

fn collect_from(dirname: &str) -> CollectResult<HashMap<String, SASPort>> {
    let mut sasports: HashMap<String, SASPort> = HashMap::new();
    let sas_port_path = PathBuf::from(dirname);

    let re_phy = match Regex::new(r"^phy-[0-9:]+$") {
        Ok(r) => r,
        Err(err) => return Err(MetricError::RegexError(err)),
    };

    let re_expanders = match Regex::new(r"expander-[0-9:]+$") {
        Ok(r) => r,
        Err(err) => return Err(MetricError::RegexError(err)),
    };

    let re_end_devices = match Regex::new(r"^end_device-[0-9:]+$") {
        Ok(r) => r,
        Err(err) => return Err(MetricError::RegexError(err)),
    };

    for sas_port_item in utils::list_dir_content(&sas_port_path, "", "sas_port") {
        let mut sas_port = SASPort::new();
        let mut sas_port_item_path = sas_port_path.clone();
        sas_port_item_path.push(&sas_port_item);
        sas_port_item_path.push("device");

        for sas_port_device_item in utils::list_dir_content(&sas_port_item_path, "", "device") {
            if re_phy.is_match(&sas_port_device_item) {
                sas_port.sas_phys.push(sas_port_device_item)
            } else if re_expanders.is_match(&sas_port_device_item) {
                sas_port.expanders.push(sas_port_device_item)
            } else if re_end_devices.is_match(&sas_port_device_item) {
                sas_port.end_devices.push(sas_port_device_item)
            }
        }

        sasports.insert(sas_port_item, sas_port);
    }

    Ok(sasports)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sas_port_information() {
        let sasports =
            collect_from("test_data/fixtures/sys/class/sas_port/").expect("sas ports information");

        for (sasport_name, sasport_info) in sasports {
            match sasport_name.as_str() {
                "port-11:0:2" => {
                    assert_eq!(sasport_info.sas_phys, ["phy-11:0:6"]);
                    assert_eq!(sasport_info.expanders.is_empty(), true);
                    assert_eq!(sasport_info.end_devices, ["end_device-11:0:2"]);
                }
                "port-11:0:1" => {
                    assert_eq!(sasport_info.sas_phys, ["phy-11:0:4"]);
                    assert_eq!(sasport_info.expanders.is_empty(), true);
                    assert_eq!(sasport_info.end_devices, ["end_device-11:0:1"]);
                }
                "port-11:0:0" => {
                    assert_eq!(sasport_info.sas_phys, ["phy-11:0:2"]);
                    assert_eq!(sasport_info.expanders.is_empty(), true);
                    assert_eq!(sasport_info.end_devices, ["end_device-11:0:0"]);
                }
                "port-11:0" => {
                    assert_eq!(sasport_info.sas_phys.len(), 4);
                    assert_eq!(sasport_info.expanders, ["expander-11:0"]);
                    assert_eq!(sasport_info.end_devices.is_empty(), true);
                }
                "port-11:1" => {
                    assert_eq!(sasport_info.sas_phys.len(), 4);
                    assert_eq!(sasport_info.expanders, ["expander-11:1"]);
                    assert_eq!(sasport_info.end_devices.is_empty(), true);
                }
                _ => panic!("invalid sas port name: {}", sasport_name),
            }
        }
    }
}
