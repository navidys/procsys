use regex::Regex;
use serde::Serialize;
use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::{
    error::{CollectResult, MetricError},
    utils,
};

#[derive(Debug, Serialize, Clone, Default)]
pub struct SASPhy {
    /// /sys/class/sas_phy/<Name>/sas_address
    pub sas_address: String,

    /// /sys/class/sas_phy/<Name>/device/port
    pub sas_port: String,

    /// /sys/class/sas_phy/<Name>/device_type
    pub device_type: String,

    /// /sys/class/sas_phy/<Name>/initiator_port_protocols
    pub initiator_port_protocols: Vec<String>,

    /// /sys/class/sas_phy/<Name>/invalid_dword_count
    pub invalid_dword_count: i64,

    /// /sys/class/sas_phy/<Name>/loss_of_dword_sync_count
    pub loss_of_dword_sync_count: i64,

    /// /sys/class/sas_phy/<Name>/maximum_linkrate
    pub maximum_linkrate: f64,

    /// /sys/class/sas_phy/<Name>/maximum_linkrate_hw
    pub maximum_linkrate_hw: f64,

    /// /sys/class/sas_phy/<Name>/minimum_linkrate
    pub minimum_linkrate: f64,

    /// /sys/class/sas_phy/<Name>/minimum_linkrate_hw
    pub minimum_linkrate_hw: f64,

    /// /sys/class/sas_phy/<Name>/negotiated_linkrate
    pub negotiated_linkrate: f64,

    /// /sys/class/sas_phy/<Name>/phy_identifier
    pub phy_identifier: String,

    /// /sys/class/sas_phy/<Name>/phy_reset_problem_count
    pub phy_reset_problem_count: i64,

    /// /sys/class/sas_phy/<Name>/running_disparity_error_count
    pub running_disparity_error_count: i64,

    /// /sys/class/sas_phy/<Name>/target_port_protocols
    pub target_port_protocols: Vec<String>,
}

impl SASPhy {
    fn new() -> Self {
        Default::default()
    }
}

/// collects sas phys information
/// # Example
/// ```
/// use procsys::sysfs::class_sas_phy;
///
/// let sas_phys = class_sas_phy::collect().expect("sas phys information");
/// let json_output = serde_json::to_string_pretty(&sas_phys).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<HashMap<String, SASPhy>> {
    collect_from("/sys/class/sas_phy/")
}

