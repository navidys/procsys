use crate::{error::CollectResult, utils};
use serde::Serialize;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// InfiniBandDevice contains info from files in /sys/class/infiniband for a
/// single InfiniBand device.
#[derive(Debug, Serialize, Clone, Default)]
pub struct InfiniBandDevice {
    pub name: String,
    pub board_id: String,         // /sys/class/infiniband/<Name>/board_id
    pub firmware_version: String, // /sys/class/infiniband/<Name>/fw_ver
    pub hca_type: String,         // /sys/class/infiniband/<Name>/hca_type
    pub ports: HashMap<u64, InfiniBandPort>,
}

/// InfiniBandPort contains info from files in
/// /sys/class/infiniband/<Name>/ports/<Port>
/// for a single port of one InfiniBand device.
#[derive(Debug, Serialize, Clone, Default)]
pub struct InfiniBandPort {
    pub name: String,
    pub port: u64,
    pub state: String, // String representation from /sys/class/infiniband/<Name>/ports/<Port>/state
    pub state_id: u64, // ID from /sys/class/infiniband/<Name>/ports/<Port>/state
    pub phys_state: String, // String representation from /sys/class/infiniband/<Name>/ports/<Port>/phys_state
    pub phys_state_id: u64, // String representation from /sys/class/infiniband/<Name>/ports/<Port>/phys_state
    pub rate: u64,          // in bytes/second from /sys/class/infiniband/<Name>/ports/<Port>/rate
    pub counters: InfiniBandCounters,
    pub hw_counters: InfiniBandHwCounters,
}

/// InfiniBandHwCounters contains counter value from files in
/// /sys/class/infiniband/<Name>/ports/<Port>/hw_counters
/// for a single port of one InfiniBand device.
#[derive(Debug, Serialize, Clone, Default)]
pub struct InfiniBandHwCounters {
    pub duplicate_request: Option<u64>,
    pub implied_nak_seq_err: Option<u64>,
    pub lifespan: Option<u64>,
    pub local_ack_timeout_err: Option<u64>,
    pub np_cnp_sent: Option<u64>,
    pub np_ecn_marked_roce_packets: Option<u64>,
    pub out_of_buffer: Option<u64>,
    pub out_of_sequence: Option<u64>,
    pub packet_seq_err: Option<u64>,
    pub req_cqe_error: Option<u64>,
    pub req_cqe_flush_error: Option<u64>,
    pub req_remote_access_errors: Option<u64>,
    pub req_remote_invalid_request: Option<u64>,
    pub resp_cqe_error: Option<u64>,
    pub resp_cqe_flush_error: Option<u64>,
    pub resp_local_length_error: Option<u64>,
    pub resp_remote_access_errors: Option<u64>,
    pub rnr_nak_retry_err: Option<u64>,
    pub roce_adp_retrans: Option<u64>,
    pub roce_adp_retrans_to: Option<u64>,
    pub roce_slow_restart: Option<u64>,
    pub roce_slow_restart_cnps: Option<u64>,
    pub roce_slow_restart_trans: Option<u64>,
    pub rp_cnp_handled: Option<u64>,
    pub rp_cnp_ignored: Option<u64>,
    pub rx_atomic_requests: Option<u64>,
    pub rx_dct_connect: Option<u64>,
    pub rx_icrc_encapsulated: Option<u64>,
    pub rx_read_requests: Option<u64>,
    pub rx_write_requests: Option<u64>,
}

/// InfiniBandCounters contains counter values from files in
/// /sys/class/infiniband/<Name>/ports/<Port>/counters or
/// /sys/class/infiniband/<Name>/ports/<Port>/counters_ext
/// for a single port of one InfiniBand device.
#[derive(Debug, Serialize, Clone, Default)]
pub struct InfiniBandCounters {
    pub legacy_port_multicast_rcv_packets: Option<u64>,
    pub legacy_port_multicast_xmit_packets: Option<u64>,
    pub legacy_port_rcv_data_64: Option<u64>,
    pub legacy_port_rcv_packets_64: Option<u64>,
    pub legacy_port_unicast_rcv_packets: Option<u64>,
    pub legacy_port_unicast_xmit_packets: Option<u64>,
    pub legacy_port_xmit_data_64: Option<u64>,
    pub legacy_port_xmit_packets_64: Option<u64>,

    pub excessive_buffer_overrun_errors: Option<u64>,
    pub link_downed: Option<u64>,
    pub link_error_recovery: Option<u64>,
    pub local_link_integrity_errors: Option<u64>,
    pub multicast_rcv_packets: Option<u64>,
    pub multicast_xmit_packets: Option<u64>,
    pub port_rcv_constraint_errors: Option<u64>,
    pub port_rcv_data: Option<u64>,
    pub port_rcv_discards: Option<u64>,
    pub port_rcv_errors: Option<u64>,
    pub port_rcv_packets: Option<u64>,
    pub port_rcv_remote_physical_errors: Option<u64>,
    pub port_rcv_switch_relay_errors: Option<u64>,
    pub port_xmit_constraint_errors: Option<u64>,
    pub port_xmit_data: Option<u64>,
    pub port_xmit_discards: Option<u64>,
    pub port_xmit_packets: Option<u64>,
    pub port_xmit_wait: Option<u64>,
    pub symbol_error: Option<u64>,
    pub unicast_rcv_packets: Option<u64>,
    pub unicast_xmit_packets: Option<u64>,
    pub vl15_dropped: Option<u64>,
}

