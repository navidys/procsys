use std::{collections::HashMap, path::PathBuf};

use serde::Serialize;

use crate::{error::CollectResult, utils};

/// ScsiTapeCounters contains statistics info for a single scsi tape
#[derive(Debug, Serialize, Clone, Default)]
pub struct ScsiTapeCounters {
    pub write_ns: u64,
    pub read_byte_cnt: u64,
    pub io_ns: u64,
    pub write_cnt: u64,
    pub resid_cnt: u64,
    pub read_ns: u64,
    pub in_flight: u64,
    pub other_cnt: u64,
    pub read_cnt: u64,
    pub write_byte_cnt: u64,
}

impl ScsiTapeCounters {
    fn new() -> Self {
        Default::default()
    }
}

/// collects the the power supplies statistics
/// # Example
/// ```
/// use procsys::sysfs::class_scsi_tape;
///
/// let sys_scsi_tapes = class_scsi_tape::collect().expect("scsi tape statistics information");
/// let json_output = serde_json::to_string_pretty(&sys_scsi_tapes).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<HashMap<String, ScsiTapeCounters>> {
    collect_from("/sys/class/scsi_tape/")
}

fn collect_from(filename: &str) -> CollectResult<HashMap<String, ScsiTapeCounters>> {
    let mut scsi_tapes: HashMap<String, ScsiTapeCounters> = HashMap::new();

    let scsi_tape_path = PathBuf::from(filename);

    for scsitape_item in utils::list_dir_content(&scsi_tape_path, "", "scsi_tape") {
        let mut scsitape_item_path = scsi_tape_path.clone();
        scsitape_item_path.push(&scsitape_item);
        scsitape_item_path.push("stats");

        let mut scsitape_counters = ScsiTapeCounters::new();

        scsitape_counters.write_ns =
            utils::collect_info_u64("write_ns", &scsitape_item_path)?.unwrap_or_default();

        scsitape_counters.read_byte_cnt =
            utils::collect_info_u64("read_byte_cnt", &scsitape_item_path)?.unwrap_or_default();

        scsitape_counters.io_ns =
            utils::collect_info_u64("io_ns", &scsitape_item_path)?.unwrap_or_default();

        scsitape_counters.write_cnt =
            utils::collect_info_u64("write_cnt", &scsitape_item_path)?.unwrap_or_default();

        scsitape_counters.resid_cnt =
            utils::collect_info_u64("resid_cnt", &scsitape_item_path)?.unwrap_or_default();

        scsitape_counters.read_ns =
            utils::collect_info_u64("read_ns", &scsitape_item_path)?.unwrap_or_default();

        scsitape_counters.in_flight =
            utils::collect_info_u64("in_flight", &scsitape_item_path)?.unwrap_or_default();

        scsitape_counters.other_cnt =
            utils::collect_info_u64("other_cnt", &scsitape_item_path)?.unwrap_or_default();

        scsitape_counters.read_cnt =
            utils::collect_info_u64("read_cnt", &scsitape_item_path)?.unwrap_or_default();

        scsitape_counters.write_byte_cnt =
            utils::collect_info_u64("write_byte_cnt", &scsitape_item_path)?.unwrap_or_default();

        scsi_tapes.insert(scsitape_item, scsitape_counters);
    }

    Ok(scsi_tapes)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scsi_tape_counters() {
        let scsi_tapes = collect_from("test_data/fixtures/sys/class/scsi_tape/")
            .expect("collecting scsi tapes stats information");

        for (name, stats) in scsi_tapes {
            match name.as_str() {
                "nst0" | "nst0a" | "nst0l" | "nst0m" | "st0" | "st0a" | "st0l" | "st0m" => {
                    assert_eq!(stats.write_ns, 5233597394395);
                    assert_eq!(stats.read_byte_cnt, 979383912);
                    assert_eq!(stats.io_ns, 9247011087720);
                    assert_eq!(stats.write_cnt, 53772916);
                    assert_eq!(stats.resid_cnt, 19);
                    assert_eq!(stats.read_ns, 33788355744);
                    assert_eq!(stats.in_flight, 1);
                    assert_eq!(stats.other_cnt, 1409);
                    assert_eq!(stats.read_cnt, 3741);
                    assert_eq!(stats.write_byte_cnt, 1496246784000);
                }
                _ => panic!("invalid scsi tape name: {}", name),
            }
        }
    }
}
