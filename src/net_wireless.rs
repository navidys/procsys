use serde::Serialize;

use crate::{
    error::{CollectResult, MetricError},
    utils,
};

/// Wireless models the content of /proc/net/wireless
#[derive(Debug, Serialize, Clone, Default)]
pub struct Wireless {
    pub name: String,

    /// status is the current 4-digit hex value status of the interface
    pub status: u64,

    /// quality_link is the link quality
    pub quality_link: i64,

    /// quality_level is the signal gain (dBm)
    pub quality_level: i64,

    /// quality_noise is the signal noise baseline (dBm)
    pub quality_noise: i64,

    /// discarded_nwid is the number of discarded packets with wrong nwid/essid
    pub discarded_nwid: i64,

    /// discarded_crypt is the number of discarded packets with wrong code/decode (WEP)
    pub discarded_crypt: i64,

    /// discarded_frag is the number of discarded packets that can't perform MAC reassembly
    pub discarded_frag: i64,

    /// discarded_retry is the number of discarded packets that reached max MAC retries
    pub discarded_retry: i64,

    /// discarded_misc is the number of discarded packets for other reasons
    pub discarded_misc: i64,

    /// missed_beacon is the number of missed beacons/superframe
    pub missed_beacon: i64,
}

impl Wireless {
    fn new() -> Self {
        Default::default()
    }
}

/// collects the network wireless information
/// # Example
/// ```
/// use procsys::net_wireless;
///
/// let netwireless = net_wireless::collect().expect("network wireless information");
/// let json_output = serde_json::to_string_pretty(&netwireless).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<Vec<Wireless>> {
    collect_from("/proc/net/wireless")
}

fn collect_from(filename: &str) -> CollectResult<Vec<Wireless>> {
    let mut netwireless: Vec<Wireless> = Vec::new();

    let mut line_index = 0;
    for line in utils::read_file_lines(filename)? {
        line_index += 1;

        if line_index <= 2 {
            continue;
        }

        let fields: Vec<&str> = line.trim().split(' ').filter(|s| !s.is_empty()).collect();

        if fields.len() < 11 {
            return Err(MetricError::InvalidFieldNumberError(
                "wireless".to_string(),
                fields.len(),
                line,
            ));
        }

        let mut wireless = Wireless::new();
        wireless.name = fields[0].trim_matches(':').to_string();
        wireless.status = u64::from_str_radix(fields[1], 16).unwrap_or_default();

        let quality_link = fields[2].trim_end_matches(".");
        wireless.quality_link = quality_link.parse::<i64>().unwrap_or_default();

        let quality_level = fields[3].trim_end_matches(".");
        wireless.quality_level = quality_level.parse::<i64>().unwrap_or_default();

        let quality_noise = fields[4].trim_end_matches(".");
        wireless.quality_noise = quality_noise.parse::<i64>().unwrap_or_default();
        wireless.discarded_nwid = fields[5].parse::<i64>().unwrap_or_default();
        wireless.discarded_crypt = fields[6].parse::<i64>().unwrap_or_default();
        wireless.discarded_frag = fields[7].parse::<i64>().unwrap_or_default();
        wireless.discarded_retry = fields[8].parse::<i64>().unwrap_or_default();
        wireless.discarded_misc = fields[9].parse::<i64>().unwrap_or_default();
        wireless.missed_beacon = fields[10].parse::<i64>().unwrap_or_default();

        netwireless.push(wireless);
    }

    Ok(netwireless)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn net_wireless() {
        let netwireless = collect_from("test_data/fixtures/proc/net/wireless")
            .expect("collecting network wireless information");

        assert_eq!(netwireless.len(), 2);

        for netw in netwireless {
            match netw.name.as_str() {
                "wlan0" => {
                    assert_eq!(netw.status, 1);
                    assert_eq!(netw.quality_link, 2);
                    assert_eq!(netw.quality_level, 3);
                    assert_eq!(netw.quality_noise, 4);
                    assert_eq!(netw.discarded_nwid, 5);
                    assert_eq!(netw.discarded_crypt, 6);
                    assert_eq!(netw.discarded_frag, 7);
                    assert_eq!(netw.discarded_retry, 8);
                    assert_eq!(netw.discarded_misc, 9);
                    assert_eq!(netw.missed_beacon, 10);
                }
                "wlan1" => {
                    assert_eq!(netw.status, 16);
                    assert_eq!(netw.quality_link, 9);
                    assert_eq!(netw.quality_level, 8);
                    assert_eq!(netw.quality_noise, 7);
                    assert_eq!(netw.discarded_nwid, 6);
                    assert_eq!(netw.discarded_crypt, 5);
                    assert_eq!(netw.discarded_frag, 4);
                    assert_eq!(netw.discarded_retry, 3);
                    assert_eq!(netw.discarded_misc, 2);
                    assert_eq!(netw.missed_beacon, 1);
                }
                _ => panic!("invalid wireless name: {}", netw.name),
            }
        }
    }
}