fn collect_from(dirname: &str) -> CollectResult<HashMap<String, SASPhy>> {
    let mut sas_phys: HashMap<String, SASPhy> = HashMap::new();
    let sas_phy_path = PathBuf::from(dirname);

    let re_port = match Regex::new(r"^port-[0-9:]+$") {
        Ok(r) => r,
        Err(err) => return Err(MetricError::RegexError(err)),
    };

    for sas_phy_item in utils::list_dir_content(&sas_phy_path, "", "sas_phy") {
        let mut sas_phy = SASPhy::new();
        let mut sas_phy_item_path = sas_phy_path.clone();
        sas_phy_item_path.push(&sas_phy_item);

        for phy_info in utils::list_dir_content(&sas_phy_item_path, "", &sas_phy_item) {
            match phy_info.as_str() {
                "sas_address" => {
                    sas_phy.sas_address =
                        utils::collect_info_string("sas_address", &sas_phy_item_path)?
                            .unwrap_or_default();
                }
                "device" => {
                    let mut sas_phy_ports_path = sas_phy_item_path.clone();
                    sas_phy_ports_path.push("device");
                    sas_phy_ports_path.push("port");
                    sas_phy.sas_port = match fs::read_link(&sas_phy_ports_path) {
                        Ok(p) => {
                            let port = p
                                .file_name()
                                .unwrap_or_default()
                                .to_str()
                                .unwrap_or_default();

                            if re_port.is_match(port) {
                                port.to_string()
                            } else {
                                "".to_string()
                            }
                        }
                        Err(err) => return Err(MetricError::IOError(sas_phy_ports_path, err)),
                    };
                }
                "device_type" => {
                    sas_phy.device_type =
                        utils::collect_info_string("device_type", &sas_phy_item_path)?
                            .unwrap_or_default();
                }
                "initiator_port_protocols" => {
                    let port_protocols =
                        utils::collect_info_string("initiator_port_protocols", &sas_phy_item_path)?
                            .unwrap_or_default();
                    let initiator_port_protocols: Vec<String> = port_protocols
                        .trim()
                        .replace(", ", ",")
                        .split(',')
                        .filter(|s| !s.is_empty())
                        .map(str::to_string)
                        .collect();

                    sas_phy.initiator_port_protocols = initiator_port_protocols
                }
                "invalid_dword_count" => {
                    sas_phy.invalid_dword_count =
                        utils::collect_info_i64("invalid_dword_count", &sas_phy_item_path)?
                            .unwrap_or_default();
                }
                "loss_of_dword_sync_count" => {
                    sas_phy.loss_of_dword_sync_count =
                        utils::collect_info_i64("loss_of_dword_sync_count", &sas_phy_item_path)?
                            .unwrap_or_default();
                }
                "maximum_linkrate" => {
                    let linkrate = get_linkrate("maximum_linkrate", &sas_phy_item_path)?;
                    sas_phy.maximum_linkrate = linkrate;
                }
                "maximum_linkrate_hw" => {
                    let linkrate = get_linkrate("maximum_linkrate_hw", &sas_phy_item_path)?;
                    sas_phy.maximum_linkrate_hw = linkrate;
                }
                "minimum_linkrate" => {
                    let linkrate = get_linkrate("minimum_linkrate", &sas_phy_item_path)?;
                    sas_phy.minimum_linkrate = linkrate;
                }
                "minimum_linkrate_hw" => {
                    let linkrate = get_linkrate("minimum_linkrate_hw", &sas_phy_item_path)?;
                    sas_phy.minimum_linkrate_hw = linkrate;
                }
                "negotiated_linkrate" => {
                    let linkrate = get_linkrate("negotiated_linkrate", &sas_phy_item_path)?;
                    sas_phy.negotiated_linkrate = linkrate;
                }
                "phy_identifier" => {
                    sas_phy.phy_identifier =
                        utils::collect_info_string("phy_identifier", &sas_phy_item_path)?
                            .unwrap_or_default();
                }
                "phy_reset_problem_count" => {
                    sas_phy.phy_reset_problem_count =
                        utils::collect_info_i64("phy_reset_problem_count", &sas_phy_item_path)?
                            .unwrap_or_default();
                }
                "running_disparity_error_count" => {
                    sas_phy.running_disparity_error_count = utils::collect_info_i64(
                        "running_disparity_error_count",
                        &sas_phy_item_path,
                    )?
                    .unwrap_or_default();
                }
                "target_port_protocols" => {
                    let port_protocols =
                        utils::collect_info_string("target_port_protocols", &sas_phy_item_path)?
                            .unwrap_or_default();
                    let target_port_protocols: Vec<String> = port_protocols
                        .trim()
                        .replace(", ", ",")
                        .split(',')
                        .filter(|s| !s.is_empty())
                        .map(str::to_string)
                        .collect();

                    sas_phy.target_port_protocols = target_port_protocols
                }
                _ => {}
            }
        }

        sas_phys.insert(sas_phy_item, sas_phy);
    }

    Ok(sas_phys)
}

