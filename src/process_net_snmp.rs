use serde::Serialize;

use crate::{
    error::{CollectResult, MetricError},
    process::Process,
    utils,
};

/// ProcessNetSnmp models the content of /proc/\<pid\>/net/snmp
#[derive(Debug, Serialize, Clone, Default)]
pub struct ProcessNetSnmp {
    pub ip: Ip,
    pub icmp: Icmp,
    pub icmp_msg: IcmpMsg,
    pub tcp: Tcp,
    pub udp: Udp,
    pub upd_lite: UdpLite,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct Ip {
    pub forwarding: Option<i64>,
    pub default_ttl: Option<i64>,
    pub in_receives: Option<i64>,
    pub in_hdr_errors: Option<i64>,
    pub in_addr_errors: Option<i64>,
    pub forw_datagrams: Option<i64>,
    pub in_unkown_protos: Option<i64>,
    pub in_discards: Option<i64>,
    pub in_delivers: Option<i64>,
    pub out_requests: Option<i64>,
    pub out_discards: Option<i64>,
    pub out_no_routes: Option<i64>,
    pub reasm_timeout: Option<i64>,
    pub reasm_reqds: Option<i64>,
    pub reasm_oks: Option<i64>,
    pub reasm_fails: Option<i64>,
    pub frag_oks: Option<i64>,
    pub frag_fails: Option<i64>,
    pub frag_creates: Option<i64>,
    pub out_transmits: Option<i64>,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct Icmp {
    pub in_msgs: Option<i64>,
    pub in_errors: Option<i64>,
    pub in_csum_errors: Option<i64>,
    pub in_dest_unreachs: Option<i64>,
    pub in_time_excds: Option<i64>,
    pub in_parm_probs: Option<i64>,
    pub in_src_quenchs: Option<i64>,
    pub in_redirects: Option<i64>,
    pub in_echos: Option<i64>,
    pub in_echo_reps: Option<i64>,
    pub in_timestamps: Option<i64>,
    pub in_timestamp_reps: Option<i64>,
    pub in_addr_masks: Option<i64>,
    pub in_addr_mask_reps: Option<i64>,
    pub out_msgs: Option<i64>,
    pub out_errors: Option<i64>,
    pub out_rate_limit_global: Option<i64>,
    pub out_rate_limit_host: Option<i64>,
    pub out_dest_unreachs: Option<i64>,
    pub out_time_excds: Option<i64>,
    pub out_parm_probs: Option<i64>,
    pub out_src_quenchs: Option<i64>,
    pub out_redirects: Option<i64>,
    pub out_echos: Option<i64>,
    pub out_echo_reps: Option<i64>,
    pub out_timestamps: Option<i64>,
    pub out_timestamp_reps: Option<i64>,
    pub out_addr_masks: Option<i64>,
    pub out_addr_mask_reps: Option<i64>,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct IcmpMsg {
    pub in_type3: Option<i64>,
    pub out_type3: Option<i64>,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct Tcp {
    pub rto_algorithm: Option<i64>,
    pub rto_min: Option<i64>,
    pub rto_max: Option<i64>,
    pub max_conn: Option<i64>,
    pub active_opens: Option<i64>,
    pub passive_opens: Option<i64>,
    pub attempt_fails: Option<i64>,
    pub estab_resets: Option<i64>,
    pub curr_estab: Option<i64>,
    pub in_segs: Option<i64>,
    pub out_segs: Option<i64>,
    pub retrans_segs: Option<i64>,
    pub in_errs: Option<i64>,
    pub out_rsts: Option<i64>,
    pub in_csum_errors: Option<i64>,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct Udp {
    pub in_datagrams: Option<i64>,
    pub no_ports: Option<i64>,
    pub in_errors: Option<i64>,
    pub out_datagrams: Option<i64>,
    pub rcvbuf_errors: Option<i64>,
    pub sndbuf_errors: Option<i64>,
    pub in_csum_errors: Option<i64>,
    pub ignored_multi: Option<i64>,
    pub mem_errors: Option<i64>,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct UdpLite {
    pub in_datagrams: Option<i64>,
    pub no_ports: Option<i64>,
    pub in_errors: Option<i64>,
    pub out_datagrams: Option<i64>,
    pub rcvbuf_errors: Option<i64>,
    pub sndbuf_errors: Option<i64>,
    pub in_csum_errors: Option<i64>,
    pub ignored_multi: Option<i64>,
    pub mem_errors: Option<i64>,
}

impl ProcessNetSnmp {
    fn new() -> Self {
        Self {
            ip: Ip::new(),
            icmp: Icmp::new(),
            icmp_msg: IcmpMsg::new(),
            tcp: Tcp::new(),
            udp: Udp::new(),
            upd_lite: UdpLite::new(),
        }
    }
}

impl Ip {
    fn new() -> Self {
        Default::default()
    }
}

impl Icmp {
    fn new() -> Self {
        Default::default()
    }
}

impl IcmpMsg {
    fn new() -> Self {
        Default::default()
    }
}

impl Tcp {
    fn new() -> Self {
        Default::default()
    }
}

impl Udp {
    fn new() -> Self {
        Default::default()
    }
}

impl UdpLite {
    fn new() -> Self {
        Default::default()
    }
}

impl Process {
    /// net_snmp returns the current net/snmp stats of the process
    pub fn net_snmp(&self) -> CollectResult<ProcessNetSnmp> {
        let mut proc_netsnmp = ProcessNetSnmp::new();

        let proc_netsnmp_path_str = format!("{:?}/net", self.path());
        let proc_netsnmp_file = format!("{}/snmp", proc_netsnmp_path_str.replace("\"", ""));

        let netsnmp_data = utils::read_file_lines(&proc_netsnmp_file)?;
        let mut line_index = 0;

        while line_index < netsnmp_data.len() {
            let header_line = &netsnmp_data[line_index].to_lowercase();
            line_index += 1;

            if line_index >= netsnmp_data.len() {
                break;
            }

            let value_line = &netsnmp_data[line_index].to_lowercase();

            let header_sp: Vec<&str> = header_line
                .trim()
                .split(":")
                .filter(|s| !s.is_empty())
                .collect();
            let value_sp: Vec<&str> = value_line
                .trim()
                .split(":")
                .filter(|s| !s.is_empty())
                .collect();

            if header_sp.len() != 2 {
                return Err(MetricError::InvalidFieldNumberError(
                    "process net snmp header".to_string(),
                    header_sp.len(),
                    header_line.to_string(),
                ));
            }

            if value_sp.len() != 2 {
                return Err(MetricError::InvalidFieldNumberError(
                    "process net snmp header".to_string(),
                    value_sp.len(),
                    value_line.to_string(),
                ));
            }

            let header_data: Vec<&str> = header_sp[1]
                .trim()
                .split(" ")
                .filter(|s| !s.is_empty())
                .collect();
            let value_data: Vec<&str> = value_sp[1]
                .trim()
                .split(" ")
                .filter(|s| !s.is_empty())
                .collect();

            if header_data.len() != value_data.len() {
                return Err(MetricError::InvalidFieldNumberError(
                    "process net snmp mismatch field count mismatch header".to_string(),
                    header_data.len(),
                    header_data.len().to_string(),
                ));
            }

            match header_sp[0] {
                "ip" => {
                    let mut data_index = 0;
                    while data_index < header_data.len() {
                        match header_data[data_index] {
                            "forwarding" => {
                                proc_netsnmp.ip.forwarding =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "defaultttl" => {
                                proc_netsnmp.ip.default_ttl =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inreceives" => {
                                proc_netsnmp.ip.in_receives =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inhdrerrors" => {
                                proc_netsnmp.ip.in_hdr_errors =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inaddrerrors" => {
                                proc_netsnmp.ip.in_addr_errors =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "forwdatagrams" => {
                                proc_netsnmp.ip.forw_datagrams =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inunknownprotos" => {
                                proc_netsnmp.ip.in_unkown_protos =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "indiscards" => {
                                proc_netsnmp.ip.in_discards =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "indelivers" => {
                                proc_netsnmp.ip.in_delivers =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outrequests" => {
                                proc_netsnmp.ip.out_requests =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outdiscards" => {
                                proc_netsnmp.ip.out_discards =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outnoroutes" => {
                                proc_netsnmp.ip.out_no_routes =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "reasmtimeout" => {
                                proc_netsnmp.ip.reasm_timeout =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "reasmreqds" => {
                                proc_netsnmp.ip.reasm_reqds =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "reasmoks" => {
                                proc_netsnmp.ip.reasm_oks =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "reasmfails" => {
                                proc_netsnmp.ip.reasm_fails =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "fragoks" => {
                                proc_netsnmp.ip.frag_oks =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "fragfails" => {
                                proc_netsnmp.ip.frag_fails =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "fragcreates" => {
                                proc_netsnmp.ip.frag_creates =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outtransmits" => {
                                proc_netsnmp.ip.out_transmits =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            _ => {}
                        }

                        data_index += 1;
                    }
                }
                "icmp" => {
                    let mut data_index = 0;
                    while data_index < header_data.len() {
                        match header_data[data_index] {
                            "inmsgs" => {
                                proc_netsnmp.icmp.in_msgs =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inerrors" => {
                                proc_netsnmp.icmp.in_errors =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "incsumerrors" => {
                                proc_netsnmp.icmp.in_csum_errors =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "indestunreachs" => {
                                proc_netsnmp.icmp.in_dest_unreachs =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "intimeexcds" => {
                                proc_netsnmp.icmp.in_time_excds =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inparmprobs" => {
                                proc_netsnmp.icmp.in_parm_probs =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "insrcquenchs" => {
                                proc_netsnmp.icmp.in_src_quenchs =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inredirects" => {
                                proc_netsnmp.icmp.in_redirects =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inechos" => {
                                proc_netsnmp.icmp.in_echos =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inechoreps" => {
                                proc_netsnmp.icmp.in_echo_reps =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "intimestamps" => {
                                proc_netsnmp.icmp.in_timestamps =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "intimestampreps" => {
                                proc_netsnmp.icmp.in_timestamp_reps =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inaddrmasks" => {
                                proc_netsnmp.icmp.in_addr_masks =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inaddrmaskreps" => {
                                proc_netsnmp.icmp.in_addr_mask_reps =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outmsgs" => {
                                proc_netsnmp.icmp.out_msgs =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outerrors" => {
                                proc_netsnmp.icmp.out_errors =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outratelimitglobal" => {
                                proc_netsnmp.icmp.out_rate_limit_global =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outratelimithost" => {
                                proc_netsnmp.icmp.out_rate_limit_host =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outdestunreachs" => {
                                proc_netsnmp.icmp.out_dest_unreachs =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outtimeexcds" => {
                                proc_netsnmp.icmp.out_time_excds =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outparmprobs" => {
                                proc_netsnmp.icmp.out_parm_probs =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outsrcquenchs" => {
                                proc_netsnmp.icmp.out_src_quenchs =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outredirects" => {
                                proc_netsnmp.icmp.out_redirects =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outechos" => {
                                proc_netsnmp.icmp.out_echos =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outechoreps" => {
                                proc_netsnmp.icmp.out_echo_reps =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outtimestamps" => {
                                proc_netsnmp.icmp.out_timestamps =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outtimestampreps" => {
                                proc_netsnmp.icmp.out_timestamp_reps =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outaddrmasks" => {
                                proc_netsnmp.icmp.out_addr_masks =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outaddrmaskreps" => {
                                proc_netsnmp.icmp.out_addr_mask_reps =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            _ => {}
                        }
                        data_index += 1;
                    }
                }
                "icmpmsg" => {
                    let mut data_index = 0;
                    while data_index < header_data.len() {
                        match header_data[data_index] {
                            "intype3" => {
                                proc_netsnmp.icmp_msg.in_type3 =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outtype3" => {
                                proc_netsnmp.icmp_msg.out_type3 =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            _ => {}
                        }

                        data_index += 1;
                    }
                }
                "tcp" => {
                    let mut data_index = 0;
                    while data_index < header_data.len() {
                        match header_data[data_index] {
                            "rtoalgorithm" => {
                                proc_netsnmp.tcp.rto_algorithm =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "rtomin" => {
                                proc_netsnmp.tcp.rto_min =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "rtomax" => {
                                proc_netsnmp.tcp.rto_max =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "maxconn" => {
                                proc_netsnmp.tcp.max_conn =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "activeopens" => {
                                proc_netsnmp.tcp.active_opens =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "passiveopens" => {
                                proc_netsnmp.tcp.passive_opens =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "attemptfails" => {
                                proc_netsnmp.tcp.attempt_fails =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "estabresets" => {
                                proc_netsnmp.tcp.estab_resets =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "currestab" => {
                                proc_netsnmp.tcp.curr_estab =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "insegs" => {
                                proc_netsnmp.tcp.in_segs =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outsegs" => {
                                proc_netsnmp.tcp.out_segs =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "retranssegs" => {
                                proc_netsnmp.tcp.retrans_segs =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inerrs" => {
                                proc_netsnmp.tcp.in_errs =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outrsts" => {
                                proc_netsnmp.tcp.out_rsts =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "incsumerrors" => {
                                proc_netsnmp.tcp.in_csum_errors =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            _ => {}
                        }

                        data_index += 1;
                    }
                }
                "udp" => {
                    let mut data_index = 0;
                    while data_index < header_data.len() {
                        match header_data[data_index] {
                            "indatagrams" => {
                                proc_netsnmp.udp.in_datagrams =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "noports" => {
                                proc_netsnmp.udp.no_ports =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inerrors" => {
                                proc_netsnmp.udp.in_errors =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outdatagrams" => {
                                proc_netsnmp.udp.out_datagrams =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "rcvbuferrors" => {
                                proc_netsnmp.udp.rcvbuf_errors =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "sndbuferrors" => {
                                proc_netsnmp.udp.sndbuf_errors =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "incsumerrors" => {
                                proc_netsnmp.udp.in_csum_errors =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "ignoredmulti" => {
                                proc_netsnmp.udp.ignored_multi =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "memerrors" => {
                                proc_netsnmp.udp.mem_errors =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            _ => {}
                        }

                        data_index += 1;
                    }
                }
                "udplite" => {
                    let mut data_index = 0;
                    while data_index < header_data.len() {
                        match header_data[data_index] {
                            "indatagrams" => {
                                proc_netsnmp.upd_lite.in_datagrams =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "noports" => {
                                proc_netsnmp.upd_lite.no_ports =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inerrors" => {
                                proc_netsnmp.upd_lite.in_errors =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outdatagrams" => {
                                proc_netsnmp.upd_lite.out_datagrams =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "rcvbuferrors" => {
                                proc_netsnmp.upd_lite.rcvbuf_errors =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "sndbuferrors" => {
                                proc_netsnmp.upd_lite.sndbuf_errors =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "incsumerrors" => {
                                proc_netsnmp.upd_lite.in_csum_errors =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "ignoredmulti" => {
                                proc_netsnmp.upd_lite.ignored_multi =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "memerrors" => {
                                proc_netsnmp.upd_lite.mem_errors =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            _ => {}
                        }

                        data_index += 1;
                    }
                }
                _ => {}
            }

            line_index += 1;
        }

        Ok(proc_netsnmp)
    }
}

#[cfg(test)]
mod tests {
    use crate::process::*;
    use std::path::Path;

    #[test]
    fn proc_net_snmp() {
        let proc_path = Path::new("test_data/fixtures/proc");
        let sys_proc = collect_from(proc_path, 26231).expect("running proc 26231");
        let sys_proc_netsnmp = sys_proc.net_snmp().expect("running proc 26231 net snmp");

        assert_eq!(sys_proc_netsnmp.ip.forwarding.unwrap(), 2);
        assert_eq!(sys_proc_netsnmp.ip.default_ttl.unwrap(), 64);
        assert_eq!(sys_proc_netsnmp.ip.in_receives.unwrap(), 594223);
        assert_eq!(sys_proc_netsnmp.ip.in_hdr_errors.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.ip.in_addr_errors.unwrap(), 1);
        assert_eq!(sys_proc_netsnmp.ip.forw_datagrams.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.ip.in_unkown_protos.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.ip.in_discards.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.ip.in_delivers.unwrap(), 593186);
        assert_eq!(sys_proc_netsnmp.ip.out_requests.unwrap(), 547253);
        assert_eq!(sys_proc_netsnmp.ip.out_discards.unwrap(), 20);
        assert_eq!(sys_proc_netsnmp.ip.out_no_routes.unwrap(), 231);
        assert_eq!(sys_proc_netsnmp.ip.reasm_timeout.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.ip.reasm_reqds.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.ip.reasm_oks.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.ip.reasm_fails.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.ip.frag_oks.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.ip.frag_fails.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.ip.frag_creates.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.ip.out_transmits, None);

        assert_eq!(sys_proc_netsnmp.icmp.in_msgs.unwrap(), 45);
        assert_eq!(sys_proc_netsnmp.icmp.in_errors.unwrap(), 1);
        assert_eq!(sys_proc_netsnmp.icmp.in_csum_errors.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.in_dest_unreachs.unwrap(), 45);
        assert_eq!(sys_proc_netsnmp.icmp.in_time_excds.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.in_parm_probs.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.in_src_quenchs.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.in_redirects.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.in_echos.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.in_echo_reps.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.in_timestamps.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.in_timestamp_reps.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.in_addr_masks.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.in_addr_mask_reps.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.out_msgs.unwrap(), 50);
        assert_eq!(sys_proc_netsnmp.icmp.out_errors.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.out_rate_limit_global, None);
        assert_eq!(sys_proc_netsnmp.icmp.out_rate_limit_host, None);
        assert_eq!(sys_proc_netsnmp.icmp.out_dest_unreachs.unwrap(), 50);
        assert_eq!(sys_proc_netsnmp.icmp.out_time_excds.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.out_parm_probs.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.out_src_quenchs.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.out_redirects.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.out_echos.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.out_echo_reps.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.out_timestamps.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.out_timestamp_reps.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.out_addr_masks.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.icmp.out_addr_mask_reps.unwrap(), 0);

        assert_eq!(sys_proc_netsnmp.icmp_msg.in_type3.unwrap(), 45);
        assert_eq!(sys_proc_netsnmp.icmp_msg.out_type3.unwrap(), 50);

        assert_eq!(sys_proc_netsnmp.tcp.rto_algorithm.unwrap(), 1);
        assert_eq!(sys_proc_netsnmp.tcp.rto_min.unwrap(), 200);
        assert_eq!(sys_proc_netsnmp.tcp.rto_max.unwrap(), 120000);
        assert_eq!(sys_proc_netsnmp.tcp.max_conn.unwrap(), -1);
        assert_eq!(sys_proc_netsnmp.tcp.active_opens.unwrap(), 1103);
        assert_eq!(sys_proc_netsnmp.tcp.passive_opens.unwrap(), 9);
        assert_eq!(sys_proc_netsnmp.tcp.attempt_fails.unwrap(), 8);
        assert_eq!(sys_proc_netsnmp.tcp.estab_resets.unwrap(), 51);
        assert_eq!(sys_proc_netsnmp.tcp.curr_estab.unwrap(), 15);
        assert_eq!(sys_proc_netsnmp.tcp.in_segs.unwrap(), 653161);
        assert_eq!(sys_proc_netsnmp.tcp.out_segs.unwrap(), 594855);
        assert_eq!(sys_proc_netsnmp.tcp.retrans_segs.unwrap(), 348);
        assert_eq!(sys_proc_netsnmp.tcp.in_errs.unwrap(), 98);
        assert_eq!(sys_proc_netsnmp.tcp.out_rsts.unwrap(), 1038);
        assert_eq!(sys_proc_netsnmp.tcp.in_csum_errors.unwrap(), 0);

        assert_eq!(sys_proc_netsnmp.udp.in_datagrams.unwrap(), 10179);
        assert_eq!(sys_proc_netsnmp.udp.no_ports.unwrap(), 50);
        assert_eq!(sys_proc_netsnmp.udp.in_errors.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.udp.out_datagrams.unwrap(), 9846);
        assert_eq!(sys_proc_netsnmp.udp.rcvbuf_errors.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.udp.sndbuf_errors.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.udp.in_csum_errors.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.udp.ignored_multi.unwrap(), 58);
        assert_eq!(sys_proc_netsnmp.udp.mem_errors, None);

        assert_eq!(sys_proc_netsnmp.upd_lite.in_datagrams.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.upd_lite.no_ports.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.upd_lite.in_errors.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.upd_lite.out_datagrams.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.upd_lite.rcvbuf_errors.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.upd_lite.sndbuf_errors.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.upd_lite.in_csum_errors.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.upd_lite.ignored_multi.unwrap(), 0);
        assert_eq!(sys_proc_netsnmp.upd_lite.mem_errors, None);

        let sys_proc = collect_from(proc_path, 26234).expect("running proc 26234");
        let sys_proc_netsnmp = sys_proc.net_snmp();
        assert_eq!(sys_proc_netsnmp.is_err(), true);
    }
}
