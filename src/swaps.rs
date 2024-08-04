use serde::Serialize;

use crate::{
    error::{CollectResult, MetricError},
    utils,
};

/// LoadAvg represents an entry in /proc/loadavg
#[derive(Debug, Serialize, Clone, Default)]
pub struct Swaps {
    pub filename: String,
    pub swap_type: String,
    pub size: u64,
    pub used: u64,
    pub priority: i32,
}

impl Swaps {
    fn new() -> Self {
        Default::default()
    }
}

/// collects returns all configured swap devices on the system
/// # Example
/// ```
/// use procsys::swaps;
///
/// let sys_swapinfo = swaps::collect().expect("swaps information");
/// let json_output = serde_json::to_string_pretty(&sys_swapinfo).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<Vec<Swaps>> {
    collect_from("/proc/swaps")
}

fn collect_from(filename: &str) -> CollectResult<Vec<Swaps>> {
    let mut swaps_info: Vec<Swaps> = Vec::new();
    let swaps_data = utils::read_file_lines(filename)?;

    for line in &swaps_data[1..] {
        let swap_info = line.to_owned();
        let swap_info_field: Vec<&str> = swap_info
            .split_whitespace()
            .filter(|s| !s.is_empty())
            .collect();

        if swap_info_field.len() != 5 {
            return Err(MetricError::InvalidFieldNumberError(
                "swaps".to_string(),
                swap_info_field.len(),
                line.to_owned(),
            ));
        }

        let mut swap_data = Swaps::new();
        swap_data.filename = swap_info_field[0].to_string();
        swap_data.swap_type = swap_info_field[1].to_string();
        swap_data.size = swap_info_field[2].parse::<u64>().unwrap_or_default();
        swap_data.used = swap_info_field[3].parse::<u64>().unwrap_or_default();
        swap_data.priority = swap_info_field[4].parse::<i32>().unwrap_or_default();

        swaps_info.push(swap_data);
    }

    Ok(swaps_info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn system_swaps() {
        let swaps_info = collect_from("test_data/fixtures/proc/swaps").expect("swaps information");

        assert_eq!(swaps_info.len(), 2);

        for swap_data in swaps_info {
            match swap_data.filename.as_str() {
                "/dev/zram0" => {
                    assert_eq!(swap_data.swap_type, "partition");
                    assert_eq!(swap_data.size, 8388604);
                    assert_eq!(swap_data.used, 0);
                    assert_eq!(swap_data.priority, 100);
                }
                "/dev/dm-2" => {
                    assert_eq!(swap_data.swap_type, "partition");
                    assert_eq!(swap_data.size, 131068);
                    assert_eq!(swap_data.used, 176);
                    assert_eq!(swap_data.priority, -2);
                }
                _ => panic!("invalid swap filename: {}", swap_data.filename),
            }
        }
    }
}
