use serde::Serialize;

use crate::{
    error::{CollectResult, MetricError},
    utils,
};

/// NetProtocol represents a single line parsed from /proc/net/protocols
#[derive(Debug, Serialize, Clone, Default)]
pub struct NetProtocol {
    pub name: String,
    pub size: u64,
    pub sockets: i64,
    pub memory: i64,
    pub pressure: Option<bool>,
    pub max_header: u64,
    pub slab: bool,
    pub module_name: String,
    pub capabilities: NetProtocolCapabilities,
}

/// NetProtocolCapabilities contains a list of capabilities for each protocol
#[derive(Debug, Serialize, Clone, Default)]
pub struct NetProtocolCapabilities {
    pub close: bool,
    pub connect: bool,
    pub disconnect: bool,
    pub accept: bool,
    pub ioctl: bool,
    pub init: bool,
    pub destroy: bool,
    pub shutdown: bool,
    pub set_socketopt: bool,
    pub get_socketopt: bool,
    pub send_msg: bool,
    pub recv_msg: bool,
    /// kernel 6.9 does not include send page in anymore
    pub send_page: Option<bool>,
    pub bind: bool,
    pub backlog_rcv: bool,
    pub hash: bool,
    pub unhash: bool,
    pub get_port: bool,
    pub entry_memory_pressure: bool,
}

impl NetProtocol {
    fn new() -> Self {
        Default::default()
    }
}

impl NetProtocolCapabilities {
    fn new() -> Self {
        Default::default()
    }
}

/// collects the network protocols information
/// # Example
/// ```
/// use procsys::net_protocols;
///
/// let netprots = net_protocols::collect().expect("network protocols");
/// let json_output = serde_json::to_string_pretty(&netprots).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<Vec<NetProtocol>> {
    let mut netprotos: Vec<NetProtocol> = Vec::new();

    let netprotos_info: Vec<String> = utils::read_file_lines("/proc/net/protocols")?;
    let header: Vec<&str> = netprotos_info[0]
        .trim()
        .split(' ')
        .filter(|s| !s.is_empty())
        .collect();

    let sp_included = header[20] == "sp";

    for line in &netprotos_info[1..] {
        let net_info = line.to_owned();
        let net_info_fields: Vec<&str> = net_info
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect();

        if (sp_included && net_info_fields.len() < 27)
            || (!sp_included && net_info_fields.len() < 26)
        {
            return Err(MetricError::InvalidFieldNumberError(
                "net protocols".to_string(),
                net_info_fields.len(),
                line.to_owned(),
            ));
        }

        let mut net_proto = NetProtocol::new();
        net_proto.capabilities = NetProtocolCapabilities::new();

        net_proto.name = net_info_fields[0].to_string();
        net_proto.size = net_info_fields[1].parse::<u64>().unwrap_or_default();
        net_proto.sockets = net_info_fields[2].parse::<i64>().unwrap_or_default();
        net_proto.memory = net_info_fields[3].parse::<i64>().unwrap_or_default();

        if net_info_fields[4] != "NI" {
            net_proto.pressure = Some(net_info_fields[4] == "yes")
        }

        net_proto.max_header = net_info_fields[5].parse::<u64>().unwrap_or_default();
        net_proto.slab = net_info_fields[6] == "yes";
        net_proto.module_name = net_info_fields[7].to_string();

        // capabilities
        let default_y = "y";
        net_proto.capabilities.close = net_info_fields[8] == default_y;
        net_proto.capabilities.connect = net_info_fields[9] == default_y;
        net_proto.capabilities.disconnect = net_info_fields[10] == default_y;
        net_proto.capabilities.accept = net_info_fields[11] == default_y;
        net_proto.capabilities.ioctl = net_info_fields[12] == default_y;
        net_proto.capabilities.init = net_info_fields[13] == default_y;
        net_proto.capabilities.destroy = net_info_fields[14] == default_y;
        net_proto.capabilities.shutdown = net_info_fields[15] == default_y;
        net_proto.capabilities.set_socketopt = net_info_fields[16] == default_y;
        net_proto.capabilities.get_socketopt = net_info_fields[17] == default_y;
        net_proto.capabilities.send_msg = net_info_fields[18] == default_y;
        net_proto.capabilities.recv_msg = net_info_fields[19] == default_y;

        if sp_included {
            net_proto.capabilities.send_page = Some(net_info_fields[20] == default_y);
            net_proto.capabilities.bind = net_info_fields[21] == default_y;
            net_proto.capabilities.backlog_rcv = net_info_fields[22] == default_y;
            net_proto.capabilities.hash = net_info_fields[23] == default_y;
            net_proto.capabilities.unhash = net_info_fields[24] == default_y;
            net_proto.capabilities.get_port = net_info_fields[25] == default_y;
            net_proto.capabilities.entry_memory_pressure = net_info_fields[26] == default_y;
        } else {
            net_proto.capabilities.bind = net_info_fields[20] == default_y;
            net_proto.capabilities.backlog_rcv = net_info_fields[21] == default_y;
            net_proto.capabilities.hash = net_info_fields[22] == default_y;
            net_proto.capabilities.unhash = net_info_fields[23] == default_y;
            net_proto.capabilities.get_port = net_info_fields[24] == default_y;
            net_proto.capabilities.entry_memory_pressure = net_info_fields[25] == default_y;
        }

        netprotos.push(net_proto);
    }

    Ok(netprotos)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn net_protocols() {
        let netprotos = collect().expect("collecting network protocols");
        assert!(!netprotos.is_empty());

        for protocol in netprotos {
            assert!(!protocol.name.is_empty());
            assert!(protocol.size.ge(&0));
            assert!(protocol.sockets.ge(&0));
            assert!(protocol.memory.ge(&-1));
            assert!(protocol.max_header.ge(&0));
            assert!(!protocol.module_name.is_empty());
        }
    }
}
