use regex::Regex;
use serde::Serialize;
use std::{collections::HashMap, path::PathBuf};

use crate::{
    error::{CollectResult, MetricError},
    utils,
};

#[derive(Debug, Serialize, Clone, Default)]
pub struct SASHost {
    /// /sys/class/sas_host/<Name>/device/phy-*
    pub sas_phys: Vec<String>,

    /// /sys/class/sas_host/<Name>/device/ports-*
    pub sas_ports: Vec<String>,
}

impl SASHost {
    fn new() -> Self {
        Default::default()
    }
}

/// collects sas hosts information
/// # Example
/// ```
/// use procsys::sysfs::class_sas_host;
///
/// let sashosts = class_sas_host::collect().expect("sas hosts information");
/// let json_output = serde_json::to_string_pretty(&sashosts).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<HashMap<String, SASHost>> {
    collect_from("/sys/class/sas_host/")
}

fn collect_from(dirname: &str) -> CollectResult<HashMap<String, SASHost>> {
    let mut sashosts: HashMap<String, SASHost> = HashMap::new();
    let sas_host_path = PathBuf::from(dirname);

    let re_phy = match Regex::new(r"^phy-[0-9:]+$") {
        Ok(r) => r,
        Err(err) => return Err(MetricError::RegexError(err)),
    };

    let re_port = match Regex::new(r"^port-[0-9:]+$") {
        Ok(r) => r,
        Err(err) => return Err(MetricError::RegexError(err)),
    };

    for item in utils::list_dir_content(&sas_host_path, "", "sas_host") {
        let mut sas_host = SASHost::new();
        let mut item_host_path = sas_host_path.clone();
        item_host_path.push(&item);
        item_host_path.push("device");

        for device_item in utils::list_dir_content(&item_host_path, "", &item) {
            if re_phy.is_match(&device_item) {
                sas_host.sas_phys.push(device_item);
            } else if re_port.is_match(&device_item) {
                sas_host.sas_ports.push(device_item);
            }
        }

        sashosts.insert(item.clone(), sas_host);
    }

    Ok(sashosts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sas_host_information() {
        let sashosts =
            collect_from("test_data/fixtures/sys/class/sas_host/").expect("sas hosts information");

        assert_eq!(sashosts.len(), 1);

        let sashost = sashosts.get("host11").unwrap();
        assert_eq!(sashost.sas_phys.len(), 9);
        assert_eq!(sashost.sas_ports.len(), 3);
    }
}
