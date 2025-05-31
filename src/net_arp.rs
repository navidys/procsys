use mac_address;
use serde::Serialize;
use std::net;

use crate::{
    error::{CollectResult, MetricError},
    utils,
};

// Learned from https://github.com/prometheus/procfs/arp.go include/uapi/linux/if_arp.h.
const ATF_COMPLETE: i32 = 0x02;
const ATF_PERMANENT: i32 = 0x04;
const ATF_PUBLISH: i32 = 0x08;
const ATF_USE_TRAILERS: i32 = 0x10;
const ATF_NETMASK: i32 = 0x20;
const ATF_DONT_PUBLISH: i32 = 0x40;

/// ARPEntry contains a network device information parsed from /proc/net/arp
#[derive(Debug, Serialize, Clone)]
pub struct ARPEntry {
    pub ip_address: net::IpAddr,
    pub hw_address: mac_address::MacAddress,
    pub device: String,
    pub flags: i32,
}

impl ARPEntry {
    fn new() -> Self {
        Self {
            ip_address: net::IpAddr::V4(net::Ipv4Addr::new(0, 0, 0, 0)),
            hw_address: Default::default(),
            device: Default::default(),
            flags: Default::default(),
        }
    }

    pub fn is_complete(&self) -> bool {
        self.flags == ATF_COMPLETE
    }

    pub fn is_permanent(&self) -> bool {
        self.flags == ATF_PERMANENT
    }

    pub fn is_publish(&self) -> bool {
        self.flags == ATF_PUBLISH
    }

    pub fn is_use_trailers(&self) -> bool {
        self.flags == ATF_USE_TRAILERS
    }

    pub fn is_netmask(&self) -> bool {
        self.flags == ATF_NETMASK
    }

    pub fn is_dont_publish(&self) -> bool {
        self.flags == ATF_DONT_PUBLISH
    }
}

/// collects network device information
/// # Example
/// ```
/// use procsys::net_arp;
///
/// let net_arp_entries = net_arp::collect().expect("network arp entries");
/// let json_output = serde_json::to_string_pretty(&net_arp_entries).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<Vec<ARPEntry>> {
    collect_from("/proc/net/arp")
}

fn collect_from(filename: &str) -> CollectResult<Vec<ARPEntry>> {
    let mut arp_entries = Vec::new();

    let mut line_index = 0;
    for line in utils::read_file_lines(filename)? {
        line_index += 1;

        if line_index == 1 {
            continue;
        }

        let fields: Vec<&str> = line.trim().split(' ').filter(|s| !s.is_empty()).collect();
        let mut arp_entry = ARPEntry::new();

        let ip_addr = match fields[0].parse::<net::IpAddr>() {
            Ok(ip) => ip,
            Err(err) => return Err(MetricError::ParseError(err.to_string())),
        };

        let mac_addr = match fields[3].parse::<mac_address::MacAddress>() {
            Ok(mac) => mac,
            Err(err) => return Err(MetricError::ParseError(err.to_string())),
        };

        let arp_flag = utils::convert_hex_to_i32(fields[2])?;

        arp_entry.ip_address = ip_addr;
        arp_entry.hw_address = mac_addr;
        arp_entry.device = fields[5].to_string();
        arp_entry.flags = arp_flag;

        arp_entries.push(arp_entry);
    }

    Ok(arp_entries)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn arp_entries() {
        let arp_entries =
            collect_from("test_data/fixtures/proc/net/arp").expect("arp entries information");
        for arp_entry in arp_entries {
            match arp_entry.hw_address.to_string().as_str() {
                "00:50:56:C0:00:08" => {
                    let ip_addr = "192.168.224.1".parse::<net::IpAddr>().expect("ip address");
                    assert_eq!(arp_entry.ip_address, ip_addr);
                    assert_eq!(arp_entry.device, "ens33");
                    assert_eq!(arp_entry.is_complete(), true);
                    assert_eq!(arp_entry.is_permanent(), false);
                    assert_eq!(arp_entry.is_publish(), false);
                    assert_eq!(arp_entry.is_use_trailers(), false);
                    assert_eq!(arp_entry.is_netmask(), false);
                    assert_eq!(arp_entry.is_dont_publish(), false);
                }
                "00:00:00:00:00:00" => {
                    let ip_addr = "192.168.224.2".parse::<net::IpAddr>().expect("ip address");
                    assert_eq!(arp_entry.ip_address, ip_addr);
                    assert_eq!(arp_entry.device, "ens33");
                    assert_eq!(arp_entry.is_complete(), false);
                    assert_eq!(arp_entry.is_permanent(), false);
                    assert_eq!(arp_entry.is_publish(), false);
                    assert_eq!(arp_entry.is_use_trailers(), false);
                    assert_eq!(arp_entry.is_netmask(), false);
                    assert_eq!(arp_entry.is_dont_publish(), false);
                }
                _ => panic!("invalid arp entry hw address: {}", arp_entry.hw_address),
            }
        }
    }
}
