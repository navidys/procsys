use std::{collections::HashMap, path::PathBuf};

use serde::Serialize;

use crate::{error::CollectResult, utils};

/// NVMeDevice contains info from files in /sys/class/nvme for a single NVMe device
#[derive(Debug, Serialize, Clone, Default)]
pub struct NVMeDevice {
    pub serial: String,
    pub model: String,
    pub state: String,
    pub firmware_revision: String,
}

impl NVMeDevice {
    fn new() -> Self {
        Default::default()
    }
}

/// collects the the nvme devices information
/// # Example
/// ```
/// use procsys::sysfs::class_nvme;
///
/// let nvme_devices = class_nvme::collect().expect("nvme devices information");
/// let json_output = serde_json::to_string_pretty(&nvme_devices).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<HashMap<String, NVMeDevice>> {
    collect_from("/sys/class/nvme/")
}

fn collect_from(filename: &str) -> CollectResult<HashMap<String, NVMeDevice>> {
    let mut nvme_devices: HashMap<String, NVMeDevice> = HashMap::new();

    let nvme_path = PathBuf::from(filename);

    for nvme_item in utils::list_dir_content(&nvme_path, "", "nvme") {
        let mut nvme_item_path = nvme_path.clone();
        nvme_item_path.push(&nvme_item);

        let mut nvme_device = NVMeDevice::new();

        nvme_device.serial =
            utils::collect_info_string("serial", &nvme_item_path)?.unwrap_or_default();

        nvme_device.model =
            utils::collect_info_string("model", &nvme_item_path)?.unwrap_or_default();

        nvme_device.state =
            utils::collect_info_string("state", &nvme_item_path)?.unwrap_or_default();

        nvme_device.firmware_revision =
            utils::collect_info_string("firmware_rev", &nvme_item_path)?.unwrap_or_default();

        nvme_devices.insert(nvme_item, nvme_device);
    }

    Ok(nvme_devices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nvme_information() {
        let nvme_devices = collect_from("test_data/fixtures/sys/class/nvme/")
            .expect("collecting nvme devices information");

        for (name, nvme_dev) in nvme_devices {
            match name.as_str() {
                "nvme0" => {
                    assert_eq!(nvme_dev.serial, "S680HF8N190894I");
                    assert_eq!(nvme_dev.model, "Samsung SSD 970 PRO 512GB");
                    assert_eq!(nvme_dev.state, "live");
                    assert_eq!(nvme_dev.firmware_revision, "1B2QEXP7");
                }
                _ => panic!("invalid nvme device name: {}", name),
            }
        }
    }
}
