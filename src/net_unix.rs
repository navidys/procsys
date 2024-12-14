use serde::Serialize;

use crate::{
    error::{CollectResult, MetricError},
    utils,
};

/// NetUnix represents a line of /proc/net/unix
#[derive(Debug, Serialize, Clone, Default)]
pub struct NetUnix {
    pub kernel_ptr: String,
    pub ref_count: u64,
    pub flags: u64,
    pub ntype: u64,
    pub state: u64,
    pub inode: u64,
    pub path: Option<String>,
}

impl NetUnix {
    fn new() -> Self {
        Default::default()
    }
}

/// collects the network unix information
/// # Example
/// ```
/// use procsys::net_unix;
///
/// let netunix = net_unix::collect().expect("network unix information");
/// let json_output = serde_json::to_string_pretty(&netunix).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<Vec<NetUnix>> {
    collect_from("/proc/net/unix")
}

fn collect_from(filename: &str) -> CollectResult<Vec<NetUnix>> {
    let mut all_net_unix: Vec<NetUnix> = Vec::new();

    let mut line_index = 0;
    for line in utils::read_file_lines(filename)? {
        line_index += 1;

        if line_index <= 1 {
            continue;
        }

        let fields: Vec<&str> = line.trim().split(' ').filter(|s| !s.is_empty()).collect();

        if fields.len() < 7 {
            return Err(MetricError::InvalidFieldNumberError(
                "net unix".to_string(),
                fields.len(),
                line,
            ));
        }

        let mut net_unix = NetUnix::new();
        net_unix.kernel_ptr = fields[0].trim_matches(':').to_string();
        net_unix.ref_count = u64::from_str_radix(fields[1], 16).unwrap_or_default();
        net_unix.flags = u64::from_str_radix(fields[3], 16).unwrap_or_default();
        net_unix.ntype = u64::from_str_radix(fields[4], 16).unwrap_or_default();
        net_unix.state = u64::from_str_radix(fields[5], 16).unwrap_or_default();
        net_unix.inode = fields[6].parse::<u64>().unwrap_or_default();

        if fields.len() > 7 && !fields[7].is_empty() {
            net_unix.path = Some(fields[7].to_string())
        }

        all_net_unix.push(net_unix);
    }

    Ok(all_net_unix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn net_unix() {
        let all_net_unix = collect_from("test_data/fixtures/proc/net/unix")
            .expect("collecting network unix information");

        assert_eq!(all_net_unix.len(), 5);

        for netu in all_net_unix {
            match netu.kernel_ptr.as_str() {
                "0000000000000001" => {
                    assert_eq!(netu.ref_count, 2);
                    assert_eq!(netu.flags, 1 << 16);
                    assert_eq!(netu.ntype, 1);
                    assert_eq!(netu.state, 1);
                    assert_eq!(netu.inode, 3442596);
                    assert_eq!(
                        netu.path,
                        Some("/var/run/postgresql/.s.PGSQL.5432".to_string())
                    );
                }
                "0000000000000002" => {
                    assert_eq!(netu.ref_count, 10);
                    assert_eq!(netu.flags, 1 << 16);
                    assert_eq!(netu.ntype, 5);
                    assert_eq!(netu.state, 1);
                    assert_eq!(netu.inode, 10061);
                    assert_eq!(netu.path, Some("/run/udev/control".to_string()));
                }
                "0000000000000003" => {
                    assert_eq!(netu.ref_count, 7);
                    assert_eq!(netu.flags, 0);
                    assert_eq!(netu.ntype, 2);
                    assert_eq!(netu.state, 1);
                    assert_eq!(netu.inode, 12392);
                    assert_eq!(netu.path, Some("/dev/log".to_string()));
                }
                "0000000000000004" => {
                    assert_eq!(netu.ref_count, 3);
                    assert_eq!(netu.flags, 0);
                    assert_eq!(netu.ntype, 1);
                    assert_eq!(netu.state, 3);
                    assert_eq!(netu.inode, 4787297);
                    assert_eq!(
                        netu.path,
                        Some("/var/run/postgresql/.s.PGSQL.5432".to_string())
                    );
                }
                "0000000000000005" => {
                    assert_eq!(netu.ref_count, 3);
                    assert_eq!(netu.flags, 0);
                    assert_eq!(netu.ntype, 1);
                    assert_eq!(netu.state, 3);
                    assert_eq!(netu.inode, 5091797);
                    assert_eq!(netu.path, None);
                }
                _ => panic!("invalid network unix kernel ptr: {}", netu.kernel_ptr),
            }
        }
    }
}
