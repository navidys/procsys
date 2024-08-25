use serde::Serialize;

use crate::{
    error::{CollectResult, MetricError},
    process::Process,
    utils,
};

/// ProcessNetSnmp6 models the content of /proc/\<pid\>/net/snmp6
#[derive(Debug, Serialize, Clone, Default)]
pub struct ProcessNetSnmp6 {
    pub ip6_in_receives: Option<i64>,
    pub ip6_in_hdr_errors: Option<i64>,
    pub ip6_in_too_big_errors: Option<i64>,
    pub ip6_in_no_routes: Option<i64>,
    pub ip6_in_addr_errors: Option<i64>,
    pub ip6_in_unknown_protos: Option<i64>,
    pub ip6_in_truncated_pkts: Option<i64>,
    pub ip6_in_discards: Option<i64>,
    pub ip6_in_delivers: Option<i64>,
    pub ip6_out_forw_datagrams: Option<i64>,
    pub ip6_out_requests: Option<i64>,
    pub ip6_out_discards: Option<i64>,
    pub ip6_out_no_routes: Option<i64>,
    pub ip6_reasm_timeout: Option<i64>,
    pub ip6_reasm_reqds: Option<i64>,
    pub ip6_reasm_oks: Option<i64>,
    pub ip6_reasm_fails: Option<i64>,
    pub ip6_frag_oks: Option<i64>,
    pub ip6_frag_fails: Option<i64>,
    pub ip6_frag_creates: Option<i64>,
    pub ip6_in_mcast_pkts: Option<i64>,
    pub ip6_out_mcast_pkts: Option<i64>,
    pub ip6_in_octets: Option<i64>,
    pub ip6_out_octets: Option<i64>,
    pub ip6_in_mcast_octets: Option<i64>,
    pub ip6_out_mcast_octets: Option<i64>,
    pub ip6_in_bcast_octets: Option<i64>,
    pub ip6_out_bcast_octets: Option<i64>,
    pub ip6_in_no_ect_pkts: Option<i64>,
    pub ip6_in_ect1_pkts: Option<i64>,
    pub ip6_in_ect0_pkts: Option<i64>,
    pub ip6_in_ce_pkts: Option<i64>,
    pub ip6_out_transmits: Option<i64>,
    pub icmp6_in_msgs: Option<i64>,
    pub icmp6_in_errors: Option<i64>,
    pub icmp6_out_msgs: Option<i64>,
    pub icmp6_out_errors: Option<i64>,
    pub icmp6_in_csum_errors: Option<i64>,
    pub icmp6_out_rate_limit_host: Option<i64>,
    pub icmp6_in_dest_unreachs: Option<i64>,
    pub icmp6_in_pkt_too_bigs: Option<i64>,
    pub icmp6_in_time_excds: Option<i64>,
    pub icmp6_in_parm_problems: Option<i64>,
    pub icmp6_in_echos: Option<i64>,
    pub icmp6_in_echo_replies: Option<i64>,
    pub icmp6_in_group_memb_queries: Option<i64>,
    pub icmp6_in_group_memb_responses: Option<i64>,
    pub icmp6_in_group_memb_reductions: Option<i64>,
    pub icmp6_in_router_solicits: Option<i64>,
    pub icmp6_in_router_advertisements: Option<i64>,
    pub icmp6_in_neighbor_solicits: Option<i64>,
    pub icmp6_in_neighbor_advertisements: Option<i64>,
    pub icmp6_in_redirects: Option<i64>,
    pub icmp6_in_mldv2_reports: Option<i64>,
    pub icmp6_out_dest_unreachs: Option<i64>,
    pub icmp6_out_pkt_too_bigs: Option<i64>,
    pub icmp6_out_time_excds: Option<i64>,
    pub icmp6_out_parm_problems: Option<i64>,
    pub icmp6_out_echos: Option<i64>,
    pub icmp6_out_echo_replies: Option<i64>,
    pub icmp6_out_group_memb_queries: Option<i64>,
    pub icmp6_out_group_memb_responses: Option<i64>,
    pub icmp6_out_group_memb_reductions: Option<i64>,
    pub icmp6_out_router_solicits: Option<i64>,
    pub icmp6_out_router_advertisements: Option<i64>,
    pub icmp6_out_neighbor_solicits: Option<i64>,
    pub icmp6_out_neighbor_advertisements: Option<i64>,
    pub icmp6_out_redirects: Option<i64>,
    pub icmp6_out_mldv2_reports: Option<i64>,
    pub icmp6_in_type1: Option<i64>,
    pub icmp6_in_type134: Option<i64>,
    pub icmp6_in_type135: Option<i64>,
    pub icmp6_in_type136: Option<i64>,
    pub icmp6_in_type143: Option<i64>,
    pub icmp6_out_type133: Option<i64>,
    pub icmp6_out_type135: Option<i64>,
    pub icmp6_out_type136: Option<i64>,
    pub icmp6_out_type143: Option<i64>,
    pub udp6_in_datagrams: Option<i64>,
    pub udp6_no_ports: Option<i64>,
    pub udp6_in_errors: Option<i64>,
    pub udp6_out_datagrams: Option<i64>,
    pub udp6_rcvbuf_errors: Option<i64>,
    pub udp6_sndbuf_errors: Option<i64>,
    pub udp6_in_csum_errors: Option<i64>,
    pub udp6_ignored_multi: Option<i64>,
    pub udp6_mem_mrrors: Option<i64>,
    pub udp_lite6_in_datagrams: Option<i64>,
    pub udp_lite6_no_ports: Option<i64>,
    pub udp_lite6_in_errors: Option<i64>,
    pub udp_lite6_out_datagrams: Option<i64>,
    pub udp_lite6_rcvbuf_errors: Option<i64>,
    pub udp_lite6_sndbuf_errors: Option<i64>,
    pub udp_lite6_in_csum_errors: Option<i64>,
    pub udp_lite6_mem_errors: Option<i64>,
}