impl InfiniBandDevice {
    fn new() -> Self {
        Default::default()
    }
}

impl InfiniBandPort {
    fn new() -> Self {
        Default::default()
    }
}

impl InfiniBandHwCounters {
    fn new() -> Self {
        Default::default()
    }
}

impl InfiniBandCounters {
    fn new() -> Self {
        Default::default()
    }
}

/// collects the the nvme devices information
/// # Example
/// ```
/// use procsys::sysfs::class_infiniband;
///
/// let infiniband_devices = class_infiniband::collect().expect("infiniband information");
/// let json_output = serde_json::to_string_pretty(&infiniband_devices).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<HashMap<String, InfiniBandDevice>> {
    collect_from("test_data/fixtures/sys/class/infiniband/")
}

fn collect_from(filename: &str) -> CollectResult<HashMap<String, InfiniBandDevice>> {
    let mut infiniband_devices: HashMap<String, InfiniBandDevice> = HashMap::new();

    let infi_devices_path = PathBuf::from(filename);
    for infi_device in utils::list_dir_content(&infi_devices_path, "", "infiniband") {
        let mut new_device_path = infi_devices_path.clone();
        new_device_path.push(&infi_device);

        let mut new_device = InfiniBandDevice::new();

        // firmware version
        new_device.firmware_version =
            utils::collect_info_string("fw_ver", &new_device_path)?.unwrap_or_default();

        // board id
        new_device.board_id =
            utils::collect_info_string("board_id", &new_device_path)?.unwrap_or_default();

        // hca type
        new_device.hca_type =
            utils::collect_info_string("hca_type", &new_device_path)?.unwrap_or_default();

        new_device.name = infi_device.clone();

        new_device.ports = collect_infiniband_ports(&infi_device, &infi_devices_path)?;

        infiniband_devices.insert(infi_device, new_device);
    }

    Ok(infiniband_devices)
}

fn collect_infiniband_ports(
    name: &str,
    path: &Path,
) -> CollectResult<HashMap<u64, InfiniBandPort>> {
    let mut infini_ports: HashMap<u64, InfiniBandPort> = HashMap::new();

    let mut ports_path = path.to_path_buf();
    ports_path.push(name);
    ports_path.push("ports");

    for port in utils::list_dir_content(&ports_path, "", "ports") {
        let mut infini_port_path = ports_path.clone();
        infini_port_path.push(&port);

        let mut infini_port = InfiniBandPort::new();

        infini_port.name = name.to_string();

        let port_number = utils::convert_str_to_u64(&port)?;
        infini_port.port = port_number;

        // port state and ID
        let state = utils::collect_info_string("state", &infini_port_path)?.unwrap_or_default();
        if !state.is_empty() {
            let state_fields: Vec<&str> =
                state.trim().split(':').filter(|s| !s.is_empty()).collect();
            if state_fields.len() == 2 {
                infini_port.state_id = utils::convert_str_to_u64(state_fields[0])?;
                infini_port.state = state_fields[1].trim().to_string();
            }
        }

        // port phys state and state ID
        let phys_state =
            utils::collect_info_string("phys_state", &infini_port_path)?.unwrap_or_default();
        if !phys_state.is_empty() {
            let phys_state_fields: Vec<&str> = phys_state
                .trim()
                .split(':')
                .filter(|s| !s.is_empty())
                .collect();

            if phys_state_fields.len() == 2 {
                infini_port.phys_state_id = utils::convert_str_to_u64(phys_state_fields[0])?;
                infini_port.phys_state = phys_state_fields[1].trim().to_string();
            }
        }

        // rate
        let rate = utils::collect_info_string("rate", &infini_port_path)?.unwrap_or_default();
        if !rate.is_empty() {
            let rate_fields: Vec<&str> = rate.trim().split(' ').filter(|s| !s.is_empty()).collect();
            if rate_fields.len() >= 2 {
                let rategb = utils::convert_str_to_u64(rate_fields[0])?;

                // Convert Gb/s into bytes/s
                infini_port.rate = rategb * 125000000;
            }
        }

        infini_port.counters = collect_infiniband_port_counters(&infini_port_path)?;
        infini_port.hw_counters = collect_infiniband_port_hwcounters(&infini_port_path)?;

        infini_ports.insert(port_number, infini_port);
    }

    Ok(infini_ports)
}