fn get_linkrate(link: &str, phy_path: &Path) -> CollectResult<f64> {
    let linkrate = utils::collect_info_string(link, phy_path)?.unwrap_or_default();

    let linkrate_items: Vec<String> = linkrate
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .map(str::to_string)
        .collect();

    if linkrate_items.is_empty() {
        return Ok(0.0);
    }

    let linkrate = match linkrate_items[0].parse::<f64>() {
        Ok(f) => f,
        Err(err) => {
            return Err(MetricError::ParseFloatError(
                linkrate_items[0].to_owned(),
                err,
            ))
        }
    };

    Ok(linkrate)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sas_phy_information() {
        let sas_phys =
            collect_from("test_data/fixtures/sys/class/sas_phy/").expect("sas phys information");

        for (phy_name, phy_info) in sas_phys {
            match phy_name.as_str() {
                "phy-11:0:2" => {
                    assert_eq!(phy_info.sas_address, "0x5000ccab0200947e");
                    assert_eq!(phy_info.sas_port, "port-11:0:0");
                    assert_eq!(phy_info.device_type, "edge expander");
                    assert_eq!(phy_info.initiator_port_protocols, ["smp"]);
                    assert_eq!(phy_info.invalid_dword_count, 18);
                    assert_eq!(phy_info.loss_of_dword_sync_count, 1);
                    assert_eq!(phy_info.maximum_linkrate, 12.0);
                    assert_eq!(phy_info.maximum_linkrate_hw, 12.0);
                    assert_eq!(phy_info.minimum_linkrate, 1.5);
                    assert_eq!(phy_info.minimum_linkrate_hw, 1.5);
                    assert_eq!(phy_info.negotiated_linkrate, 6.0);
                    assert_eq!(phy_info.phy_identifier, "2");
                    assert_eq!(phy_info.phy_reset_problem_count, 0);
                    assert_eq!(phy_info.running_disparity_error_count, 18);
                    assert_eq!(phy_info.target_port_protocols, ["smp"]);
                }
                "phy-11:0:4" => {}
                "phy-11:0:6" => {}
                "phy-11:7" => {}
                "phy-11:8" => {
                    assert_eq!(phy_info.sas_address, "0x500062b2047b51c4");
                    assert_eq!(phy_info.sas_port, "port-11:0");
                    assert_eq!(phy_info.device_type, "end device");
                    assert_eq!(phy_info.initiator_port_protocols.len(), 3);
                    assert_eq!(phy_info.invalid_dword_count, 0);
                    assert_eq!(phy_info.loss_of_dword_sync_count, 0);
                    assert_eq!(phy_info.maximum_linkrate, 12.0);
                    assert_eq!(phy_info.maximum_linkrate_hw, 12.0);
                    assert_eq!(phy_info.minimum_linkrate, 3.0);
                    assert_eq!(phy_info.minimum_linkrate_hw, 1.5);
                    assert_eq!(phy_info.negotiated_linkrate, 12.0);
                    assert_eq!(phy_info.phy_identifier, "8");
                    assert_eq!(phy_info.phy_reset_problem_count, 0);
                    assert_eq!(phy_info.running_disparity_error_count, 0);
                    assert_eq!(phy_info.target_port_protocols, ["none"]);
                }
                "phy-11:9" => {}
                "phy-11:10" => {}
                "phy-11:11" => {
                    assert_eq!(phy_info.sas_address, "0x500062b2047b51c4");
                    assert_eq!(phy_info.sas_port, "port-11:0");
                    assert_eq!(phy_info.device_type, "end device");
                    assert_eq!(phy_info.initiator_port_protocols.len(), 3);
                    assert_eq!(phy_info.invalid_dword_count, 0);
                    assert_eq!(phy_info.loss_of_dword_sync_count, 0);
                    assert_eq!(phy_info.maximum_linkrate, 12.0);
                    assert_eq!(phy_info.maximum_linkrate_hw, 12.0);
                    assert_eq!(phy_info.minimum_linkrate, 3.0);
                    assert_eq!(phy_info.minimum_linkrate_hw, 1.5);
                    assert_eq!(phy_info.negotiated_linkrate, 12.0);
                    assert_eq!(phy_info.phy_identifier, "11");
                    assert_eq!(phy_info.phy_reset_problem_count, 0);
                    assert_eq!(phy_info.running_disparity_error_count, 0);
                    assert_eq!(phy_info.target_port_protocols, ["none"]);
                }
                "phy-11:12" => {}
                "phy-11:13" => {
                    assert_eq!(phy_info.sas_address, "0x500062b2047b51c4");
                    assert_eq!(phy_info.sas_port, "port-11:1");
                    assert_eq!(phy_info.device_type, "end device");
                    assert_eq!(phy_info.initiator_port_protocols.len(), 3);
                    assert_eq!(phy_info.invalid_dword_count, 0);
                    assert_eq!(phy_info.loss_of_dword_sync_count, 0);
                    assert_eq!(phy_info.maximum_linkrate, 12.0);
                    assert_eq!(phy_info.maximum_linkrate_hw, 12.0);
                    assert_eq!(phy_info.minimum_linkrate, 3.0);
                    assert_eq!(phy_info.minimum_linkrate_hw, 1.5);
                    assert_eq!(phy_info.negotiated_linkrate, 6.0);
                    assert_eq!(phy_info.phy_identifier, "13");
                    assert_eq!(phy_info.phy_reset_problem_count, 0);
                    assert_eq!(phy_info.running_disparity_error_count, 0);
                    assert_eq!(phy_info.target_port_protocols, ["none"]);
                }
                "phy-11:14" => {}
                "phy-11:15" => {}
                _ => panic!("invalid sas phy name: {}", phy_name),
            }
        }
    }
}
