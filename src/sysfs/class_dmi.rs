use crate::{error::MetricError, utils};
use getset::Getters;
use serde::Serialize;
use std::path::Path;

enum DMIType {
    BiosDate,
    BiosRelease,
    BiosVendor,
    BiosVersion,
    BoardAssetTag,
    BoardName,
    BoardSerial,
    BoardVendor,
    BoardVersion,
    ChassisAssetTag,
    ChassisSerial,
    ChassisType,
    ChassisVendor,
    ChassisVersion,
    ProductFamily,
    ProductName,
    ProductSerial,
    ProductSku,
    ProductUuid,
    SystemVendor,
    Unknown,
}

impl DMIType {
    fn from(name: &str) -> DMIType {
        match name {
            "bios_date" => DMIType::BiosDate,
            "bios_release" => DMIType::BiosRelease,
            "bios_vendor" => DMIType::BiosVendor,
            "bios_version" => DMIType::BiosVersion,
            "board_asset_tag" => DMIType::BoardAssetTag,
            "board_name" => DMIType::BoardName,
            "board_serial" => DMIType::BoardSerial,
            "board_vendor" => DMIType::BoardVendor,
            "board_version" => DMIType::BoardVersion,
            "chassis_asset_tag" => DMIType::ChassisAssetTag,
            "chassis_serial" => DMIType::ChassisSerial,
            "chassis_type" => DMIType::ChassisType,
            "chassis_vendor" => DMIType::ChassisVendor,
            "chassis_version" => DMIType::ChassisVersion,
            "product_family" => DMIType::ProductFamily,
            "product_name" => DMIType::ProductName,
            "product_serial" => DMIType::ProductSerial,
            "product_sku" => DMIType::ProductSku,
            "product_uuid" => DMIType::ProductUuid,
            "sys_vendor" => DMIType::SystemVendor,
            _ => DMIType::Unknown,
        }
    }
}

/// The DMI contains the content of Desktop Management Interface from files in in /sys/class/dmi/id
/// # Example
/// ```
/// use procsys::sysfs::class_dmi;
///
/// let dmi_info = class_dmi::collect();
///
/// println!("bios date: {}", dmi_info.bios_date().to_owned().unwrap_or_default());
/// println!("board serial: {}", dmi_info.board_serial().to_owned().unwrap_or_default());
///
/// // print all dmi information in json format
/// let json_output = serde_json::to_string_pretty(&dmi_info).unwrap();
/// println!("{}", json_output);
///
/// ```
#[derive(Debug, Serialize, Clone, Getters)]
pub struct DMI {
    #[getset(get = "pub")]
    bios_date: Option<String>,

    #[getset(get = "pub")]
    bios_release: Option<String>,

    #[getset(get = "pub")]
    bios_vendor: Option<String>,

    #[getset(get = "pub")]
    bios_version: Option<String>,

    #[getset(get = "pub")]
    board_asset_tag: Option<String>,

    #[getset(get = "pub")]
    board_name: Option<String>,

    #[getset(get = "pub")]
    board_serial: Option<String>,

    #[getset(get = "pub")]
    board_vendor: Option<String>,

    #[getset(get = "pub")]
    board_version: Option<String>,

    #[getset(get = "pub")]
    chassis_asset_tag: Option<String>,

    #[getset(get = "pub")]
    chassis_serial: Option<String>,

    #[getset(get = "pub")]
    chassis_type: Option<String>,

    #[getset(get = "pub")]
    chassis_vendor: Option<String>,

    #[getset(get = "pub")]
    chassis_version: Option<String>,

    #[getset(get = "pub")]
    product_family: Option<String>,

    #[getset(get = "pub")]
    product_name: Option<String>,

    #[getset(get = "pub")]
    product_serial: Option<String>,

    #[getset(get = "pub")]
    product_sku: Option<String>,

    #[getset(get = "pub")]
    product_uuid: Option<String>,

