Supported Features
* ✅ `/proc/<pid>`
    * cgroup
    * cmdline
    * comm
    * cwd
    * environ
    * exe
    * io
    * limits
    * root
    * ns

* ✅ `/proc/<pid>/net`
    * netstat
    * snmp
    * snmp6

* ✅ `/proc/buddyinfo`

* ✅ `/proc/cmdline`

* ✅ `/proc/crypto`

* ✅ `/proc/cpuinfo`

* ✅ `/proc/loadavg`

* ✅ `/proc/meminfo`

* ✅ `/proc/net/`
    * arp
    * dev
    * protocols
    * sockstat
    * sockstat6
    * unix
    * wireless

* ✅ `/proc/sys/kernel/random`
    * entropy_avail
    * poolsize
    * urandom_min_reseed_secs
    * write_wakeup_threshold
    * read_wakeup_threshold

* ✅ `/proc/softirqs`

* ✅ `/proc/swaps`

* ✅ `/sys/class/dmi/id`
    * bios_date
    * bios_release
    * bios_vendor
    * bios_version
    * board_asset_tag
    * board_name
    * board_serial
    * board_vendor
    * board_version
    * chassis_asset_tag
    * chassis_serial
    * chassis_type
    * chassis_vendor
    * product_family
    * product_name
    * product_serial
    * product_sku
    * product_uuid
    * sys_vendor

* ✅ `/sys/class/fc_host/<NAME>`
    * speed
    * port_state
    * port_type
    * symbolic_name
    * node_name
    * port_id
    * port_name
    * fabric_name
    * dev_loss_tmo
    * supported_classes
    * supported_speeds
    * statistics/
        * dumped_frames
        * error_frames
        * invalid_crc_count
        * rx_frames
        * rx_words
        * tx_frames
        * tx_words
        * seconds_since_last_reset
        * invalid_tx_word_count
        * link_failure_count
        * loss_of_sync_count
        * loss_of_signal_count
        * nos_count
        * fcp_packet_aborts

* ✅ `/sys/class/infiniband/<NAME>`
    * board_id
    * fw_ver
    * hca_type
    * ports/`<PORT>`
        * state
        * phys_state
        * rate
        * counters
            * excessive_buffer_overrun_errors
            * link_downed
            * link_error_recovery
            * local_link_integrity_errors
            * multicast_rcv_packets
            * multicast_xmit_packets
            * port_rcv_constraint_errors
            * port_rcv_data
            * port_rcv_discards
            * port_rcv_errors
            * port_rcv_packets
            * port_rcv_remote_physical_errors
            * port_rcv_switch_relay_errors
            * port_xmit_constraint_errors
            * port_xmit_data
            * port_xmit_discards
            * port_xmit_packets
            * port_xmit_wait
            * symbol_error
            * unicast_rcv_packets
            * unicast_xmit_packets
            * VL15_dropped
        * counters_ext
            * port_multicast_rcv_packets
            * port_multicast_xmit_packets
            * port_rcv_data_64
            * port_rcv_packets_64
            * port_unicast_rcv_packets
            * port_unicast_xmit_packets
            * port_xmit_data_64
            * port_xmit_packets_64
        * hw_counters
            * duplicate_request
            * implied_nak_seq_err
            * lifespan
            * local_ack_timeout_err
            * np_cnp_sent
            * np_ecn_marked_roce_packets
            * out_of_buffer
            * out_of_sequence
            * packet_seq_err
            * req_cqe_error
            * req_cqe_flush_error
            * req_remote_access_errors
            * req_remote_invalid_request
            * resp_cqe_error
            * resp_cqe_flush_error
            * resp_local_length_error
            * resp_remote_access_errors
            * rnr_nak_retry_err
            * roce_adp_retrans
            * roce_adp_retrans_to
            * roce_slow_restart
            * roce_slow_restart_cnps
            * roce_slow_restart_trans
            * rp_cnp_handled
            * rp_cnp_ignored
            * rx_atomic_requests
            * rx_dct_connect
            * rx_icrc_encapsulated
            * rx_read_requests
            * rx_write_requests

* ✅ `/sys/class/nvme/<NAME>`
    * serial
    * model
    * state
    * firmware_rev

* ✅ `/sys/class/power_supply/<NAME>`
    * authentic
    * calibrate
    * capacity
    * capacity_alert_max
    * capacity_alert_min
    * capacity_level
    * charge_avg
    * charge_control_limit
    * charge_control_limit_max
    * charge_counter
    * charge_empty
    * charge_empty_design
    * charge_start_threshold
    * charge_stop_threshold
    * charge_full
    * charge_full_design
    * charge_now
    * charge_term_current
    * charge_type
    * constant_charge_current
    * constant_charge_current_max
    * constant_charge_voltage
    * constant_charge_voltage_max
    * current_avg
    * current_boot
    * current_max
    * current_now
    * cycle_count
    * energy_avg
    * energy_empty
    * energy_empty_design
    * energy_full
    * energy_full_design
    * energy_now
    * health
    * input_current_limit
    * manufacturer
    * model_name
    * online
    * power_avg
    * power_now
    * precharge_current
    * present
    * scope
    * serial_number
    * status
    * technology
    * temp
    * temp_alert_max
    * temp_alert_min
    * temp_ambient
    * temp_ambient_max
    * temp_ambient_min
    * temp_max
    * temp_min
    * time_to_empty_avg
    * time_to_empty_now
    * time_to_full_avg
    * time_to_full_now
    * type
    * usb_type
    * voltage_avg
    * voltage_boot
    * voltage_max
    * voltage_max_design
    * voltage_min
    * voltage_min_design
    * voltage_now
    * voltage_ocv

* ✅ `/sys/class/sas_device/<NAME>/`
    * sas_address
    * device/
        * phy-*
        * ports-*
        * target*/\*/block/*

* ✅ `/sys/class/sas_host/<NAME>/`
    * device/
        * phy-*
        * ports-*

* ✅ `/sys/class/sas_port/<NAME>/`
    * device/
        * phy-*
        * expander-*
        * end_device-*

* ✅ `/sys/class/sas_phy/<NAME>/`
    * sas_address
    * device/
        * port
    * device_type
    * initiator_port_protocols
    * invalid_dword_count
    * loss_of_dword_sync_count
    * maximum_linkrate
    * maximum_linkrate_hw
    * minimum_linkrate
    * minimum_linkrate_hw
    * negotiated_linkrate
    * phy_identifier
    * phy_reset_problem_count
    * running_disparity_error_count
    * target_port_protocols

* ✅ `/sys/class/scsi_tape/<NAME>/statistics`
    * write_ns
    * read_byte_cnt
    * io_ns
    * write_cnt
    * resid_cnt
    * read_ns
    * in_flight
    * other_cnt
    * read_cnt
    * write_byte_cnt

* ✅ `/sys/class/thermal/cooling_device<X>`
    * type
    * max_state
    * cur_state

* ✅ `/sys/class/thermal/thermal_zone<X>`
    * type
    * temp
    * policy
    * mode
    * passive

* ✅ `/sys/class/watchdog/<name>`
    * bootstatus
    * options
    * fw_version
    * identity
    * nowayout
    * state
    * status
    * timeleft
    * timeout
    * min_timeout
    * max_timeout
    * pretimeout
    * pretimeout_governor
    * access_cs0

* ✅ `/sys/devices/system/clocksource/clocksource<X>`
    * available_clocksource
    * current_clocksource