impl ProcessNetSnmp6 {
    fn new() -> Self {
        Default::default()
    }
}

impl Process {
    /// net_snmp6 returns the current net/snmp6 stats of the process
    pub fn net_snmp6(&self) -> CollectResult<ProcessNetSnmp6> {
        let mut proc_netsnmp6 = ProcessNetSnmp6::new();

        let proc_netsnmp6_path_str = format!("{:?}/net", self.path());
        let proc_netsnmp6_file = format!("{}/snmp6", proc_netsnmp6_path_str.replace("\"", ""));

        for line in utils::read_file_lines(&proc_netsnmp6_file)? {
            let snmp6_data: Vec<&str> = line.trim().split(" ").filter(|s| !s.is_empty()).collect();

            if snmp6_data.len() != 2 {
                return Err(MetricError::InvalidFieldNumberError(
                    "process net snmp6".to_string(),
                    snmp6_data.len(),
                    line,
                ));
            }

            let item = snmp6_data[0].trim();
            let value = utils::convert_str_to_i64(snmp6_data[1].trim())?;

            match item {
                "Ip6InReceives" => proc_netsnmp6.ip6_in_receives = Some(value),
                "Ip6InHdrErrors" => proc_netsnmp6.ip6_in_hdr_errors = Some(value),
                "Ip6InTooBigErrors" => proc_netsnmp6.ip6_in_too_big_errors = Some(value),
                "Ip6InNoRoutes" => proc_netsnmp6.ip6_in_no_routes = Some(value),
                "Ip6InAddrErrors" => proc_netsnmp6.ip6_in_addr_errors = Some(value),
                "Ip6InUnknownProtos" => proc_netsnmp6.ip6_in_unknown_protos = Some(value),
                "Ip6InTruncatedPkts" => proc_netsnmp6.ip6_in_truncated_pkts = Some(value),
                "Ip6InDiscards" => proc_netsnmp6.ip6_in_discards = Some(value),
                "Ip6InDelivers" => proc_netsnmp6.ip6_in_delivers = Some(value),
                "Ip6OutForwDatagrams" => proc_netsnmp6.ip6_out_forw_datagrams = Some(value),
                "Ip6OutRequests" => proc_netsnmp6.ip6_out_requests = Some(value),
                "Ip6OutDiscards" => proc_netsnmp6.ip6_out_discards = Some(value),
                "Ip6OutNoRoutes" => proc_netsnmp6.ip6_out_no_routes = Some(value),
                "Ip6ReasmTimeout" => proc_netsnmp6.ip6_reasm_timeout = Some(value),
                "Ip6ReasmReqds" => proc_netsnmp6.ip6_reasm_reqds = Some(value),
                "Ip6ReasmOKs" => proc_netsnmp6.ip6_reasm_oks = Some(value),
                "Ip6ReasmFails" => proc_netsnmp6.ip6_reasm_fails = Some(value),
                "Ip6FragOKs" => proc_netsnmp6.ip6_frag_oks = Some(value),
                "Ip6FragFails" => proc_netsnmp6.ip6_frag_fails = Some(value),
                "Ip6FragCreates" => proc_netsnmp6.ip6_frag_creates = Some(value),
                "Ip6InMcastPkts" => proc_netsnmp6.ip6_in_mcast_pkts = Some(value),
                "Ip6OutMcastPkts" => proc_netsnmp6.ip6_out_mcast_pkts = Some(value),
                "Ip6InOctets" => proc_netsnmp6.ip6_in_octets = Some(value),
                "Ip6OutOctets" => proc_netsnmp6.ip6_out_octets = Some(value),
                "Ip6InMcastOctets" => proc_netsnmp6.ip6_in_mcast_octets = Some(value),
                "Ip6OutMcastOctets" => proc_netsnmp6.ip6_out_mcast_octets = Some(value),
                "Ip6InBcastOctets" => proc_netsnmp6.ip6_in_bcast_octets = Some(value),
                "Ip6OutBcastOctets" => proc_netsnmp6.ip6_out_bcast_octets = Some(value),
                "Ip6InNoECTPkts" => proc_netsnmp6.ip6_in_no_ect_pkts = Some(value),
                "Ip6InECT1Pkts" => proc_netsnmp6.ip6_in_ect1_pkts = Some(value),
                "Ip6InECT0Pkts" => proc_netsnmp6.ip6_in_ect0_pkts = Some(value),
                "Ip6InCEPkts" => proc_netsnmp6.ip6_in_ce_pkts = Some(value),
                "Ip6OutTransmits" => proc_netsnmp6.ip6_out_transmits = Some(value),
                "Icmp6InMsgs" => proc_netsnmp6.icmp6_in_msgs = Some(value),
                "Icmp6InErrors" => proc_netsnmp6.icmp6_in_errors = Some(value),
                "Icmp6OutMsgs" => proc_netsnmp6.icmp6_out_msgs = Some(value),
                "Icmp6OutErrors" => proc_netsnmp6.icmp6_out_errors = Some(value),
                "Icmp6InCsumErrors" => proc_netsnmp6.icmp6_in_csum_errors = Some(value),
                "Icmp6OutRateLimitHost" => proc_netsnmp6.icmp6_out_rate_limit_host = Some(value),
                "Icmp6InDestUnreachs" => proc_netsnmp6.icmp6_in_dest_unreachs = Some(value),
                "Icmp6InPktTooBigs" => proc_netsnmp6.icmp6_in_pkt_too_bigs = Some(value),
                "Icmp6InTimeExcds" => proc_netsnmp6.icmp6_in_time_excds = Some(value),
                "Icmp6InParmProblems" => proc_netsnmp6.icmp6_in_parm_problems = Some(value),
                "Icmp6InEchos" => proc_netsnmp6.icmp6_in_echos = Some(value),
                "Icmp6InEchoReplies" => proc_netsnmp6.icmp6_in_echo_replies = Some(value),
                "Icmp6InGroupMembQueries" => {
                    proc_netsnmp6.icmp6_in_group_memb_queries = Some(value)
                }
                "Icmp6InGroupMembResponses" => {
                    proc_netsnmp6.icmp6_in_group_memb_responses = Some(value)
                }
                "Icmp6InGroupMembReductions" => {
                    proc_netsnmp6.icmp6_in_group_memb_reductions = Some(value)
                }
                "Icmp6InRouterSolicits" => proc_netsnmp6.icmp6_in_router_solicits = Some(value),
                "Icmp6InRouterAdvertisements" => {
                    proc_netsnmp6.icmp6_in_router_advertisements = Some(value)
                }
                "Icmp6InNeighborSolicits" => proc_netsnmp6.icmp6_in_neighbor_solicits = Some(value),
                "Icmp6InNeighborAdvertisements" => {
                    proc_netsnmp6.icmp6_in_neighbor_advertisements = Some(value)
                }
                "Icmp6InRedirects" => proc_netsnmp6.icmp6_in_redirects = Some(value),
                "Icmp6InMLDv2Reports" => proc_netsnmp6.icmp6_in_mldv2_reports = Some(value),
                "Icmp6OutDestUnreachs" => proc_netsnmp6.icmp6_out_dest_unreachs = Some(value),
                "Icmp6OutPktTooBigs" => proc_netsnmp6.icmp6_out_pkt_too_bigs = Some(value),
                "Icmp6OutTimeExcds" => proc_netsnmp6.icmp6_out_time_excds = Some(value),
                "Icmp6OutParmProblems" => proc_netsnmp6.icmp6_out_parm_problems = Some(value),
                "Icmp6OutEchos" => proc_netsnmp6.icmp6_out_echos = Some(value),
                "Icmp6OutEchoReplies" => proc_netsnmp6.icmp6_out_echo_replies = Some(value),
                "Icmp6OutGroupMembQueries" => {
                    proc_netsnmp6.icmp6_out_group_memb_queries = Some(value)
                }
                "Icmp6OutGroupMembResponses" => {
                    proc_netsnmp6.icmp6_out_group_memb_responses = Some(value)
                }
                "Icmp6OutGroupMembReductions" => {
                    proc_netsnmp6.icmp6_out_group_memb_reductions = Some(value)
                }
                "Icmp6OutRouterSolicits" => proc_netsnmp6.icmp6_out_router_solicits = Some(value),
                "Icmp6OutRouterAdvertisements" => {
                    proc_netsnmp6.icmp6_out_router_advertisements = Some(value)
                }
                "Icmp6OutNeighborSolicits" => {
                    proc_netsnmp6.icmp6_out_neighbor_solicits = Some(value)
                }
                "Icmp6OutNeighborAdvertisements" => {
                    proc_netsnmp6.icmp6_out_neighbor_advertisements = Some(value)
                }
                "Icmp6OutRedirects" => proc_netsnmp6.icmp6_out_redirects = Some(value),
                "Icmp6OutMLDv2Reports" => proc_netsnmp6.icmp6_out_mldv2_reports = Some(value),
                "Icmp6InType1" => proc_netsnmp6.icmp6_in_type1 = Some(value),
                "Icmp6InType134" => proc_netsnmp6.icmp6_in_type134 = Some(value),
                "Icmp6InType135" => proc_netsnmp6.icmp6_in_type135 = Some(value),
                "Icmp6InType136" => proc_netsnmp6.icmp6_in_type136 = Some(value),
                "Icmp6InType143" => proc_netsnmp6.icmp6_in_type143 = Some(value),
                "Icmp6OutType133" => proc_netsnmp6.icmp6_out_type133 = Some(value),
                "Icmp6OutType135" => proc_netsnmp6.icmp6_out_type135 = Some(value),
                "Icmp6OutType136" => proc_netsnmp6.icmp6_out_type136 = Some(value),
                "Icmp6OutType143" => proc_netsnmp6.icmp6_out_type143 = Some(value),
                "Udp6InDatagrams" => proc_netsnmp6.udp6_in_datagrams = Some(value),
                "Udp6NoPorts" => proc_netsnmp6.udp6_no_ports = Some(value),
                "Udp6InErrors" => proc_netsnmp6.udp6_in_errors = Some(value),
                "Udp6OutDatagrams" => proc_netsnmp6.udp6_out_datagrams = Some(value),
                "Udp6RcvbufErrors" => proc_netsnmp6.udp6_rcvbuf_errors = Some(value),
                "Udp6SndbufErrors" => proc_netsnmp6.udp6_sndbuf_errors = Some(value),
                "Udp6InCsumErrors" => proc_netsnmp6.udp6_in_csum_errors = Some(value),
                "Udp6IgnoredMulti" => proc_netsnmp6.udp6_ignored_multi = Some(value),
                "Udp6MemErrors" => proc_netsnmp6.udp6_mem_mrrors = Some(value),
                "UdpLite6InDatagrams" => proc_netsnmp6.udp_lite6_in_datagrams = Some(value),
                "UdpLite6NoPorts" => proc_netsnmp6.udp_lite6_no_ports = Some(value),
                "UdpLite6InErrors" => proc_netsnmp6.udp_lite6_in_errors = Some(value),
                "UdpLite6OutDatagrams" => proc_netsnmp6.udp_lite6_out_datagrams = Some(value),
                "UdpLite6RcvbufErrors" => proc_netsnmp6.udp_lite6_rcvbuf_errors = Some(value),
                "UdpLite6SndbufErrors" => proc_netsnmp6.udp_lite6_sndbuf_errors = Some(value),
                "UdpLite6InCsumErrors" => proc_netsnmp6.udp_lite6_in_csum_errors = Some(value),
                "UdpLite6MemErrors" => proc_netsnmp6.udp_lite6_mem_errors = Some(value),
                _ => {}
            }
        }

        Ok(proc_netsnmp6)
    }
}

