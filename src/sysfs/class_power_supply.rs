use std::{collections::HashMap, path::PathBuf};

use serde::Serialize;

use crate::{error::CollectResult, utils};

/// PowerSupply contains info from files in /sys/class/power_supply for a
// single power supply
#[derive(Debug, Serialize, Clone, Default)]
pub struct PowerSupply {
    pub authentic: Option<i64>,
    pub calibrate: Option<i64>,
    pub capacity: Option<i64>,
    pub capacity_alert_max: Option<i64>,
    pub capacity_alert_min: Option<i64>,
    pub capacity_level: Option<String>,
    pub charge_avg: Option<i64>,
    pub charge_control_limit: Option<i64>,
    pub charge_control_limit_max: Option<i64>,
    pub charge_counter: Option<i64>,
    pub charge_empty: Option<i64>,
    pub charge_empty_design: Option<i64>,
    pub charge_start_threshold: Option<i64>,
    pub charge_stop_threshold: Option<i64>,
    pub charge_full: Option<i64>,
    pub charge_full_design: Option<i64>,
    pub charge_now: Option<i64>,
    pub charge_term_current: Option<i64>,
    pub charge_type: Option<String>,
    pub constant_charge_current: Option<i64>,
    pub constant_charge_current_max: Option<i64>,
    pub constant_charge_voltage: Option<i64>,
    pub constant_charge_voltage_max: Option<i64>,
    pub current_avg: Option<i64>,
    pub current_boot: Option<i64>,
    pub current_max: Option<i64>,
    pub current_now: Option<i64>,
    pub cycle_count: Option<i64>,
    pub energy_avg: Option<i64>,
    pub energy_empty: Option<i64>,
    pub energy_empty_design: Option<i64>,
    pub energy_full: Option<i64>,
    pub energy_full_design: Option<i64>,
    pub energy_now: Option<i64>,
    pub health: Option<String>,
    pub input_current_limit: Option<i64>,
    pub manufacturer: Option<String>,
    pub model_name: Option<String>,
    pub online: Option<i64>,
    pub power_avg: Option<i64>,
    pub power_now: Option<i64>,
    pub precharge_current: Option<i64>,
    pub present: Option<i64>,
    pub scope: Option<String>,
    pub serial_number: Option<String>,
    pub status: Option<String>,
    pub technology: Option<String>,
    pub temp: Option<i64>,
    pub temp_alert_max: Option<i64>,
    pub temp_alert_min: Option<i64>,
    pub temp_ambient: Option<i64>,
    pub temp_ambient_max: Option<i64>,
    pub temp_ambient_min: Option<i64>,
    pub temp_max: Option<i64>,
    pub temp_min: Option<i64>,
    pub time_to_empty_avg: Option<i64>,
    pub time_to_empty_now: Option<i64>,
    pub time_to_full_avg: Option<i64>,
    pub time_to_full_now: Option<i64>,
    pub ps_type: Option<String>,
    pub usb_type: Option<String>,
    pub voltage_avg: Option<i64>,
    pub voltage_boot: Option<i64>,
    pub voltage_max: Option<i64>,
    pub voltage_max_design: Option<i64>,
    pub voltage_min: Option<i64>,
    pub voltage_min_design: Option<i64>,
    pub voltage_now: Option<i64>,
    pub voltage_ocv: Option<i64>,
}

impl PowerSupply {
    fn new() -> Self {
        Default::default()
    }
}

/// collects the the power supplies statistics
/// # Example
/// ```
/// use procsys::sysfs::class_power_supply;
///
/// let sys_power_supplies = class_power_supply::collect().expect("power supplies information");
/// let json_output = serde_json::to_string_pretty(&sys_power_supplies).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<HashMap<String, PowerSupply>> {
    collect_from("/sys/class/power_supply/")
}