fn collect_infiniband_port_hwcounters(path: &Path) -> CollectResult<InfiniBandHwCounters> {
    let mut hwcounters = InfiniBandHwCounters::new();
    let mut hwcounters_path = path.to_path_buf();
    hwcounters_path.push("hw_counters");

    hwcounters.duplicate_request = utils::collect_info_u64("duplicate_request", &hwcounters_path)?;
    hwcounters.implied_nak_seq_err =
        utils::collect_info_u64("implied_nak_seq_err", &hwcounters_path)?;
    hwcounters.lifespan = utils::collect_info_u64("lifespan", &hwcounters_path)?;
    hwcounters.local_ack_timeout_err =
        utils::collect_info_u64("local_ack_timeout_err", &hwcounters_path)?;
    hwcounters.np_cnp_sent = utils::collect_info_u64("np_cnp_sent", &hwcounters_path)?;
    hwcounters.np_ecn_marked_roce_packets =
        utils::collect_info_u64("np_ecn_marked_roce_packets", &hwcounters_path)?;
    hwcounters.out_of_buffer = utils::collect_info_u64("out_of_buffer", &hwcounters_path)?;
    hwcounters.out_of_sequence = utils::collect_info_u64("out_of_sequence", &hwcounters_path)?;
    hwcounters.packet_seq_err = utils::collect_info_u64("packet_seq_err", &hwcounters_path)?;
    hwcounters.req_cqe_error = utils::collect_info_u64("req_cqe_error", &hwcounters_path)?;
    hwcounters.req_cqe_flush_error =
        utils::collect_info_u64("req_cqe_flush_error", &hwcounters_path)?;
    hwcounters.req_remote_access_errors =
        utils::collect_info_u64("req_remote_access_errors", &hwcounters_path)?;
    hwcounters.req_remote_invalid_request =
        utils::collect_info_u64("req_remote_invalid_request", &hwcounters_path)?;
    hwcounters.resp_cqe_error = utils::collect_info_u64("resp_cqe_error", &hwcounters_path)?;
    hwcounters.resp_cqe_flush_error =
        utils::collect_info_u64("resp_cqe_flush_error", &hwcounters_path)?;
    hwcounters.resp_local_length_error =
        utils::collect_info_u64("resp_local_length_error", &hwcounters_path)?;
    hwcounters.resp_remote_access_errors =
        utils::collect_info_u64("resp_remote_access_errors", &hwcounters_path)?;
    hwcounters.rnr_nak_retry_err = utils::collect_info_u64("rnr_nak_retry_err", &hwcounters_path)?;
    hwcounters.roce_adp_retrans = utils::collect_info_u64("roce_adp_retrans", &hwcounters_path)?;
    hwcounters.roce_adp_retrans_to =
        utils::collect_info_u64("roce_adp_retrans_to", &hwcounters_path)?;
    hwcounters.roce_slow_restart = utils::collect_info_u64("roce_slow_restart", &hwcounters_path)?;
    hwcounters.roce_slow_restart_cnps =
        utils::collect_info_u64("roce_slow_restart_cnps", &hwcounters_path)?;
    hwcounters.roce_slow_restart_trans =
        utils::collect_info_u64("roce_slow_restart_trans", &hwcounters_path)?;
    hwcounters.rp_cnp_handled = utils::collect_info_u64("rp_cnp_handled", &hwcounters_path)?;
    hwcounters.rp_cnp_ignored = utils::collect_info_u64("rp_cnp_ignored", &hwcounters_path)?;
    hwcounters.rx_atomic_requests =
        utils::collect_info_u64("rx_atomic_requests", &hwcounters_path)?;
    hwcounters.rx_dct_connect = utils::collect_info_u64("rx_dct_connect", &hwcounters_path)?;
    hwcounters.rx_icrc_encapsulated =
        utils::collect_info_u64("rx_icrc_encapsulated", &hwcounters_path)?;
    hwcounters.rx_read_requests = utils::collect_info_u64("rx_read_requests", &hwcounters_path)?;
    hwcounters.rx_write_requests = utils::collect_info_u64("rx_write_requests", &hwcounters_path)?;
    Ok(hwcounters)
}

