use serde::Serialize;

use crate::{
    error::{CollectResult, MetricError},
    process::Process,
    utils,
};

/// ProcessNetstat models the content of /proc/\<pid\>/net/netstat
#[derive(Debug, Serialize, Clone, Default)]

pub struct ProcessNetstat {
    pub tcp_ext: TcpExt,
    pub ip_ext: IpExt,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct TcpExt {
    pub syn_cookies_sent: Option<i64>,
    pub syn_cookies_recv: Option<i64>,
    pub syn_cookies_failed: Option<i64>,
    pub embryonic_rsts: Option<i64>,
    pub prune_called: Option<i64>,
    pub rcv_pruned: Option<i64>,
    pub ofo_pruned: Option<i64>,
    pub out_of_window_icmps: Option<i64>,
    pub lock_dropped_icmps: Option<i64>,
    pub arp_filter: Option<i64>,
    pub tw: Option<i64>,
    pub tw_recycled: Option<i64>,
    pub tw_killed: Option<i64>,
    pub paws_active: Option<i64>,
    pub paws_estab: Option<i64>,
    pub delayed_acks: Option<i64>,
    pub delayed_ack_locked: Option<i64>,
    pub delayed_ack_lost: Option<i64>,
    pub listen_overflows: Option<i64>,
    pub listen_drops: Option<i64>,
    pub tcp_hp_hits: Option<i64>,
    pub tcp_pure_acks: Option<i64>,
    pub tcp_hp_acks: Option<i64>,
    pub tcp_reno_recovery: Option<i64>,
    pub tcp_s_ack_recovery: Option<i64>,
    pub tcp_s_ack_reneging: Option<i64>,
    pub tcp_s_ack_reorder: Option<i64>,
    pub tcp_reno_reorder: Option<i64>,
    pub tcp_ts_reorder: Option<i64>,
    pub tcp_full_undo: Option<i64>,
    pub tcp_partial_undo: Option<i64>,
    pub tcp_ds_ack_undo: Option<i64>,
    pub tcp_loss_undo: Option<i64>,
    pub tcp_lost_retransmit: Option<i64>,
    pub tcp_reno_failures: Option<i64>,
    pub tcp_s_ack_failures: Option<i64>,
    pub tcp_loss_failures: Option<i64>,
    pub tcp_fast_retrans: Option<i64>,
    pub tcp_slow_start_retrans: Option<i64>,
    pub tcp_timeouts: Option<i64>,
    pub tcp_loss_probes: Option<i64>,
    pub tcp_loss_probe_recovery: Option<i64>,
    pub tcp_reno_recovery_fail: Option<i64>,
    pub tcp_s_ack_recovery_fail: Option<i64>,
    pub tcp_rcv_collapsed: Option<i64>,
    pub tcp_backlog_coalesce: Option<i64>,
    pub tcp_ds_ack_old_sent: Option<i64>,
    pub tcp_ds_ack_ofo_sent: Option<i64>,
    pub tcp_ds_ack_recv: Option<i64>,
    pub tcp_ds_ack_ofo_recv: Option<i64>,
    pub tcp_abort_on_data: Option<i64>,
    pub tcp_abort_on_close: Option<i64>,
    pub tcp_abort_on_memory: Option<i64>,
    pub tcp_abort_on_timeout: Option<i64>,
    pub tcp_abort_on_linger: Option<i64>,
    pub tcp_abort_failed: Option<i64>,
    pub tcp_memory_pressures: Option<i64>,
    pub tcp_memory_pressures_chrono: Option<i64>,
    pub tcp_s_ack_discard: Option<i64>,
    pub tcp_ds_ack_ignored_old: Option<i64>,
    pub tcp_ds_ack_ignored_no_undo: Option<i64>,
    pub tcp_spurious_rtos: Option<i64>,
    pub tcp_md5_not_found: Option<i64>,
    pub tcp_md5_unexpected: Option<i64>,
    pub tcp_md5_failure: Option<i64>,
    pub tcp_s_ack_shifted: Option<i64>,
    pub tcp_s_ack_merged: Option<i64>,
    pub tcp_s_ack_shift_fallback: Option<i64>,
    pub tcp_backlog_drop: Option<i64>,
    pub pf_memalloc_drop: Option<i64>,
    pub tcp_min_ttl_drop: Option<i64>,
    pub tcp_defer_accept_drop: Option<i64>,
    pub ip_reverse_path_filter: Option<i64>,
    pub tcp_time_wait_overflow: Option<i64>,
    pub tcp_req_q_full_do_cookies: Option<i64>,
    pub tcp_req_q_full_drop: Option<i64>,
    pub tcp_retrans_fail: Option<i64>,
    pub tcp_rcv_coalesce: Option<i64>,
    pub tcp_ofo_queue: Option<i64>,
    pub tcp_ofo_drop: Option<i64>,
    pub tcp_ofo_merge: Option<i64>,
    pub tcp_challenge_ack: Option<i64>,
    pub tcp_syn_challenge: Option<i64>,
    pub tcp_fast_open_active: Option<i64>,
    pub tcp_fast_open_active_fail: Option<i64>,
    pub tcp_fast_open_passive: Option<i64>,
    pub tcp_fast_open_passive_fail: Option<i64>,
    pub tcp_fast_open_listen_overflow: Option<i64>,
    pub tcp_fast_open_cookie_reqd: Option<i64>,
    pub tcp_fast_open_blackhole: Option<i64>,
    pub tcp_spurious_rtx_host_queues: Option<i64>,
    pub busy_poll_rx_packets: Option<i64>,
    pub tcp_auto_corking: Option<i64>,
    pub tcp_from_zero_window_adv: Option<i64>,
    pub tcp_to_zero_window_adv: Option<i64>,
    pub tcp_want_zero_window_adv: Option<i64>,
    pub tcp_syn_retrans: Option<i64>,
    pub tcp_orig_data_sent: Option<i64>,
    pub tcp_hystart_train_detect: Option<i64>,
    pub tcp_hystart_train_cwnd: Option<i64>,
    pub tcp_hystart_delay_detect: Option<i64>,
    pub tcp_hystart_delay_cwnd: Option<i64>,
    pub tcp_ack_skipped_syn_recv: Option<i64>,
    pub tcp_ack_skipped_paws: Option<i64>,
    pub tcp_ack_skipped_seq: Option<i64>,
    pub tcp_ack_skipped_fin_wait2: Option<i64>,
    pub tcp_ack_skipped_time_wait: Option<i64>,
    pub tcp_ack_skipped_challenge: Option<i64>,
    pub tcp_win_probe: Option<i64>,
    pub tcp_keep_alive: Option<i64>,
    pub tcp_mtup_fail: Option<i64>,
    pub tcp_mtup_success: Option<i64>,
    pub tcp_delivered: Option<i64>,
    pub tcp_delivered_ce: Option<i64>,
    pub tcp_ack_compressed: Option<i64>,
    pub tcp_zero_window_drop: Option<i64>,
    pub tcp_rcv_q_drop: Option<i64>,
    pub tcp_wqueue_too_big: Option<i64>,
    pub tcp_fast_open_passive_altkey: Option<i64>,
    pub tcp_timeout_rehash: Option<i64>,
    pub tcp_duplicate_data_rehash: Option<i64>,
    pub tcp_ds_ack_recv_segs: Option<i64>,
    pub tcp_ds_ack_ignored_dubious: Option<i64>,
    pub tcp_migrate_req_success: Option<i64>,
    pub tcp_migrate_req_failure: Option<i64>,
    pub tcp_plb_rehash: Option<i64>,
    pub tcp_ao_required: Option<i64>,
    pub tcp_ao_bad: Option<i64>,
    pub tcp_ao_key_not_found: Option<i64>,
    pub tcp_ao_good: Option<i64>,
    pub tcp_ao_dropped_icmps: Option<i64>,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct IpExt {
    pub in_no_routes: Option<i64>,
    pub in_truncated_pkts: Option<i64>,
    pub in_mcast_pkts: Option<i64>,
    pub out_mcast_pkts: Option<i64>,
    pub in_bcast_pkts: Option<i64>,
    pub out_bcast_pkts: Option<i64>,
    pub in_octets: Option<i64>,
    pub out_octets: Option<i64>,
    pub in_mcast_octets: Option<i64>,
    pub out_mcast_octets: Option<i64>,
    pub in_bcast_octets: Option<i64>,
    pub out_bcast_octets: Option<i64>,
    pub in_csum_errors: Option<i64>,
    pub in_no_ect_pkts: Option<i64>,
    pub in_ect1_pkts: Option<i64>,
    pub in_ect0_pkts: Option<i64>,
    pub in_ce_pkts: Option<i64>,
    pub reasm_overlaps: Option<i64>,
}

impl ProcessNetstat {
    fn new() -> Self {
        Self {
            tcp_ext: TcpExt::new(),
            ip_ext: IpExt::new(),
        }
    }
}

impl TcpExt {
    fn new() -> Self {
        Default::default()
    }
}

impl IpExt {
    fn new() -> Self {
        Default::default()
    }
}

impl Process {
    /// netstat returns the current netstat of the process
    pub fn netstat(&self) -> CollectResult<ProcessNetstat> {
        let mut proc_netstat = ProcessNetstat::new();
        let proc_netstat_path_str = format!("{:?}/net", self.path());
        let proc_netstat_file = format!("{}/netstat", proc_netstat_path_str.replace("\"", ""));

        let netstat_data = utils::read_file_lines(&proc_netstat_file)?;
        let mut line_index = 0;

        while line_index < netstat_data.len() {
            let header_line = &netstat_data[line_index].to_lowercase();
            line_index += 1;

            if line_index >= netstat_data.len() {
                break;
            }

            let value_line = &netstat_data[line_index].to_lowercase();

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
                    "process netstat header".to_string(),
                    header_sp.len(),
                    header_line.to_string(),
                ));
            }

            if value_sp.len() != 2 {
                return Err(MetricError::InvalidFieldNumberError(
                    "process netstat header".to_string(),
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
                    "process netstat mismatch field count mismatch header".to_string(),
                    header_data.len(),
                    header_data.len().to_string(),
                ));
            }

            match header_sp[0] {
                "tcpext" => {
                    let mut data_index = 0;
                    while data_index < header_data.len() {
                        match header_data[data_index] {
                            "syncookiessent" => {
                                proc_netstat.tcp_ext.syn_cookies_sent =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "syncookiesrecv" => {
                                proc_netstat.tcp_ext.syn_cookies_recv =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "syncookiesfailed" => {
                                proc_netstat.tcp_ext.syn_cookies_failed =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "embryonicrsts" => {
                                proc_netstat.tcp_ext.embryonic_rsts =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "prunecalled" => {
                                proc_netstat.tcp_ext.prune_called =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "rcvpruned" => {
                                proc_netstat.tcp_ext.rcv_pruned =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "ofopruned" => {
                                proc_netstat.tcp_ext.ofo_pruned =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outofwindowicmps" => {
                                proc_netstat.tcp_ext.out_of_window_icmps =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "lockdroppedicmps" => {
                                proc_netstat.tcp_ext.lock_dropped_icmps =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "arpfilter" => {
                                proc_netstat.tcp_ext.arp_filter =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tw" => {
                                proc_netstat.tcp_ext.tw =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "twrecycled" => {
                                proc_netstat.tcp_ext.tw_recycled =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "twkilled" => {
                                proc_netstat.tcp_ext.tw_killed =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "pawsactive" => {
                                proc_netstat.tcp_ext.paws_active =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "pawsestab" => {
                                proc_netstat.tcp_ext.paws_estab =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "delayedacks" => {
                                proc_netstat.tcp_ext.delayed_acks =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "delayedacklocked" => {
                                proc_netstat.tcp_ext.delayed_ack_locked =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "delayedacklost" => {
                                proc_netstat.tcp_ext.delayed_ack_lost =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "listenoverflows" => {
                                proc_netstat.tcp_ext.listen_overflows =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "listendrops" => {
                                proc_netstat.tcp_ext.listen_drops =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcphphits" => {
                                proc_netstat.tcp_ext.tcp_hp_hits =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcppureacks" => {
                                proc_netstat.tcp_ext.tcp_pure_acks =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcphpacks" => {
                                proc_netstat.tcp_ext.tcp_hp_acks =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcprenorecovery" => {
                                proc_netstat.tcp_ext.tcp_reno_recovery =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpsackrecovery" => {
                                proc_netstat.tcp_ext.tcp_s_ack_recovery =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpsackreneging" => {
                                proc_netstat.tcp_ext.tcp_s_ack_reneging =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpsackreorder" => {
                                proc_netstat.tcp_ext.tcp_s_ack_reorder =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcprenoreorder" => {
                                proc_netstat.tcp_ext.tcp_reno_reorder =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcptsreorder" => {
                                proc_netstat.tcp_ext.tcp_ts_reorder =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpfullundo" => {
                                proc_netstat.tcp_ext.tcp_full_undo =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcppartialundo" => {
                                proc_netstat.tcp_ext.tcp_partial_undo =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpdsackundo" => {
                                proc_netstat.tcp_ext.tcp_ds_ack_undo =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcplossundo" => {
                                proc_netstat.tcp_ext.tcp_loss_undo =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcplostretransmit" => {
                                proc_netstat.tcp_ext.tcp_lost_retransmit =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcprenofailures" => {
                                proc_netstat.tcp_ext.tcp_reno_failures =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpsackfailures" => {
                                proc_netstat.tcp_ext.tcp_s_ack_failures =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcplossfailures" => {
                                proc_netstat.tcp_ext.tcp_loss_failures =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpfastretrans" => {
                                proc_netstat.tcp_ext.tcp_fast_retrans =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpslowstartretrans" => {
                                proc_netstat.tcp_ext.tcp_slow_start_retrans =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcptimeouts" => {
                                proc_netstat.tcp_ext.tcp_timeouts =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcplossprobes" => {
                                proc_netstat.tcp_ext.tcp_loss_probes =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcplossproberecovery" => {
                                proc_netstat.tcp_ext.tcp_loss_probe_recovery =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcprenorecoveryfail" => {
                                proc_netstat.tcp_ext.tcp_reno_recovery_fail =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpsackrecoveryfail" => {
                                proc_netstat.tcp_ext.tcp_s_ack_recovery_fail =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcprcvcollapsed" => {
                                proc_netstat.tcp_ext.tcp_rcv_collapsed =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpbacklogcoalesce" => {
                                proc_netstat.tcp_ext.tcp_backlog_coalesce =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpdsackoldsent" => {
                                proc_netstat.tcp_ext.tcp_ds_ack_old_sent =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpdsackofosent" => {
                                proc_netstat.tcp_ext.tcp_ds_ack_ofo_sent =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpdsackrecv" => {
                                proc_netstat.tcp_ext.tcp_ds_ack_recv =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpdsackoforecv" => {
                                proc_netstat.tcp_ext.tcp_ds_ack_ofo_recv =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpabortondata" => {
                                proc_netstat.tcp_ext.tcp_abort_on_data =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpabortonclose" => {
                                proc_netstat.tcp_ext.tcp_abort_on_close =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpabortonmemory" => {
                                proc_netstat.tcp_ext.tcp_abort_on_memory =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpabortontimeout" => {
                                proc_netstat.tcp_ext.tcp_abort_on_timeout =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpabortonlinger" => {
                                proc_netstat.tcp_ext.tcp_abort_on_linger =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpabortfailed" => {
                                proc_netstat.tcp_ext.tcp_abort_failed =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpmemorypressures" => {
                                proc_netstat.tcp_ext.tcp_memory_pressures =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpmemorypressureschrono" => {
                                proc_netstat.tcp_ext.tcp_memory_pressures_chrono =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpsackdiscard" => {
                                proc_netstat.tcp_ext.tcp_s_ack_discard =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpdsackignoredold" => {
                                proc_netstat.tcp_ext.tcp_ds_ack_ignored_old =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpdsackignorednoundo" => {
                                proc_netstat.tcp_ext.tcp_ds_ack_ignored_no_undo =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpspuriousrtos" => {
                                proc_netstat.tcp_ext.tcp_spurious_rtos =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpmd5notfound" => {
                                proc_netstat.tcp_ext.tcp_md5_not_found =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpmd5unexpected" => {
                                proc_netstat.tcp_ext.tcp_md5_unexpected =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpmd5failure" => {
                                proc_netstat.tcp_ext.tcp_md5_failure =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpsackshifted" => {
                                proc_netstat.tcp_ext.tcp_s_ack_shifted =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpsackmerged" => {
                                proc_netstat.tcp_ext.tcp_s_ack_merged =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpsackshiftfallback" => {
                                proc_netstat.tcp_ext.tcp_s_ack_shift_fallback =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpbacklogdrop" => {
                                proc_netstat.tcp_ext.tcp_backlog_drop =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "pfmemallocdrop" => {
                                proc_netstat.tcp_ext.pf_memalloc_drop =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpminttldrop" => {
                                proc_netstat.tcp_ext.tcp_min_ttl_drop =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpdeferacceptdrop" => {
                                proc_netstat.tcp_ext.tcp_defer_accept_drop =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "ipreversepathfilter" => {
                                proc_netstat.tcp_ext.ip_reverse_path_filter =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcptimewaitoverflow" => {
                                proc_netstat.tcp_ext.tcp_time_wait_overflow =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpreqqfulldocookies" => {
                                proc_netstat.tcp_ext.tcp_req_q_full_do_cookies =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpreqqfulldrop" => {
                                proc_netstat.tcp_ext.tcp_req_q_full_drop =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpretransfail" => {
                                proc_netstat.tcp_ext.tcp_retrans_fail =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcprcvcoalesce" => {
                                proc_netstat.tcp_ext.tcp_rcv_coalesce =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpofoqueue" => {
                                proc_netstat.tcp_ext.tcp_ofo_queue =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpofodrop" => {
                                proc_netstat.tcp_ext.tcp_ofo_drop =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpofomerge" => {
                                proc_netstat.tcp_ext.tcp_ofo_merge =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpchallengeack" => {
                                proc_netstat.tcp_ext.tcp_challenge_ack =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpsynchallenge" => {
                                proc_netstat.tcp_ext.tcp_syn_challenge =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpfastopenactive" => {
                                proc_netstat.tcp_ext.tcp_fast_open_active =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpfastopenactivefail" => {
                                proc_netstat.tcp_ext.tcp_fast_open_active_fail =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpfastopenpassive" => {
                                proc_netstat.tcp_ext.tcp_fast_open_passive =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpfastopenpassivefail" => {
                                proc_netstat.tcp_ext.tcp_fast_open_passive_fail =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpfastopenlistenoverflow" => {
                                proc_netstat.tcp_ext.tcp_fast_open_listen_overflow =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpfastopencookiereqd" => {
                                proc_netstat.tcp_ext.tcp_fast_open_cookie_reqd =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpfastopenblackhole" => {
                                proc_netstat.tcp_ext.tcp_fast_open_blackhole =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpspuriousrtxhostqueues" => {
                                proc_netstat.tcp_ext.tcp_spurious_rtx_host_queues =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "busypollrxpackets" => {
                                proc_netstat.tcp_ext.busy_poll_rx_packets =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpautocorking" => {
                                proc_netstat.tcp_ext.tcp_auto_corking =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpfromzerowindowadv" => {
                                proc_netstat.tcp_ext.tcp_from_zero_window_adv =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcptozerowindowadv" => {
                                proc_netstat.tcp_ext.tcp_to_zero_window_adv =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpwantzerowindowadv" => {
                                proc_netstat.tcp_ext.tcp_want_zero_window_adv =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpsynretrans" => {
                                proc_netstat.tcp_ext.tcp_syn_retrans =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcporigdatasent" => {
                                proc_netstat.tcp_ext.tcp_orig_data_sent =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcphystarttraindetect" => {
                                proc_netstat.tcp_ext.tcp_hystart_train_detect =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcphystarttraincwnd" => {
                                proc_netstat.tcp_ext.tcp_hystart_train_cwnd =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcphystartdelaydetect" => {
                                proc_netstat.tcp_ext.tcp_hystart_delay_detect =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcphystartdelaycwnd" => {
                                proc_netstat.tcp_ext.tcp_hystart_delay_cwnd =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpackskippedsynrecv" => {
                                proc_netstat.tcp_ext.tcp_ack_skipped_syn_recv =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpackskippedpaws" => {
                                proc_netstat.tcp_ext.tcp_ack_skipped_paws =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpackskippedseq" => {
                                proc_netstat.tcp_ext.tcp_ack_skipped_seq =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpackskippedfinwait2" => {
                                proc_netstat.tcp_ext.tcp_ack_skipped_fin_wait2 =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpackskippedtimewait" => {
                                proc_netstat.tcp_ext.tcp_ack_skipped_time_wait =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpackskippedchallenge" => {
                                proc_netstat.tcp_ext.tcp_ack_skipped_challenge =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpwinprobe" => {
                                proc_netstat.tcp_ext.tcp_win_probe =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpkeepalive" => {
                                proc_netstat.tcp_ext.tcp_keep_alive =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpmtupfail" => {
                                proc_netstat.tcp_ext.tcp_mtup_fail =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpmtupsuccess" => {
                                proc_netstat.tcp_ext.tcp_mtup_success =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpdelivered" => {
                                proc_netstat.tcp_ext.tcp_delivered =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpdeliveredce" => {
                                proc_netstat.tcp_ext.tcp_delivered_ce =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpackcompressed" => {
                                proc_netstat.tcp_ext.tcp_ack_compressed =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpzerowindowdrop" => {
                                proc_netstat.tcp_ext.tcp_zero_window_drop =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcprcvqdrop" => {
                                proc_netstat.tcp_ext.tcp_rcv_q_drop =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpwqueuetoobig" => {
                                proc_netstat.tcp_ext.tcp_wqueue_too_big =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpfastopenpassivealtkey" => {
                                proc_netstat.tcp_ext.tcp_fast_open_passive_altkey =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcptimeoutrehash" => {
                                proc_netstat.tcp_ext.tcp_timeout_rehash =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpduplicatedatarehash" => {
                                proc_netstat.tcp_ext.tcp_duplicate_data_rehash =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpdsackrecvsegs" => {
                                proc_netstat.tcp_ext.tcp_ds_ack_recv_segs =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpdsackignoreddubious" => {
                                proc_netstat.tcp_ext.tcp_ds_ack_ignored_dubious =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpmigratereqsuccess" => {
                                proc_netstat.tcp_ext.tcp_migrate_req_success =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpmigratereqfailure" => {
                                proc_netstat.tcp_ext.tcp_migrate_req_failure =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpplbrehash" => {
                                proc_netstat.tcp_ext.tcp_plb_rehash =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpaorequired" => {
                                proc_netstat.tcp_ext.tcp_ao_required =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpaobad" => {
                                proc_netstat.tcp_ext.tcp_ao_bad =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpaokeynotfound" => {
                                proc_netstat.tcp_ext.tcp_ao_key_not_found =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpaogood" => {
                                proc_netstat.tcp_ext.tcp_ao_good =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "tcpaodroppedicmps" => {
                                proc_netstat.tcp_ext.tcp_ao_dropped_icmps =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            _ => {}
                        }

                        data_index += 1
                    }
                }
                "ipext" => {
                    let mut data_index = 0;
                    while data_index < header_data.len() {
                        match header_data[data_index] {
                            "innoroutes" => {
                                proc_netstat.ip_ext.in_no_routes =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "intruncatedpkts" => {
                                proc_netstat.ip_ext.in_truncated_pkts =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inmcastpkts" => {
                                proc_netstat.ip_ext.in_mcast_pkts =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outmcastpkts" => {
                                proc_netstat.ip_ext.out_mcast_pkts =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inbcastpkts" => {
                                proc_netstat.ip_ext.in_bcast_pkts =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outbcastpkts" => {
                                proc_netstat.ip_ext.out_bcast_pkts =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inoctets" => {
                                proc_netstat.ip_ext.in_octets =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outoctets" => {
                                proc_netstat.ip_ext.out_octets =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inmcastoctets" => {
                                proc_netstat.ip_ext.in_mcast_octets =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outmcastoctets" => {
                                proc_netstat.ip_ext.out_mcast_octets =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inbcastoctets" => {
                                proc_netstat.ip_ext.in_bcast_octets =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "outbcastoctets" => {
                                proc_netstat.ip_ext.out_bcast_octets =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "incsumerrors" => {
                                proc_netstat.ip_ext.in_csum_errors =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "innoectpkts" => {
                                proc_netstat.ip_ext.in_no_ect_pkts =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inect1pkts" => {
                                proc_netstat.ip_ext.in_ect1_pkts =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "inect0pkts" => {
                                proc_netstat.ip_ext.in_ect0_pkts =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "incepkts" => {
                                proc_netstat.ip_ext.in_ce_pkts =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            "reasmoverlaps" => {
                                proc_netstat.ip_ext.reasm_overlaps =
                                    Some(utils::convert_str_to_i64(value_data[data_index])?);
                            }
                            _ => {}
                        }

                        data_index += 1
                    }
                }
                _ => {}
            }

            line_index += 1;
        }

        Ok(proc_netstat)
    }
}

#[cfg(test)]
mod tests {
    use crate::process::*;
    use std::path::Path;

    #[test]
    fn proc_netstat() {
        let proc_path = Path::new("test_data/fixtures/proc");
        let sys_proc = collect_from(proc_path, 26231).expect("running proc 26231");
        let sys_proc_netstat = sys_proc.netstat().expect("running proc 26231 netstat");

        assert_eq!(sys_proc_netstat.tcp_ext.syn_cookies_sent.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.syn_cookies_recv.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.syn_cookies_failed.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.embryonic_rsts.unwrap(), 1);
        assert_eq!(sys_proc_netstat.tcp_ext.prune_called.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.rcv_pruned.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.ofo_pruned.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.out_of_window_icmps.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.lock_dropped_icmps.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.arp_filter.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tw.unwrap(), 83);
        assert_eq!(sys_proc_netstat.tcp_ext.tw_recycled.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tw_killed.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.paws_active.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.paws_estab.unwrap(), 3640);
        assert_eq!(sys_proc_netstat.tcp_ext.delayed_acks.unwrap(), 287);
        assert_eq!(sys_proc_netstat.tcp_ext.delayed_ack_locked.unwrap(), 1);
        assert_eq!(sys_proc_netstat.tcp_ext.delayed_ack_lost.unwrap(), 7460);
        assert_eq!(sys_proc_netstat.tcp_ext.listen_overflows.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.listen_drops.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_hp_hits.unwrap(), 134193);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_pure_acks.unwrap(), 1335);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_hp_acks.unwrap(), 829);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_reno_recovery.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_s_ack_recovery.unwrap(), 4);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_s_ack_reneging.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_s_ack_reorder.unwrap(), 1);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_reno_reorder.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_ts_reorder.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_full_undo.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_partial_undo.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_ds_ack_undo.unwrap(), 1);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_loss_undo.unwrap(), 19);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_lost_retransmit.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_reno_failures.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_s_ack_failures.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_loss_failures.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_fast_retrans.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_slow_start_retrans.unwrap(), 3);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_timeouts.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_loss_probes.unwrap(), 32);
        assert_eq!(
            sys_proc_netstat.tcp_ext.tcp_loss_probe_recovery.unwrap(),
            100
        );
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_reno_recovery_fail.unwrap(), 4);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_s_ack_recovery_fail.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_rcv_collapsed.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_ds_ack_old_sent.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_ds_ack_ofo_sent.unwrap(), 7460);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_ds_ack_ofo_recv.unwrap(), 49);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_abort_on_data.unwrap(), 1);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_abort_on_close.unwrap(), 62);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_abort_on_memory.unwrap(), 6);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_abort_on_timeout.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_abort_on_linger.unwrap(), 23);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_abort_failed.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_memory_pressures.unwrap(), 7);
        assert_eq!(
            sys_proc_netstat
                .tcp_ext
                .tcp_memory_pressures_chrono
                .unwrap(),
            0
        );
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_s_ack_discard.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_ds_ack_ignored_old.unwrap(), 0);
        assert_eq!(
            sys_proc_netstat.tcp_ext.tcp_ds_ack_ignored_no_undo.unwrap(),
            0
        );
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_spurious_rtos.unwrap(), 19);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_md5_not_found.unwrap(), 2);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_md5_unexpected.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_md5_failure.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_s_ack_shifted.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_s_ack_merged.unwrap(), 0);
        assert_eq!(
            sys_proc_netstat.tcp_ext.tcp_s_ack_shift_fallback.unwrap(),
            0
        );
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_backlog_drop.unwrap(), 6);
        assert_eq!(sys_proc_netstat.tcp_ext.pf_memalloc_drop.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_min_ttl_drop.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_defer_accept_drop.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.ip_reverse_path_filter.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_timeout_rehash, None);
        assert_eq!(
            sys_proc_netstat.tcp_ext.tcp_req_q_full_do_cookies.unwrap(),
            0
        );
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_req_q_full_drop.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_retrans_fail.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_rcv_coalesce.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_rcv_q_drop.unwrap(), 92425);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_ofo_queue.unwrap(), 65515);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_ofo_drop.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_ofo_merge.unwrap(), 2421);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_challenge_ack.unwrap(), 4);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_syn_challenge.unwrap(), 4);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_fast_open_active.unwrap(), 0);
        assert_eq!(
            sys_proc_netstat.tcp_ext.tcp_fast_open_active_fail.unwrap(),
            0
        );
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_fast_open_passive.unwrap(), 0);
        assert_eq!(
            sys_proc_netstat.tcp_ext.tcp_fast_open_passive_fail.unwrap(),
            0
        );
        assert_eq!(
            sys_proc_netstat
                .tcp_ext
                .tcp_fast_open_listen_overflow
                .unwrap(),
            0
        );
        assert_eq!(
            sys_proc_netstat.tcp_ext.tcp_fast_open_cookie_reqd.unwrap(),
            0
        );
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_fast_open_blackhole.unwrap(), 0);
        assert_eq!(
            sys_proc_netstat
                .tcp_ext
                .tcp_spurious_rtx_host_queues
                .unwrap(),
            0
        );
        assert_eq!(sys_proc_netstat.tcp_ext.busy_poll_rx_packets.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_auto_corking.unwrap(), 10);
        assert_eq!(
            sys_proc_netstat.tcp_ext.tcp_from_zero_window_adv.unwrap(),
            0
        );
        assert_eq!(
            sys_proc_netstat.tcp_ext.tcp_want_zero_window_adv.unwrap(),
            0
        );
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_syn_retrans.unwrap(), 16);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_orig_data_sent.unwrap(), 2221);
        assert_eq!(
            sys_proc_netstat.tcp_ext.tcp_hystart_train_detect.unwrap(),
            0
        );
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_hystart_train_cwnd.unwrap(), 0);
        assert_eq!(
            sys_proc_netstat.tcp_ext.tcp_hystart_delay_detect.unwrap(),
            2
        );
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_hystart_delay_cwnd.unwrap(), 45);
        assert_eq!(
            sys_proc_netstat.tcp_ext.tcp_ack_skipped_syn_recv.unwrap(),
            0
        );
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_ack_skipped_paws.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_ack_skipped_seq.unwrap(), 3);
        assert_eq!(
            sys_proc_netstat.tcp_ext.tcp_ack_skipped_fin_wait2.unwrap(),
            0
        );
        assert_eq!(
            sys_proc_netstat.tcp_ext.tcp_ack_skipped_time_wait.unwrap(),
            0
        );
        assert_eq!(
            sys_proc_netstat.tcp_ext.tcp_ack_skipped_challenge.unwrap(),
            0
        );
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_win_probe.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_keep_alive.unwrap(), 456);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_mtup_fail.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_mtup_success.unwrap(), 0);
        assert_eq!(sys_proc_netstat.tcp_ext.tcp_wqueue_too_big.unwrap(), 0);

        assert_eq!(sys_proc_netstat.ip_ext.in_no_routes.unwrap(), 0);
        assert_eq!(sys_proc_netstat.ip_ext.in_truncated_pkts.unwrap(), 0);
        assert_eq!(sys_proc_netstat.ip_ext.in_mcast_pkts.unwrap(), 208);
        assert_eq!(sys_proc_netstat.ip_ext.out_mcast_pkts.unwrap(), 214);
        assert_eq!(sys_proc_netstat.ip_ext.in_bcast_pkts.unwrap(), 118);
        assert_eq!(sys_proc_netstat.ip_ext.out_bcast_pkts.unwrap(), 111);
        assert_eq!(sys_proc_netstat.ip_ext.in_octets.unwrap(), 190585481);
        assert_eq!(sys_proc_netstat.ip_ext.out_octets.unwrap(), 7512674);
        assert_eq!(sys_proc_netstat.ip_ext.in_mcast_octets.unwrap(), 26093);
        assert_eq!(sys_proc_netstat.ip_ext.out_mcast_octets.unwrap(), 25903);
        assert_eq!(sys_proc_netstat.ip_ext.in_bcast_octets.unwrap(), 14546);
        assert_eq!(sys_proc_netstat.ip_ext.out_bcast_octets.unwrap(), 13628);
        assert_eq!(sys_proc_netstat.ip_ext.in_csum_errors.unwrap(), 0);
        assert_eq!(sys_proc_netstat.ip_ext.in_no_ect_pkts.unwrap(), 134215);
        assert_eq!(sys_proc_netstat.ip_ext.in_ect1_pkts.unwrap(), 0);
        assert_eq!(sys_proc_netstat.ip_ext.in_ect0_pkts.unwrap(), 0);
        assert_eq!(sys_proc_netstat.ip_ext.in_ce_pkts.unwrap(), 0);
        assert_eq!(sys_proc_netstat.ip_ext.reasm_overlaps.unwrap(), 0);

        let sys_proc = collect_from(proc_path, 26234).expect("running proc 26234");
        let sys_proc_netstat = sys_proc.netstat();
        assert_eq!(sys_proc_netstat.is_err(), true);
    }
}