fn collect_from(filename: &str) -> CollectResult<HashMap<String, PowerSupply>> {
    let mut power_supplies: HashMap<String, PowerSupply> = HashMap::new();

    let proc_ps_path = PathBuf::from(filename);

    for ps_item in utils::list_dir_content(&proc_ps_path, "", "power_supply") {
        let mut ps_item_path = proc_ps_path.clone();
        ps_item_path.push(&ps_item);

        let mut power_supply = PowerSupply::new();
        power_supply.authentic = utils::collect_info_i64("authentic", &ps_item_path)?;

        power_supply.calibrate = utils::collect_info_i64("calibrate", &ps_item_path)?;

        power_supply.capacity = utils::collect_info_i64("capacity", &ps_item_path)?;

        power_supply.capacity_alert_max =
            utils::collect_info_i64("capacity_alert_max", &ps_item_path)?;

        power_supply.capacity_alert_min =
            utils::collect_info_i64("capacity_alert_min", &ps_item_path)?;

        power_supply.capacity_level = utils::collect_info_string("capacity_level", &ps_item_path)?;

        power_supply.charge_avg = utils::collect_info_i64("charge_avg", &ps_item_path)?;

        power_supply.charge_control_limit =
            utils::collect_info_i64("charge_control_limit", &ps_item_path)?;

        power_supply.charge_control_limit_max =
            utils::collect_info_i64("charge_control_limit_max", &ps_item_path)?;

        power_supply.charge_counter = utils::collect_info_i64("charge_counter", &ps_item_path)?;

        power_supply.charge_empty = utils::collect_info_i64("charge_empty", &ps_item_path)?;

        power_supply.charge_empty_design =
            utils::collect_info_i64("charge_empty_design", &ps_item_path)?;

        power_supply.charge_start_threshold =
            utils::collect_info_i64("charge_start_threshold", &ps_item_path)?;

        power_supply.charge_stop_threshold =
            utils::collect_info_i64("charge_stop_threshold", &ps_item_path)?;

        power_supply.charge_full = utils::collect_info_i64("charge_full", &ps_item_path)?;

        power_supply.charge_full_design =
            utils::collect_info_i64("charge_full_design", &ps_item_path)?;

        power_supply.charge_now = utils::collect_info_i64("charge_now", &ps_item_path)?;

        power_supply.charge_term_current =
            utils::collect_info_i64("charge_term_current", &ps_item_path)?;

        power_supply.charge_type = utils::collect_info_string("charge_type", &ps_item_path)?;

        power_supply.constant_charge_current =
            utils::collect_info_i64("constant_charge_current", &ps_item_path)?;

        power_supply.constant_charge_current_max =
            utils::collect_info_i64("constant_charge_current_max", &ps_item_path)?;

        power_supply.constant_charge_voltage =
            utils::collect_info_i64("constant_charge_voltage", &ps_item_path)?;

        power_supply.constant_charge_voltage_max =
            utils::collect_info_i64("constant_charge_voltage_max", &ps_item_path)?;

        power_supply.current_avg = utils::collect_info_i64("current_avg", &ps_item_path)?;

        power_supply.current_boot = utils::collect_info_i64("current_boot", &ps_item_path)?;

        power_supply.current_max = utils::collect_info_i64("current_max", &ps_item_path)?;

        power_supply.current_now = utils::collect_info_i64("current_now", &ps_item_path)?;

        power_supply.cycle_count = utils::collect_info_i64("cycle_count", &ps_item_path)?;

        power_supply.energy_avg = utils::collect_info_i64("energy_avg", &ps_item_path)?;

        power_supply.energy_empty = utils::collect_info_i64("energy_empty", &ps_item_path)?;

        power_supply.energy_empty_design =
            utils::collect_info_i64("energy_empty_design", &ps_item_path)?;

        power_supply.energy_full = utils::collect_info_i64("energy_full", &ps_item_path)?;

        power_supply.energy_full_design =
            utils::collect_info_i64("energy_full_design", &ps_item_path)?;

        power_supply.energy_now = utils::collect_info_i64("energy_now", &ps_item_path)?;

        power_supply.health = utils::collect_info_string("health", &ps_item_path)?;

        power_supply.input_current_limit =
            utils::collect_info_i64("input_current_limit", &ps_item_path)?;

        power_supply.manufacturer = utils::collect_info_string("manufacturer", &ps_item_path)?;

        power_supply.model_name = utils::collect_info_string("model_name", &ps_item_path)?;

        power_supply.online = utils::collect_info_i64("online", &ps_item_path)?;

        power_supply.power_avg = utils::collect_info_i64("power_avg", &ps_item_path)?;

        power_supply.power_now = utils::collect_info_i64("power_now", &ps_item_path)?;

        power_supply.precharge_current =
            utils::collect_info_i64("precharge_current", &ps_item_path)?;

        power_supply.present = utils::collect_info_i64("present", &ps_item_path)?;

        power_supply.scope = utils::collect_info_string("scope", &ps_item_path)?;

        power_supply.serial_number = utils::collect_info_string("serial_number", &ps_item_path)?;

        power_supply.status = utils::collect_info_string("status", &ps_item_path)?;

        power_supply.technology = utils::collect_info_string("technology", &ps_item_path)?;

        power_supply.temp = utils::collect_info_i64("temp", &ps_item_path)?;

        power_supply.temp_alert_max = utils::collect_info_i64("temp_alert_max", &ps_item_path)?;

        power_supply.temp_alert_min = utils::collect_info_i64("temp_alert_min", &ps_item_path)?;

        power_supply.temp_ambient = utils::collect_info_i64("temp_ambient", &ps_item_path)?;

        power_supply.temp_ambient_max = utils::collect_info_i64("temp_ambient_max", &ps_item_path)?;

        power_supply.temp_ambient_min = utils::collect_info_i64("temp_ambient_min", &ps_item_path)?;

        power_supply.temp_max = utils::collect_info_i64("temp_max", &ps_item_path)?;

        power_supply.temp_min = utils::collect_info_i64("temp_min", &ps_item_path)?;

        power_supply.time_to_empty_avg =
            utils::collect_info_i64("time_to_empty_avg", &ps_item_path)?;

        power_supply.time_to_empty_now =
            utils::collect_info_i64("time_to_empty_now", &ps_item_path)?;

        power_supply.time_to_full_avg = utils::collect_info_i64("time_to_full_avg", &ps_item_path)?;

        power_supply.time_to_full_now = utils::collect_info_i64("time_to_full_now", &ps_item_path)?;

        power_supply.ps_type = utils::collect_info_string("type", &ps_item_path)?;

        power_supply.usb_type = utils::collect_info_string("usb_type", &ps_item_path)?;

        power_supply.voltage_avg = utils::collect_info_i64("voltage_avg", &ps_item_path)?;

        power_supply.voltage_boot = utils::collect_info_i64("voltage_boot", &ps_item_path)?;

        power_supply.voltage_max = utils::collect_info_i64("voltage_max", &ps_item_path)?;

        power_supply.voltage_max_design =
            utils::collect_info_i64("voltage_max_design", &ps_item_path)?;

        power_supply.voltage_min = utils::collect_info_i64("voltage_min", &ps_item_path)?;

        power_supply.voltage_min_design =
            utils::collect_info_i64("voltage_min_design", &ps_item_path)?;

        power_supply.voltage_now = utils::collect_info_i64("voltage_now", &ps_item_path)?;

        power_supply.voltage_ocv = utils::collect_info_i64("voltage_ocv", &ps_item_path)?;

        power_supplies.insert(ps_item, power_supply);
    }

    Ok(power_supplies)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn power_supplies() {
        let power_supplies = collect_from("test_data/fixtures/sys/class/power_supply/")
            .expect("collecting system power supplies information");

        for (ps_name, ps_info) in &power_supplies {
            match ps_name.as_str() {
                "AC" => {
                    assert_eq!(ps_info.authentic, None);
                    assert_eq!(ps_info.calibrate, None);
                    assert_eq!(ps_info.capacity, None);
                    assert_eq!(ps_info.capacity_alert_max, None);
                    assert_eq!(ps_info.capacity_alert_min, None);
                    assert_eq!(ps_info.capacity_level, None);
                    assert_eq!(ps_info.charge_avg, None);
                    assert_eq!(ps_info.charge_control_limit, None);
                    assert_eq!(ps_info.charge_control_limit_max, None);
                    assert_eq!(ps_info.charge_counter, None);
                    assert_eq!(ps_info.charge_empty, None);
                    assert_eq!(ps_info.charge_empty_design, None);
                    assert_eq!(ps_info.charge_start_threshold, None);
                    assert_eq!(ps_info.charge_stop_threshold, None);
                    assert_eq!(ps_info.charge_full, None);
                    assert_eq!(ps_info.charge_full_design, None);
                    assert_eq!(ps_info.charge_now, None);
                    assert_eq!(ps_info.charge_term_current, None);
                    assert_eq!(ps_info.charge_type, None);
                    assert_eq!(ps_info.constant_charge_current, None);
                    assert_eq!(ps_info.constant_charge_current_max, None);
                    assert_eq!(ps_info.constant_charge_voltage, None);
                    assert_eq!(ps_info.constant_charge_voltage_max, None);
                    assert_eq!(ps_info.current_avg, None);
                    assert_eq!(ps_info.current_boot, None);
                    assert_eq!(ps_info.current_max, None);
                    assert_eq!(ps_info.current_now, None);
                    assert_eq!(ps_info.cycle_count, None);
                    assert_eq!(ps_info.energy_avg, None);
                    assert_eq!(ps_info.energy_empty, None);
                    assert_eq!(ps_info.energy_empty_design, None);
                    assert_eq!(ps_info.energy_full, None);
                    assert_eq!(ps_info.energy_full_design, None);
                    assert_eq!(ps_info.energy_now, None);
                    assert_eq!(ps_info.health, None);
                    assert_eq!(ps_info.input_current_limit, None);
                    assert_eq!(ps_info.manufacturer, None);
                    assert_eq!(ps_info.model_name, None);
                    assert_eq!(ps_info.online, Some(0));
                    assert_eq!(ps_info.power_avg, None);
                    assert_eq!(ps_info.power_now, None);
                    assert_eq!(ps_info.precharge_current, None);
                    assert_eq!(ps_info.present, None);
                    assert_eq!(ps_info.scope, None);
                    assert_eq!(ps_info.serial_number, None);
                    assert_eq!(ps_info.status, None);
                    assert_eq!(ps_info.technology, None);
                    assert_eq!(ps_info.temp, None);
                    assert_eq!(ps_info.temp_alert_max, None);
                    assert_eq!(ps_info.temp_alert_min, None);
                    assert_eq!(ps_info.temp_ambient, None);
                    assert_eq!(ps_info.temp_ambient_max, None);
                    assert_eq!(ps_info.temp_ambient_min, None);
                    assert_eq!(ps_info.temp_max, None);
                    assert_eq!(ps_info.temp_min, None);
                    assert_eq!(ps_info.time_to_empty_avg, None);
                    assert_eq!(ps_info.time_to_empty_now, None);
                    assert_eq!(ps_info.time_to_full_avg, None);
                    assert_eq!(ps_info.time_to_full_now, None);
                    assert_eq!(ps_info.ps_type, Some(String::from("Mains")));
                    assert_eq!(ps_info.usb_type, None);
                    assert_eq!(ps_info.voltage_avg, None);
                    assert_eq!(ps_info.voltage_boot, None);
                    assert_eq!(ps_info.voltage_max, None);
                    assert_eq!(ps_info.voltage_max_design, None);
                    assert_eq!(ps_info.voltage_min, None);
                    assert_eq!(ps_info.voltage_min_design, None);
                    assert_eq!(ps_info.voltage_now, None);
                    assert_eq!(ps_info.voltage_ocv, None);
                }
                "BAT0" => {
                    assert_eq!(ps_info.authentic, None);
                    assert_eq!(ps_info.calibrate, None);
                    assert_eq!(ps_info.capacity, Some(98));
                    assert_eq!(ps_info.capacity_alert_max, None);
                    assert_eq!(ps_info.capacity_alert_min, None);
                    assert_eq!(ps_info.capacity_level, Some(String::from("Normal")));
                    assert_eq!(ps_info.charge_avg, None);
                    assert_eq!(ps_info.charge_control_limit, None);
                    assert_eq!(ps_info.charge_control_limit_max, None);
                    assert_eq!(ps_info.charge_counter, None);
                    assert_eq!(ps_info.charge_empty, None);
                    assert_eq!(ps_info.charge_empty_design, None);
                    assert_eq!(ps_info.charge_start_threshold, Some(95));
                    assert_eq!(ps_info.charge_stop_threshold, Some(100));
                    assert_eq!(ps_info.charge_full, None);
                    assert_eq!(ps_info.charge_full_design, None);
                    assert_eq!(ps_info.charge_now, None);
                    assert_eq!(ps_info.charge_term_current, None);
                    assert_eq!(ps_info.charge_type, None);
                    assert_eq!(ps_info.constant_charge_current, None);
                    assert_eq!(ps_info.constant_charge_current_max, None);
                    assert_eq!(ps_info.constant_charge_voltage, None);
                    assert_eq!(ps_info.constant_charge_voltage_max, None);
                    assert_eq!(ps_info.current_avg, None);
                    assert_eq!(ps_info.current_boot, None);
                    assert_eq!(ps_info.current_max, None);
                    assert_eq!(ps_info.current_now, None);
                    assert_eq!(ps_info.cycle_count, Some(0));
                    assert_eq!(ps_info.energy_avg, None);
                    assert_eq!(ps_info.energy_empty, None);
                    assert_eq!(ps_info.energy_empty_design, None);
                    assert_eq!(ps_info.energy_full, Some(50060000));
                    assert_eq!(ps_info.energy_full_design, Some(47520000));
                    assert_eq!(ps_info.energy_now, Some(49450000));
                    assert_eq!(ps_info.health, None);
                    assert_eq!(ps_info.input_current_limit, None);
                    assert_eq!(ps_info.manufacturer, Some(String::from("LGC")));
                    assert_eq!(ps_info.model_name, Some(String::from("LNV-45N1")));
                    assert_eq!(ps_info.online, None);
                    assert_eq!(ps_info.power_avg, None);
                    assert_eq!(ps_info.power_now, Some(4830000));
                    assert_eq!(ps_info.precharge_current, None);
                    assert_eq!(ps_info.present, Some(1));
                    assert_eq!(ps_info.scope, None);
                    assert_eq!(ps_info.serial_number, Some(String::from("38109")));
                    assert_eq!(ps_info.status, Some(String::from("Discharging")));
                    assert_eq!(ps_info.technology, Some(String::from("Li-ion")));
                    assert_eq!(ps_info.temp, None);
                    assert_eq!(ps_info.temp_alert_max, None);
                    assert_eq!(ps_info.temp_alert_min, None);
                    assert_eq!(ps_info.temp_ambient, None);
                    assert_eq!(ps_info.temp_ambient_max, None);
                    assert_eq!(ps_info.temp_ambient_min, None);
                    assert_eq!(ps_info.temp_max, None);
                    assert_eq!(ps_info.temp_min, None);
                    assert_eq!(ps_info.time_to_empty_avg, None);
                    assert_eq!(ps_info.time_to_empty_now, None);
                    assert_eq!(ps_info.time_to_full_avg, None);
                    assert_eq!(ps_info.time_to_full_now, None);
                    assert_eq!(ps_info.ps_type, Some(String::from("Battery")));
                    assert_eq!(ps_info.usb_type, None);
                    assert_eq!(ps_info.voltage_avg, None);
                    assert_eq!(ps_info.voltage_boot, None);
                    assert_eq!(ps_info.voltage_max, None);
                    assert_eq!(ps_info.voltage_max_design, None);
                    assert_eq!(ps_info.voltage_min, None);
                    assert_eq!(ps_info.voltage_min_design, Some(10800000));
                    assert_eq!(ps_info.voltage_now, Some(12229000));
                    assert_eq!(ps_info.voltage_ocv, None);
                }
                _ => panic!("invalid power supply name: {}", ps_name),
            }
        }
    }
}