    #[getset(get = "pub")]
    product_version: Option<String>,

    #[getset(get = "pub")]
    system_vendor: Option<String>,
}

impl DMI {
    fn new() -> Self {
        Self {
            bios_date: None,
            bios_release: None,
            bios_vendor: None,
            bios_version: None,
            board_asset_tag: None,
            board_name: None,
            board_serial: None,
            board_vendor: None,
            board_version: None,
            chassis_asset_tag: None,
            chassis_serial: None,
            chassis_type: None,
            chassis_vendor: None,
            chassis_version: None,
            product_family: None,
            product_name: None,
            product_serial: None,
            product_sku: None,
            product_uuid: None,
            product_version: None,
            system_vendor: None,
        }
    }
}

/// attempts to collect dmi information.
pub fn collect() -> DMI {
    let mut dmi = DMI::new();

    let dmi_class_path = Path::new("/sys/class/dmi/id");
    if !dmi_class_path.exists() {
        log::error!("{}", MetricError::DmiSupportError);

        return dmi;
    }

    for device in utils::list_dir_content(dmi_class_path, "", "id") {
        match DMIType::from(device.as_str()) {
            DMIType::BiosDate => {
                dmi.bios_date = utils::collect_info_string(&device, dmi_class_path);
            }

            DMIType::BiosRelease => {
                dmi.bios_release = utils::collect_info_string(&device, dmi_class_path);
            }

            DMIType::BiosVendor => {
                dmi.bios_vendor = utils::collect_info_string(&device, dmi_class_path);
            }

            DMIType::BiosVersion => {
                dmi.bios_version = utils::collect_info_string(&device, dmi_class_path);
            }

            DMIType::BoardAssetTag => {
                dmi.board_asset_tag = utils::collect_info_string(&device, dmi_class_path);
            }

            DMIType::BoardName => {
                dmi.board_name = utils::collect_info_string(&device, dmi_class_path);
            }

            DMIType::BoardSerial => {
                dmi.board_serial = utils::collect_info_string(&device, dmi_class_path);
            }

            DMIType::BoardVendor => {
                dmi.board_vendor = utils::collect_info_string(&device, dmi_class_path);
            }

            DMIType::BoardVersion => {
                dmi.board_version = utils::collect_info_string(&device, dmi_class_path);
            }

            DMIType::ChassisAssetTag => {
                dmi.chassis_asset_tag = utils::collect_info_string(&device, dmi_class_path);
            }

            DMIType::ChassisSerial => {
                dmi.chassis_serial = utils::collect_info_string(&device, dmi_class_path);
            }

            DMIType::ChassisType => {
                dmi.chassis_type = utils::collect_info_string(&device, dmi_class_path);
            }

            DMIType::ChassisVendor => {
                dmi.chassis_vendor = utils::collect_info_string(&device, dmi_class_path);
            }

            DMIType::ChassisVersion => {
                dmi.chassis_version = utils::collect_info_string(&device, dmi_class_path);
            }

            DMIType::ProductFamily => {
                dmi.product_family = utils::collect_info_string(&device, dmi_class_path);
            }

            DMIType::ProductName => {
                dmi.product_name = utils::collect_info_string(&device, dmi_class_path);
            }

            DMIType::ProductSerial => {
                dmi.product_serial = utils::collect_info_string(&device, dmi_class_path);
            }

            DMIType::ProductSku => {
                dmi.product_sku = utils::collect_info_string(&device, dmi_class_path);
            }

            DMIType::ProductUuid => {
                dmi.product_uuid = utils::collect_info_string(&device, dmi_class_path);
            }

            DMIType::SystemVendor => {
                dmi.system_vendor = utils::collect_info_string(&device, dmi_class_path);
            }

            DMIType::Unknown => {}
        }
    }

    dmi
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dmi_collect() {
        let dmi_result = collect();
        assert!(dmi_result.bios_date.is_some())
    }
}