#[cfg(test)]
mod tests {
    use crate::process::*;
    use std::path::Path;

    #[test]
    fn proc_net_snmp6() {
        let proc_path = Path::new("test_data/fixtures/proc");
        let sys_proc = collect_from(proc_path, 26231).expect("running proc 26231");
        let proc_netsnmp6 = sys_proc.net_snmp6().expect("running proc 26231 net snmp6");

        assert_eq!(proc_netsnmp6.ip6_in_receives.unwrap(), 92166);
        assert_eq!(proc_netsnmp6.ip6_in_hdr_errors.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_in_too_big_errors.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_in_no_routes.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_in_addr_errors.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_in_unknown_protos.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_in_truncated_pkts.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_in_discards.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_in_delivers.unwrap(), 92053);
        assert_eq!(proc_netsnmp6.ip6_out_forw_datagrams.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_out_requests.unwrap(), 57502);
        assert_eq!(proc_netsnmp6.ip6_out_discards.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_out_no_routes.unwrap(), 169);
        assert_eq!(proc_netsnmp6.ip6_reasm_timeout.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_reasm_reqds.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_reasm_oks.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_reasm_fails.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_frag_oks.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_frag_fails.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_frag_creates.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_in_mcast_pkts.unwrap(), 381);
        assert_eq!(proc_netsnmp6.ip6_out_mcast_pkts.unwrap(), 148);
        assert_eq!(proc_netsnmp6.ip6_in_octets.unwrap(), 113479132);
        assert_eq!(proc_netsnmp6.ip6_out_octets.unwrap(), 9842685);
        assert_eq!(proc_netsnmp6.ip6_in_mcast_octets.unwrap(), 65971);
        assert_eq!(proc_netsnmp6.ip6_out_mcast_octets.unwrap(), 19394);
        assert_eq!(proc_netsnmp6.ip6_in_bcast_octets.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_out_bcast_octets.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_in_no_ect_pkts.unwrap(), 92166);
        assert_eq!(proc_netsnmp6.ip6_in_ect1_pkts.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_in_ect0_pkts.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_in_ce_pkts.unwrap(), 0);
        assert_eq!(proc_netsnmp6.ip6_out_transmits, None);
        assert_eq!(proc_netsnmp6.icmp6_in_msgs.unwrap(), 142);
        assert_eq!(proc_netsnmp6.icmp6_in_errors.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_out_msgs.unwrap(), 58);
        assert_eq!(proc_netsnmp6.icmp6_out_errors.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_in_csum_errors.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_out_rate_limit_host, None);
        assert_eq!(proc_netsnmp6.icmp6_in_dest_unreachs.unwrap(), 2);
        assert_eq!(proc_netsnmp6.icmp6_in_pkt_too_bigs.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_in_time_excds.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_in_parm_problems.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_in_echos.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_in_echo_replies.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_in_group_memb_queries.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_in_group_memb_responses.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_in_group_memb_reductions.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_in_router_solicits.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_in_router_advertisements.unwrap(), 111);
        assert_eq!(proc_netsnmp6.icmp6_in_neighbor_solicits.unwrap(), 26);
        assert_eq!(proc_netsnmp6.icmp6_in_neighbor_advertisements.unwrap(), 1);
        assert_eq!(proc_netsnmp6.icmp6_in_redirects.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_in_mldv2_reports.unwrap(), 2);
        assert_eq!(proc_netsnmp6.icmp6_out_dest_unreachs.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_out_pkt_too_bigs.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_out_time_excds.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_out_parm_problems.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_out_echos.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_out_echo_replies.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_out_group_memb_queries.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_out_group_memb_responses.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_out_group_memb_reductions.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_out_router_solicits.unwrap(), 2);
        assert_eq!(proc_netsnmp6.icmp6_out_router_advertisements.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_out_neighbor_solicits.unwrap(), 5);
        assert_eq!(proc_netsnmp6.icmp6_out_neighbor_advertisements.unwrap(), 26);
        assert_eq!(proc_netsnmp6.icmp6_out_redirects.unwrap(), 0);
        assert_eq!(proc_netsnmp6.icmp6_out_mldv2_reports.unwrap(), 25);
        assert_eq!(proc_netsnmp6.icmp6_in_type1.unwrap(), 2);
        assert_eq!(proc_netsnmp6.icmp6_in_type134.unwrap(), 111);
        assert_eq!(proc_netsnmp6.icmp6_in_type135.unwrap(), 26);
        assert_eq!(proc_netsnmp6.icmp6_in_type136.unwrap(), 1);
        assert_eq!(proc_netsnmp6.icmp6_in_type143.unwrap(), 2);
        assert_eq!(proc_netsnmp6.icmp6_out_type133.unwrap(), 2);
        assert_eq!(proc_netsnmp6.icmp6_out_type135.unwrap(), 5);
        assert_eq!(proc_netsnmp6.icmp6_out_type136.unwrap(), 26);
        assert_eq!(proc_netsnmp6.icmp6_out_type143.unwrap(), 25);
        assert_eq!(proc_netsnmp6.udp6_in_datagrams.unwrap(), 2016);
        assert_eq!(proc_netsnmp6.udp6_no_ports.unwrap(), 0);
        assert_eq!(proc_netsnmp6.udp6_in_errors.unwrap(), 0);
        assert_eq!(proc_netsnmp6.udp6_out_datagrams.unwrap(), 1546);
        assert_eq!(proc_netsnmp6.udp6_rcvbuf_errors.unwrap(), 0);
        assert_eq!(proc_netsnmp6.udp6_sndbuf_errors.unwrap(), 0);
        assert_eq!(proc_netsnmp6.udp6_in_csum_errors.unwrap(), 0);
        assert_eq!(proc_netsnmp6.udp6_ignored_multi.unwrap(), 12);
        assert_eq!(proc_netsnmp6.udp6_mem_mrrors, None);
        assert_eq!(proc_netsnmp6.udp_lite6_in_datagrams.unwrap(), 0);
        assert_eq!(proc_netsnmp6.udp_lite6_no_ports.unwrap(), 0);
        assert_eq!(proc_netsnmp6.udp_lite6_in_errors.unwrap(), 0);
        assert_eq!(proc_netsnmp6.udp_lite6_out_datagrams.unwrap(), 0);
        assert_eq!(proc_netsnmp6.udp_lite6_rcvbuf_errors.unwrap(), 0);
        assert_eq!(proc_netsnmp6.udp_lite6_sndbuf_errors.unwrap(), 0);
        assert_eq!(proc_netsnmp6.udp_lite6_in_csum_errors.unwrap(), 0);
        assert_eq!(proc_netsnmp6.udp_lite6_mem_errors, None);

        let sys_proc = collect_from(proc_path, 26234).expect("running proc 26234");
        let proc_netsnmp6 = sys_proc.net_snmp6();
        assert_eq!(proc_netsnmp6.is_err(), true);
    }
}