fn collect_infiniband_port_counters(path: &Path) -> CollectResult<InfiniBandCounters> {
    let mut counters = InfiniBandCounters::new();

    let mut counters_ext_path = path.to_path_buf();
    counters_ext_path.push("counters_ext");

    let mut counters_path = path.to_path_buf();
    counters_path.push("counters");

    counters.legacy_port_multicast_rcv_packets =
        utils::collect_info_u64("port_multicast_rcv_packets", &counters_ext_path)?;
    counters.legacy_port_multicast_xmit_packets =
        utils::collect_info_u64("port_multicast_xmit_packets", &counters_ext_path)?;
    counters.legacy_port_rcv_data_64 =
        utils::collect_info_u64("_port_rcv_data_64", &counters_ext_path)?;
    counters.legacy_port_rcv_packets_64 =
        utils::collect_info_u64("port_rcv_packets_64", &counters_ext_path)?;
    counters.legacy_port_unicast_rcv_packets =
        utils::collect_info_u64("port_unicast_rcv_packets", &counters_ext_path)?;
    counters.legacy_port_unicast_xmit_packets =
        utils::collect_info_u64("port_unicast_xmit_packets", &counters_ext_path)?;
    counters.legacy_port_xmit_data_64 =
        utils::collect_info_u64("port_xmit_data_64", &counters_ext_path)?;
    counters.legacy_port_xmit_packets_64 =
        utils::collect_info_u64("port_xmit_packets_64", &counters_ext_path)?;

    counters.excessive_buffer_overrun_errors =
        utils::collect_info_u64("excessive_buffer_overrun_errors", &counters_path)?;
    counters.link_downed = utils::collect_info_u64("link_downed", &counters_path)?;
    counters.link_error_recovery = utils::collect_info_u64("link_error_recovery", &counters_path)?;
    counters.local_link_integrity_errors =
        utils::collect_info_u64("local_link_integrity_errors", &counters_path)?;
    counters.multicast_rcv_packets =
        utils::collect_info_u64("multicast_rcv_packets", &counters_path)?;
    counters.multicast_xmit_packets =
        utils::collect_info_u64("multicast_xmit_packets", &counters_path)?;
    counters.port_rcv_constraint_errors =
        utils::collect_info_u64("port_rcv_constraint_errors", &counters_path)?;
    counters.port_rcv_data = utils::collect_info_u64("port_rcv_data", &counters_path)?;
    counters.port_rcv_discards = utils::collect_info_u64("port_rcv_discards", &counters_path)?;
    counters.port_rcv_errors = utils::collect_info_u64("port_rcv_errors", &counters_path)?;
    counters.port_rcv_packets = utils::collect_info_u64("port_rcv_packets", &counters_path)?;
    counters.port_rcv_remote_physical_errors =
        utils::collect_info_u64("port_rcv_remote_physical_errors", &counters_path)?;
    counters.port_rcv_switch_relay_errors =
        utils::collect_info_u64("port_rcv_switch_relay_errors", &counters_path)?;
    counters.port_xmit_constraint_errors =
        utils::collect_info_u64("port_xmit_constraint_errors", &counters_path)?;
    counters.port_xmit_data = utils::collect_info_u64("port_xmit_data", &counters_path)?;
    counters.port_xmit_discards = utils::collect_info_u64("port_xmit_discards", &counters_path)?;
    counters.port_xmit_packets = utils::collect_info_u64("port_xmit_packets", &counters_path)?;
    counters.port_xmit_wait = utils::collect_info_u64("port_xmit_wait", &counters_path)?;
    counters.symbol_error = utils::collect_info_u64("symbol_error", &counters_path)?;
    counters.unicast_rcv_packets = utils::collect_info_u64("unicast_rcv_packets", &counters_path)?;
    counters.unicast_xmit_packets =
        utils::collect_info_u64("unicast_xmit_packets", &counters_path)?;
    counters.excessive_buffer_overrun_errors =
        utils::collect_info_u64("excessive_buffer_overrun_errors", &counters_path)?;
    counters.excessive_buffer_overrun_errors =
        utils::collect_info_u64("excessive_buffer_overrun_errors", &counters_path)?;
    counters.excessive_buffer_overrun_errors =
        utils::collect_info_u64("excessive_buffer_overrun_errors", &counters_path)?;
    counters.vl15_dropped = utils::collect_info_u64("VL15_dropped", &counters_path)?;

    Ok(counters)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn infiniband_device_information() {
        let infini_devices = collect_from("test_data/fixtures/sys/class/infiniband/")
            .expect("collecting infiniband information");

        for (name, device) in infini_devices {
            match name.as_str() {
                "hfi1_0" => {
                    assert_eq!(device.name, "hfi1_0");
                    assert_eq!(
                        device.board_id,
                        "HPE 100Gb 1-port OP101 QSFP28 x16 PCIe Gen3 with Intel Omni-Path Adapter"
                    );
                    assert_eq!(device.firmware_version, "1.27.0");
                    assert_eq!(device.hca_type, "");

                    for (port, device_port) in device.ports {
                        match port {
                            1 => {
                                assert_eq!(device_port.name, "hfi1_0");
                                assert_eq!(device_port.port, 1);
                                assert_eq!(device_port.state_id, 4);
                                assert_eq!(device_port.state, "ACTIVE");
                                assert_eq!(device_port.phys_state_id, 5);
                                assert_eq!(device_port.phys_state, "LinkUp");
                                assert_eq!(device_port.rate, 12500000000);

                                // counters
                                let counters = device_port.counters;
                                assert_eq!(counters.legacy_port_multicast_rcv_packets, None);
                                assert_eq!(counters.legacy_port_multicast_xmit_packets, None);
                                assert_eq!(counters.legacy_port_rcv_data_64, None);
                                assert_eq!(counters.legacy_port_rcv_packets_64, None);
                                assert_eq!(counters.legacy_port_unicast_rcv_packets, None);
                                assert_eq!(counters.legacy_port_unicast_xmit_packets, None);
                                assert_eq!(counters.legacy_port_xmit_data_64, None);
                                assert_eq!(counters.legacy_port_xmit_packets_64, None);

                                assert_eq!(counters.excessive_buffer_overrun_errors, Some(0));
                                assert_eq!(counters.link_downed, Some(0));
                                assert_eq!(counters.link_error_recovery, Some(0));
                                assert_eq!(counters.local_link_integrity_errors, Some(0));
                                assert_eq!(counters.multicast_rcv_packets, None);
                                assert_eq!(counters.multicast_xmit_packets, None);
                                assert_eq!(counters.port_rcv_constraint_errors, Some(0));
                                assert_eq!(counters.port_rcv_data, Some(345091702026));
                                assert_eq!(counters.port_rcv_discards, None);
                                assert_eq!(counters.port_rcv_errors, Some(0));
                                assert_eq!(counters.port_rcv_packets, Some(638036947));
                                assert_eq!(counters.port_rcv_remote_physical_errors, Some(0));
                                assert_eq!(counters.port_rcv_switch_relay_errors, Some(0));
                                assert_eq!(counters.port_xmit_constraint_errors, Some(0));
                                assert_eq!(counters.port_xmit_data, Some(273558326543));
                                assert_eq!(counters.port_xmit_discards, Some(0));
                                assert_eq!(counters.port_xmit_packets, Some(568318856));
                                assert_eq!(counters.port_xmit_wait, Some(0));
                                assert_eq!(counters.symbol_error, Some(0));
                                assert_eq!(counters.unicast_rcv_packets, None);
                                assert_eq!(counters.unicast_xmit_packets, None);
                                assert_eq!(counters.vl15_dropped, Some(0));

                                // hw_counters
                                let hwcounters = device_port.hw_counters;
                                assert_eq!(hwcounters.duplicate_request, None);
                                assert_eq!(hwcounters.implied_nak_seq_err, None);
                                assert_eq!(hwcounters.lifespan, None);
                                assert_eq!(hwcounters.local_ack_timeout_err, None);
                                assert_eq!(hwcounters.np_cnp_sent, None);
                                assert_eq!(hwcounters.np_ecn_marked_roce_packets, None);
                                assert_eq!(hwcounters.out_of_buffer, None);
                                assert_eq!(hwcounters.out_of_sequence, None);
                                assert_eq!(hwcounters.packet_seq_err, None);
                                assert_eq!(hwcounters.req_cqe_error, None);
                                assert_eq!(hwcounters.req_cqe_flush_error, None);
                                assert_eq!(hwcounters.req_remote_access_errors, None);
                                assert_eq!(hwcounters.req_remote_invalid_request, None);
                                assert_eq!(hwcounters.resp_cqe_error, None);
                                assert_eq!(hwcounters.resp_cqe_flush_error, None);
                                assert_eq!(hwcounters.resp_local_length_error, None);
                                assert_eq!(hwcounters.resp_remote_access_errors, None);
                                assert_eq!(hwcounters.rnr_nak_retry_err, None);
                                assert_eq!(hwcounters.roce_adp_retrans, None);
                                assert_eq!(hwcounters.roce_adp_retrans_to, None);
                                assert_eq!(hwcounters.roce_slow_restart, None);
                                assert_eq!(hwcounters.roce_slow_restart_cnps, None);
                                assert_eq!(hwcounters.roce_slow_restart_trans, None);
                                assert_eq!(hwcounters.rp_cnp_handled, None);
                                assert_eq!(hwcounters.rp_cnp_ignored, None);
                                assert_eq!(hwcounters.rx_atomic_requests, None);
                                assert_eq!(hwcounters.rx_dct_connect, None);
                                assert_eq!(hwcounters.rx_icrc_encapsulated, None);
                                assert_eq!(hwcounters.rx_read_requests, None);
                                assert_eq!(hwcounters.rx_write_requests, None);
                            }
                            _ => panic!("invalid device port number: {}", port),
                        }
                    }
                }
                "mlx4_0" => {
                    assert_eq!(device.name, "mlx4_0");
                    assert_eq!(device.board_id, "SM_1141000001000");
                    assert_eq!(device.firmware_version, "2.31.5050");
                    assert_eq!(device.hca_type, "MT4099");

                    for (port, device_port) in device.ports {
                        match port {
                            1 => {
                                assert_eq!(device_port.name, "mlx4_0");
                                assert_eq!(device_port.port, 1);
                                assert_eq!(device_port.state_id, 4);
                                assert_eq!(device_port.state, "ACTIVE");
                                assert_eq!(device_port.phys_state_id, 5);
                                assert_eq!(device_port.phys_state, "LinkUp");
                                assert_eq!(device_port.rate, 5000000000);

                                // counters
                                let counters = device_port.counters;
                                assert_eq!(counters.legacy_port_multicast_rcv_packets, None);
                                assert_eq!(counters.legacy_port_multicast_xmit_packets, None);
                                assert_eq!(counters.legacy_port_rcv_data_64, None);
                                assert_eq!(counters.legacy_port_rcv_packets_64, None);
                                assert_eq!(counters.legacy_port_unicast_rcv_packets, None);
                                assert_eq!(counters.legacy_port_unicast_xmit_packets, None);
                                assert_eq!(counters.legacy_port_xmit_data_64, None);
                                assert_eq!(counters.legacy_port_xmit_packets_64, None);

                                assert_eq!(counters.excessive_buffer_overrun_errors, Some(0));
                                assert_eq!(counters.link_downed, Some(0));
                                assert_eq!(counters.link_error_recovery, Some(0));
                                assert_eq!(counters.local_link_integrity_errors, Some(0));
                                assert_eq!(counters.multicast_rcv_packets, None);
                                assert_eq!(counters.multicast_xmit_packets, None);
                                assert_eq!(counters.port_rcv_constraint_errors, Some(0));
                                assert_eq!(counters.port_rcv_data, Some(2221223609));
                                assert_eq!(counters.port_rcv_discards, None);
                                assert_eq!(counters.port_rcv_errors, Some(0));
                                assert_eq!(counters.port_rcv_packets, Some(87169372));
                                assert_eq!(counters.port_rcv_remote_physical_errors, Some(0));
                                assert_eq!(counters.port_rcv_switch_relay_errors, Some(0));
                                assert_eq!(counters.port_xmit_constraint_errors, Some(0));
                                assert_eq!(counters.port_xmit_data, Some(26509113295));
                                assert_eq!(counters.port_xmit_discards, Some(0));
                                assert_eq!(counters.port_xmit_packets, Some(85734114));
                                assert_eq!(counters.port_xmit_wait, Some(3599));
                                assert_eq!(counters.symbol_error, Some(0));
                                assert_eq!(counters.unicast_rcv_packets, None);
                                assert_eq!(counters.unicast_xmit_packets, None);
                                assert_eq!(counters.vl15_dropped, Some(0));

                                // hw_counters
                                let hwcounters = device_port.hw_counters;
                                assert_eq!(hwcounters.duplicate_request, None);
                                assert_eq!(hwcounters.implied_nak_seq_err, None);
                                assert_eq!(hwcounters.lifespan, None);
                                assert_eq!(hwcounters.local_ack_timeout_err, None);
                                assert_eq!(hwcounters.np_cnp_sent, None);
                                assert_eq!(hwcounters.np_ecn_marked_roce_packets, None);
                                assert_eq!(hwcounters.out_of_buffer, None);
                                assert_eq!(hwcounters.out_of_sequence, None);
                                assert_eq!(hwcounters.packet_seq_err, None);
                                assert_eq!(hwcounters.req_cqe_error, None);
                                assert_eq!(hwcounters.req_cqe_flush_error, None);
                                assert_eq!(hwcounters.req_remote_access_errors, None);
                                assert_eq!(hwcounters.req_remote_invalid_request, None);
                                assert_eq!(hwcounters.resp_cqe_error, None);
                                assert_eq!(hwcounters.resp_cqe_flush_error, None);
                                assert_eq!(hwcounters.resp_local_length_error, None);
                                assert_eq!(hwcounters.resp_remote_access_errors, None);
                                assert_eq!(hwcounters.rnr_nak_retry_err, None);
                                assert_eq!(hwcounters.roce_adp_retrans, None);
                                assert_eq!(hwcounters.roce_adp_retrans_to, None);
                                assert_eq!(hwcounters.roce_slow_restart, None);
                                assert_eq!(hwcounters.roce_slow_restart_cnps, None);
                                assert_eq!(hwcounters.roce_slow_restart_trans, None);
                                assert_eq!(hwcounters.rp_cnp_handled, None);
                                assert_eq!(hwcounters.rp_cnp_ignored, None);
                                assert_eq!(hwcounters.rx_atomic_requests, None);
                                assert_eq!(hwcounters.rx_dct_connect, None);
                                assert_eq!(hwcounters.rx_icrc_encapsulated, None);
                                assert_eq!(hwcounters.rx_read_requests, None);
                                assert_eq!(hwcounters.rx_write_requests, None);
                            }
                            2 => {
                                assert_eq!(device_port.name, "mlx4_0");
                                assert_eq!(device_port.port, 2);
                                assert_eq!(device_port.state_id, 4);
                                assert_eq!(device_port.state, "ACTIVE");
                                assert_eq!(device_port.phys_state_id, 5);
                                assert_eq!(device_port.phys_state, "LinkUp");
                                assert_eq!(device_port.rate, 5000000000);

                                // counters
                                let counters = device_port.counters;
                                assert_eq!(counters.legacy_port_multicast_rcv_packets, None);
                                assert_eq!(counters.legacy_port_multicast_xmit_packets, None);
                                assert_eq!(counters.legacy_port_rcv_data_64, None);
                                assert_eq!(counters.legacy_port_rcv_packets_64, None);
                                assert_eq!(counters.legacy_port_unicast_rcv_packets, None);
                                assert_eq!(counters.legacy_port_unicast_xmit_packets, None);
                                assert_eq!(counters.legacy_port_xmit_data_64, None);
                                assert_eq!(counters.legacy_port_xmit_packets_64, None);

                                assert_eq!(counters.excessive_buffer_overrun_errors, Some(0));
                                assert_eq!(counters.link_downed, Some(0));
                                assert_eq!(counters.link_error_recovery, Some(0));
                                assert_eq!(counters.local_link_integrity_errors, Some(0));
                                assert_eq!(counters.multicast_rcv_packets, None);
                                assert_eq!(counters.multicast_xmit_packets, None);
                                assert_eq!(counters.port_rcv_constraint_errors, Some(0));
                                assert_eq!(counters.port_rcv_data, Some(2460436784));
                                assert_eq!(counters.port_rcv_discards, None);
                                assert_eq!(counters.port_rcv_errors, Some(0));
                                assert_eq!(counters.port_rcv_packets, Some(89332064));
                                assert_eq!(counters.port_rcv_remote_physical_errors, Some(0));
                                assert_eq!(counters.port_rcv_switch_relay_errors, Some(0));
                                assert_eq!(counters.port_xmit_constraint_errors, Some(0));
                                assert_eq!(counters.port_xmit_data, Some(26540356890));
                                assert_eq!(counters.port_xmit_discards, Some(0));
                                assert_eq!(counters.port_xmit_packets, Some(88622850));
                                assert_eq!(counters.port_xmit_wait, Some(3846));
                                assert_eq!(counters.symbol_error, Some(0));
                                assert_eq!(counters.unicast_rcv_packets, None);
                                assert_eq!(counters.unicast_xmit_packets, None);
                                assert_eq!(counters.vl15_dropped, Some(0));

                                // hw_counters
                                let hwcounters = device_port.hw_counters;
                                assert_eq!(hwcounters.duplicate_request, None);
                                assert_eq!(hwcounters.implied_nak_seq_err, None);
                                assert_eq!(hwcounters.lifespan, None);
                                assert_eq!(hwcounters.local_ack_timeout_err, None);
                                assert_eq!(hwcounters.np_cnp_sent, None);
                                assert_eq!(hwcounters.np_ecn_marked_roce_packets, None);
                                assert_eq!(hwcounters.out_of_buffer, None);
                                assert_eq!(hwcounters.out_of_sequence, None);
                                assert_eq!(hwcounters.packet_seq_err, None);
                                assert_eq!(hwcounters.req_cqe_error, None);
                                assert_eq!(hwcounters.req_cqe_flush_error, None);
                                assert_eq!(hwcounters.req_remote_access_errors, None);
                                assert_eq!(hwcounters.req_remote_invalid_request, None);
                                assert_eq!(hwcounters.resp_cqe_error, None);
                                assert_eq!(hwcounters.resp_cqe_flush_error, None);
                                assert_eq!(hwcounters.resp_local_length_error, None);
                                assert_eq!(hwcounters.resp_remote_access_errors, None);
                                assert_eq!(hwcounters.rnr_nak_retry_err, None);
                                assert_eq!(hwcounters.roce_adp_retrans, None);
                                assert_eq!(hwcounters.roce_adp_retrans_to, None);
                                assert_eq!(hwcounters.roce_slow_restart, None);
                                assert_eq!(hwcounters.roce_slow_restart_cnps, None);
                                assert_eq!(hwcounters.roce_slow_restart_trans, None);
                                assert_eq!(hwcounters.rp_cnp_handled, None);
                                assert_eq!(hwcounters.rp_cnp_ignored, None);
                                assert_eq!(hwcounters.rx_atomic_requests, None);
                                assert_eq!(hwcounters.rx_dct_connect, None);
                                assert_eq!(hwcounters.rx_icrc_encapsulated, None);
                                assert_eq!(hwcounters.rx_read_requests, None);
                                assert_eq!(hwcounters.rx_write_requests, None);
                            }
                            _ => panic!("invalid device port number: {}", port),
                        }
                    }
                }
                "mlx5_0" => {
                    assert_eq!(device.name, "mlx5_0");
                    assert_eq!(device.board_id, "SM_2001000001034");
                    assert_eq!(device.firmware_version, "14.28.2006");
                    assert_eq!(device.hca_type, "MT4118");

                    for (port, device_port) in device.ports {
                        match port {
                            1 => {
                                assert_eq!(device_port.name, "mlx5_0");
                                assert_eq!(device_port.port, 1);
                                assert_eq!(device_port.state_id, 4);
                                assert_eq!(device_port.state, "ACTIVE");
                                assert_eq!(device_port.phys_state_id, 4);
                                assert_eq!(device_port.phys_state, "ACTIVE");
                                assert_eq!(device_port.rate, 3125000000);

                                // counters
                                let counters = device_port.counters;
                                assert_eq!(counters.legacy_port_multicast_rcv_packets, None);
                                assert_eq!(counters.legacy_port_multicast_xmit_packets, None);
                                assert_eq!(counters.legacy_port_rcv_data_64, None);
                                assert_eq!(counters.legacy_port_rcv_packets_64, None);
                                assert_eq!(counters.legacy_port_unicast_rcv_packets, None);
                                assert_eq!(counters.legacy_port_unicast_xmit_packets, None);
                                assert_eq!(counters.legacy_port_xmit_data_64, None);
                                assert_eq!(counters.legacy_port_xmit_packets_64, None);

                                assert_eq!(counters.excessive_buffer_overrun_errors, Some(0));
                                assert_eq!(counters.link_downed, Some(0));
                                assert_eq!(counters.link_error_recovery, Some(0));
                                assert_eq!(counters.local_link_integrity_errors, Some(0));
                                assert_eq!(counters.multicast_rcv_packets, Some(0));
                                assert_eq!(counters.multicast_xmit_packets, Some(0));
                                assert_eq!(counters.port_rcv_constraint_errors, Some(0));
                                assert_eq!(counters.port_rcv_data, Some(18126345378));
                                assert_eq!(counters.port_rcv_discards, None);
                                assert_eq!(counters.port_rcv_errors, Some(0));
                                assert_eq!(counters.port_rcv_packets, Some(541889824));
                                assert_eq!(counters.port_rcv_remote_physical_errors, Some(0));
                                assert_eq!(counters.port_rcv_switch_relay_errors, Some(0));
                                assert_eq!(counters.port_xmit_constraint_errors, Some(0));
                                assert_eq!(counters.port_xmit_data, Some(2880761508848));
                                assert_eq!(counters.port_xmit_discards, Some(0));
                                assert_eq!(counters.port_xmit_packets, Some(10907922116));
                                assert_eq!(counters.port_xmit_wait, Some(0));
                                assert_eq!(counters.symbol_error, Some(0));
                                assert_eq!(counters.unicast_rcv_packets, Some(541889824));
                                assert_eq!(counters.unicast_xmit_packets, Some(10907922116));
                                assert_eq!(counters.vl15_dropped, Some(0));

                                // hw_counters
                                let hwcounters = device_port.hw_counters;
                                assert_eq!(hwcounters.duplicate_request, Some(41));
                                assert_eq!(hwcounters.implied_nak_seq_err, Some(0));
                                assert_eq!(hwcounters.lifespan, Some(10));
                                assert_eq!(hwcounters.local_ack_timeout_err, Some(131));
                                assert_eq!(hwcounters.np_cnp_sent, None);
                                assert_eq!(hwcounters.np_ecn_marked_roce_packets, None);
                                assert_eq!(hwcounters.out_of_buffer, Some(0));
                                assert_eq!(hwcounters.out_of_sequence, Some(1));
                                assert_eq!(hwcounters.packet_seq_err, Some(1));
                                assert_eq!(hwcounters.req_cqe_error, Some(3481));
                                assert_eq!(hwcounters.req_cqe_flush_error, Some(80));
                                assert_eq!(hwcounters.req_remote_access_errors, Some(0));
                                assert_eq!(hwcounters.req_remote_invalid_request, Some(0));
                                assert_eq!(hwcounters.resp_cqe_error, Some(8109));
                                assert_eq!(hwcounters.resp_cqe_flush_error, Some(4708));
                                assert_eq!(hwcounters.resp_local_length_error, Some(0));
                                assert_eq!(hwcounters.resp_remote_access_errors, Some(0));
                                assert_eq!(hwcounters.rnr_nak_retry_err, Some(0));
                                assert_eq!(hwcounters.roce_adp_retrans, Some(99));
                                assert_eq!(hwcounters.roce_adp_retrans_to, Some(4));
                                assert_eq!(hwcounters.roce_slow_restart, Some(0));
                                assert_eq!(hwcounters.roce_slow_restart_cnps, Some(131));
                                assert_eq!(hwcounters.roce_slow_restart_trans, Some(0));
                                assert_eq!(hwcounters.rp_cnp_handled, None);
                                assert_eq!(hwcounters.rp_cnp_ignored, None);
                                assert_eq!(hwcounters.rx_atomic_requests, Some(0));
                                assert_eq!(hwcounters.rx_dct_connect, Some(0));
                                assert_eq!(hwcounters.rx_icrc_encapsulated, None);
                                assert_eq!(hwcounters.rx_read_requests, Some(175528982));
                                assert_eq!(hwcounters.rx_write_requests, Some(742114));
                            }
                            _ => panic!("invalid device port number: {}", port),
                        }
                    }
                }
                _ => panic!("invalid infiniband device name: {}", name),
            }
        }
    }
}
