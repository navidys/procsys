use serde::Serialize;

use crate::{error::CollectResult, utils};

/// ARPEntry contains a network device information parsed from /proc/net/arp
#[derive(Debug, Serialize, Clone, Default)]
pub struct NetSockStat {
    pub used: Option<isize>,
    pub protocols: Vec<NetSockStatProtocol>,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct NetSockStatProtocol {
    pub protocol: String,
    pub inuse: isize,
    pub orphan: Option<isize>,
    pub tw: Option<isize>,
    pub alloc: Option<isize>,
    pub mem: Option<isize>,
    pub memory: Option<isize>,
}

impl NetSockStat {
    fn new() -> Self {
        Default::default()
    }
}

impl NetSockStatProtocol {
    fn new() -> Self {
        Default::default()
    }
}

/// collects network device information
/// # Example
/// ```
/// use procsys::net_sockstat;
///
/// let net_sockstat_info = net_sockstat::collect().expect("network sockstat information");
/// let json_output = serde_json::to_string_pretty(&net_sockstat_info).unwrap();
/// println!("sockstat:\n{}", json_output);
///
/// let net_sockstat6_info = net_sockstat::collect6().expect("network sockstat6 information");
/// let json_output = serde_json::to_string_pretty(&net_sockstat6_info).unwrap();
/// println!("sockstat6:\n{}", json_output);
/// ```
pub fn collect() -> CollectResult<NetSockStat> {
    collect_from("/proc/net/sockstat")
}

pub fn collect6() -> CollectResult<NetSockStat> {
    collect_from("/proc/net/sockstat6")
}

fn collect_from(filename: &str) -> CollectResult<NetSockStat> {
    let mut net_sockstat_info = NetSockStat::new();

    for line in utils::read_file_lines(filename)? {
        let fields: Vec<&str> = line.trim().split(':').filter(|s| !s.is_empty()).collect();
        if fields.len() != 2 {
            continue;
        }

        if fields[0].is_empty() || fields[1].is_empty() {
            continue;
        }

        if fields[0] == "sockets" {
            let socket = fields[1].trim().replace("used", "");
            match socket.trim().parse::<isize>() {
                Ok(sv) => net_sockstat_info.used = Some(sv),
                Err(_err) => net_sockstat_info.used = None,
            }
        } else {
            let mut net_sockstat_protocol = NetSockStatProtocol::new();
            let value_fields: Vec<&str> = fields[1]
                .trim()
                .split(' ')
                .filter(|s| !s.is_empty())
                .collect();

            let mut value_field_index = 0;

            while value_field_index < value_fields.len() {
                let val_item = value_fields[value_field_index];

                value_field_index += 1;

                if value_field_index >= value_fields.len() {
                    break;
                }

                let val_value = value_fields[value_field_index];

                value_field_index += 1;

                match val_item {
                    "inuse" => {
                        net_sockstat_protocol.inuse = utils::convert_str_to_isize(val_value)?;
                    }
                    "orphan" => {
                        net_sockstat_protocol.orphan =
                            Some(utils::convert_str_to_isize(val_value)?);
                    }
                    "tw" => {
                        net_sockstat_protocol.tw = Some(utils::convert_str_to_isize(val_value)?);
                    }
                    "alloc" => {
                        net_sockstat_protocol.alloc = Some(utils::convert_str_to_isize(val_value)?);
                    }
                    "mem" => {
                        net_sockstat_protocol.mem = Some(utils::convert_str_to_isize(val_value)?);
                    }
                    "memory" => {
                        net_sockstat_protocol.memory = Some(utils::convert_str_to_isize(val_value)?)
                    }
                    _ => {}
                }
            }

            net_sockstat_protocol.protocol = fields[0].to_string();

            net_sockstat_info.protocols.push(net_sockstat_protocol);
        }
    }

    Ok(net_sockstat_info)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sockstat_info() {
        let sockstat = collect_from("test_data/fixtures/proc/net/sockstat")
            .expect("sockstat entries information");

        assert_eq!(sockstat.used, Some(1602));

        for protocol in sockstat.protocols {
            match protocol.protocol.as_str() {
                "TCP" => {
                    assert_eq!(protocol.inuse, 35);
                    assert_eq!(protocol.orphan, Some(0));
                    assert_eq!(protocol.tw, Some(4));
                    assert_eq!(protocol.alloc, Some(59));
                    assert_eq!(protocol.mem, Some(22));
                    assert_eq!(protocol.memory, None);
                }
                "UDP" => {
                    assert_eq!(protocol.inuse, 12);
                    assert_eq!(protocol.orphan, None);
                    assert_eq!(protocol.tw, None);
                    assert_eq!(protocol.alloc, None);
                    assert_eq!(protocol.mem, Some(62));
                    assert_eq!(protocol.memory, None);
                }
                "UDPLITE" => {
                    assert_eq!(protocol.inuse, 0);
                    assert_eq!(protocol.orphan, None);
                    assert_eq!(protocol.tw, None);
                    assert_eq!(protocol.alloc, None);
                    assert_eq!(protocol.mem, None);
                    assert_eq!(protocol.memory, None);
                }
                "RAW" => {
                    assert_eq!(protocol.inuse, 0);
                    assert_eq!(protocol.orphan, None);
                    assert_eq!(protocol.tw, None);
                    assert_eq!(protocol.alloc, None);
                    assert_eq!(protocol.mem, None);
                    assert_eq!(protocol.memory, None);
                }
                "FRAG" => {
                    assert_eq!(protocol.inuse, 0);
                    assert_eq!(protocol.orphan, None);
                    assert_eq!(protocol.tw, None);
                    assert_eq!(protocol.alloc, None);
                    assert_eq!(protocol.mem, None);
                    assert_eq!(protocol.memory, Some(0));
                }
                _ => panic!("invalid protocol: {}", protocol.protocol),
            }
        }

        let sockstat6 = collect_from("test_data/fixtures/proc/net/sockstat6")
            .expect("sockstat entries information");

        assert_eq!(sockstat6.used, None);

        for protocol in sockstat6.protocols {
            match protocol.protocol.as_str() {
                "TCP6" => {
                    assert_eq!(protocol.inuse, 17);
                    assert_eq!(protocol.orphan, None);
                    assert_eq!(protocol.tw, None);
                    assert_eq!(protocol.alloc, None);
                    assert_eq!(protocol.mem, None);
                    assert_eq!(protocol.memory, None);
                }
                "UDP6" => {
                    assert_eq!(protocol.inuse, 9);
                    assert_eq!(protocol.orphan, None);
                    assert_eq!(protocol.tw, None);
                    assert_eq!(protocol.alloc, None);
                    assert_eq!(protocol.mem, None);
                    assert_eq!(protocol.memory, None);
                }
                "UDPLITE6" => {
                    assert_eq!(protocol.inuse, 0);
                    assert_eq!(protocol.orphan, None);
                    assert_eq!(protocol.tw, None);
                    assert_eq!(protocol.alloc, None);
                    assert_eq!(protocol.mem, None);
                    assert_eq!(protocol.memory, None);
                }
                "RAW6" => {
                    assert_eq!(protocol.inuse, 1);
                    assert_eq!(protocol.orphan, None);
                    assert_eq!(protocol.tw, None);
                    assert_eq!(protocol.alloc, None);
                    assert_eq!(protocol.mem, None);
                    assert_eq!(protocol.memory, None);
                }
                "FRAG6" => {
                    assert_eq!(protocol.inuse, 0);
                    assert_eq!(protocol.orphan, None);
                    assert_eq!(protocol.tw, None);
                    assert_eq!(protocol.alloc, None);
                    assert_eq!(protocol.mem, None);
                    assert_eq!(protocol.memory, Some(0));
                }
                _ => panic!("invalid protocol: {}", protocol.protocol),
            }
        }
    }
}
