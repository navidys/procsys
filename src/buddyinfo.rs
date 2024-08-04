use serde::Serialize;

use crate::{
    error::{CollectResult, MetricError},
    utils,
};

/// BuddyInfo is the details parsed from /proc/buddyinfo
/// The data is comprised of an array of free fragments of each size
/// The sizes are 2^n*PAGE_SIZE, where n is the array index
#[derive(Debug, Serialize, Clone, Default)]
pub struct BuddyInfo {
    pub node: String,
    pub zone: String,
    pub sizes: Vec<u64>,
}

impl BuddyInfo {
    fn new() -> Self {
        Default::default()
    }
}

/// collects reads the buddyinfo statistics from the specified `proc` filesystem
/// # Example
/// ```
/// use procsys::buddyinfo;
///
/// let binfo = buddyinfo::collect().expect("buddy information");
/// let json_output = serde_json::to_string_pretty(&binfo).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<Vec<BuddyInfo>> {
    collect_from("/proc/buddyinfo")
}

fn collect_from(base_path: &str) -> CollectResult<Vec<BuddyInfo>> {
    let mut system_buddyinfo = Vec::new();

    for line in utils::read_file_lines(base_path)? {
        let fields: Vec<&str> = line.trim().split(' ').filter(|s| !s.is_empty()).collect();

        if fields.len() < 4 {
            return Err(MetricError::InvalidFieldNumberError(
                "buddyinfo".to_string(),
                fields.len(),
                line,
            ));
        }

        let mut buddyinfo = BuddyInfo::new();

        buddyinfo.node = fields[1].replace(',', "");
        buddyinfo.zone = fields[3].replace(',', "");

        for item in &fields[4..] {
            buddyinfo
                .sizes
                .push((*item).parse::<u64>().unwrap_or_default())
        }

        system_buddyinfo.push(buddyinfo);
    }

    Ok(system_buddyinfo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buddyinfo() {
        let binfolist = collect_from("test_data/fixtures/proc/buddyinfo")
            .expect("collecting buddy information");

        assert_eq!(binfolist.len(), 3);

        for binfo in binfolist {
            assert_eq!(binfo.node, "0");
            match binfo.zone.as_str() {
                "DMA" => {
                    assert_eq!(binfo.sizes, [1, 0, 1, 0, 2, 1, 1, 0, 1, 1, 3]);
                }
                "DMA32" => {
                    assert_eq!(binfo.sizes, [759, 572, 791, 475, 194, 45, 12, 0, 0, 0, 0]);
                }
                "Normal" => {
                    assert_eq!(
                        binfo.sizes,
                        [4381, 1093, 185, 1530, 567, 102, 4, 0, 0, 0, 0]
                    );
                }
                _ => panic!("invalid zone name: {}", binfo.zone),
            }
        }
    }
}
