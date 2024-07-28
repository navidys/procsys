use serde::Serialize;

use crate::{
    error::{CollectResult, MetricError},
    utils,
};

/// NetDev contains a network device information parsed from /proc/net/dev
#[derive(Debug, Serialize, Clone, Default)]
pub struct NetDev {
    pub name: String,
    pub rx_bytes: u64,
    pub rx_packets: u64,
    pub rx_errors: u64,
    pub rx_dropped: u64,
    pub rx_fifo: u64,
    pub rx_frame: u64,
    pub rx_compressed: u64,
    pub rx_multicast: u64,
    pub tx_bytes: u64,
    pub tx_packets: u64,
    pub tx_errors: u64,
    pub tx_dropped: u64,
    pub tx_fifo: u64,
    pub tx_collisions: u64,
    pub tx_carrier: u64,
    pub tx_compressed: u64,
}

impl NetDev {
    fn new() -> Self {
        Default::default()
    }
}

/// collects network device information
/// # Example
/// ```
/// use procsys::net_dev;
///
/// let net_devices = net_dev::collect().expect("network devices");
/// let json_output = serde_json::to_string_pretty(&net_devices).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<Vec<NetDev>> {
    let mut net_devices = Vec::new();

    let mut line_index = 0;

    for line in utils::read_file_lines("/proc/net/dev")? {
        line_index += 1;

        if line_index <= 2 {
            continue;
        }

        let fields: Vec<&str> = line.trim().split(' ').filter(|s| !s.is_empty()).collect();

        if fields.len() != 17 {
            return Err(MetricError::InvalidFieldNumberError(
                "network".to_string(),
                fields.len(),
                line,
            ));
        }

        let mut net_device = NetDev::new();
        net_device.name = fields[0].trim_matches(':').to_string();
        net_device.rx_bytes = fields[1].parse::<u64>().unwrap_or_default();
        net_device.rx_packets = fields[2].parse::<u64>().unwrap_or_default();
        net_device.rx_errors = fields[3].parse::<u64>().unwrap_or_default();
        net_device.rx_dropped = fields[4].parse::<u64>().unwrap_or_default();
        net_device.rx_fifo = fields[5].parse::<u64>().unwrap_or_default();
        net_device.rx_frame = fields[6].parse::<u64>().unwrap_or_default();
        net_device.rx_compressed = fields[7].parse::<u64>().unwrap_or_default();
        net_device.rx_multicast = fields[8].parse::<u64>().unwrap_or_default();
        net_device.tx_bytes = fields[9].parse::<u64>().unwrap_or_default();
        net_device.tx_packets = fields[10].parse::<u64>().unwrap_or_default();
        net_device.tx_errors = fields[11].parse::<u64>().unwrap_or_default();
        net_device.tx_dropped = fields[12].parse::<u64>().unwrap_or_default();
        net_device.tx_fifo = fields[13].parse::<u64>().unwrap_or_default();
        net_device.tx_collisions = fields[14].parse::<u64>().unwrap_or_default();
        net_device.tx_carrier = fields[15].parse::<u64>().unwrap_or_default();
        net_device.tx_compressed = fields[16].parse::<u64>().unwrap_or_default();

        net_devices.push(net_device);

        line_index += 1;
    }

    Ok(net_devices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn net_devices() {
        let minvalue = 0;
        let ndevices = collect().expect("collecting network devices");

        for net_dev in ndevices {
            assert!(!net_dev.name.is_empty());
            assert!(net_dev.rx_bytes.ge(&minvalue));
            assert!(net_dev.rx_packets.ge(&minvalue));
            assert!(net_dev.rx_errors.ge(&minvalue));
            assert!(net_dev.rx_dropped.ge(&minvalue));
            assert!(net_dev.rx_fifo.ge(&minvalue));
            assert!(net_dev.rx_frame.ge(&minvalue));
            assert!(net_dev.rx_compressed.ge(&minvalue));
            assert!(net_dev.rx_multicast.ge(&minvalue));
            assert!(net_dev.tx_bytes.ge(&minvalue));
            assert!(net_dev.tx_packets.ge(&minvalue));
            assert!(net_dev.tx_errors.ge(&minvalue));
            assert!(net_dev.tx_dropped.ge(&minvalue));
            assert!(net_dev.tx_fifo.ge(&minvalue));
            assert!(net_dev.tx_collisions.ge(&minvalue));
            assert!(net_dev.tx_carrier.ge(&minvalue));
            assert!(net_dev.tx_compressed.ge(&minvalue));
        }
    }
}
