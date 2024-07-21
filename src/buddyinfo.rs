use serde::Serialize;

use crate::utils;

/// BuddyInfo is the details parsed from /proc/buddyinfo
/// The data is comprised of an array of free fragments of each size
/// The sizes are 2^n*PAGE_SIZE, where n is the array index
#[derive(Debug, Serialize, Clone)]
pub struct BuddyInfo {
    node: String,
    zone: String,
    sizes: Vec<u64>,
}

impl BuddyInfo {
    fn new() -> Self {
        Self {
            node: String::new(),
            zone: String::new(),
            sizes: Vec::<u64>::new(),
        }
    }
}

/// collects reads the buddyinfo statistics from the specified `proc` filesystem
/// # Example
/// ```
/// use procsys::buddyinfo;
///
/// let binfo = buddyinfo::collect();
/// let json_output = serde_json::to_string_pretty(&binfo).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> Vec<BuddyInfo> {
    let mut system_buddyinfo = Vec::new();

    for line in utils::read_file_lines("/proc/buddyinfo") {
        let fields: Vec<&str> = line.trim().split(' ').filter(|s| !s.is_empty()).collect();

        if fields.len() < 4 {
            log::error!(
                "invalid buddyinfo fields number, found {}: {:?}",
                fields.len(),
                fields,
            );
            continue;
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

    system_buddyinfo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn buddyinfo() {
        let binfolist = collect();
        for binfo in binfolist {
            assert!(!binfo.node.is_empty());
            assert!(!binfo.zone.is_empty());
            assert!(!binfo.sizes.is_empty())
        }
    }
}
