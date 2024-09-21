use std::{collections::HashMap, path::PathBuf};

use serde::Serialize;

use crate::{error::CollectResult, utils};

/// FibreChannelHost contains info for a single fibrechannel host
#[derive(Debug, Serialize, Clone, Default)]
pub struct FibreChannelHost {
    pub speed: Option<String>,
    pub port_state: Option<String>,
    pub port_type: Option<String>,
    pub symbolic_name: Option<String>,
    pub node_name: Option<String>,
    pub port_id: Option<String>,
    pub port_name: Option<String>,
    pub fabric_name: Option<String>,
    pub dev_loss_tmo: Option<String>,
    pub supported_classes: Option<String>,
    pub supported_speeds: Option<String>,
    pub statistics: FibreChannelHostCounters,
}

#[derive(Debug, Serialize, Clone, Default)]
pub struct FibreChannelHostCounters {
    pub dumped_frames: Option<u64>,
    pub error_frames: Option<u64>,
    pub invalid_crc_count: Option<u64>,
    pub rx_frames: Option<u64>,
    pub rx_words: Option<u64>,
    pub tx_frames: Option<u64>,
    pub tx_words: Option<u64>,
    pub seconds_since_last_reset: Option<u64>,
    pub invalid_tx_word_count: Option<u64>,
    pub link_failure_count: Option<u64>,
    pub loss_of_sync_count: Option<u64>,
    pub loss_of_signal_count: Option<u64>,
    pub nos_count: Option<u64>,
    pub fcp_packet_aborts: Option<u64>,
}

impl FibreChannelHost {
    fn new() -> Self {
        Default::default()
    }
}

impl FibreChannelHostCounters {
    fn new() -> Self {
        Default::default()
    }
}

/// collects the the nvme devices information
/// # Example
/// ```
/// use procsys::sysfs::class_fibrechannel;
///
/// let fc_hosts = class_fibrechannel::collect().expect("fibrechannel information");
/// let json_output = serde_json::to_string_pretty(&fc_hosts).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<HashMap<String, FibreChannelHost>> {
    collect_from("/sys/class/fc_host/")
}

fn collect_from(filename: &str) -> CollectResult<HashMap<String, FibreChannelHost>> {
    let mut fc_hosts: HashMap<String, FibreChannelHost> = HashMap::new();

    let fc_hosts_path = PathBuf::from(filename);
    for fc_host_item in utils::list_dir_content(&fc_hosts_path, "", "fc_host") {
        let mut fc_host_item_path = fc_hosts_path.clone();
        fc_host_item_path.push(&fc_host_item);

        let mut fc_host = FibreChannelHost::new();
        fc_host.speed = utils::collect_info_string("speed", &fc_host_item_path)?;

        fc_host.port_state = utils::collect_info_string("port_state", &fc_host_item_path)?;

        fc_host.port_type = utils::collect_info_string("port_type", &fc_host_item_path)?;

        fc_host.symbolic_name = utils::collect_info_string("symbolic_name", &fc_host_item_path)?;

        fc_host.node_name = utils::collect_info_string("node_name", &fc_host_item_path)?;

        fc_host.port_id = utils::collect_info_string("port_id", &fc_host_item_path)?;

        fc_host.port_name = utils::collect_info_string("port_name", &fc_host_item_path)?;

        fc_host.fabric_name = utils::collect_info_string("fabric_name", &fc_host_item_path)?;

        fc_host.dev_loss_tmo = utils::collect_info_string("dev_loss_tmo", &fc_host_item_path)?;

        fc_host.supported_classes =
            utils::collect_info_string("supported_classes", &fc_host_item_path)?;

        fc_host.supported_speeds =
            utils::collect_info_string("supported_speeds", &fc_host_item_path)?;

        fc_host.statistics = FibreChannelHostCounters::new();
        fc_host_item_path.push("statistics");

        let dumped_frames =
            utils::collect_info_string("dumped_frames", &fc_host_item_path)?.unwrap_or_default();
        if !dumped_frames.is_empty() {
            fc_host.statistics.dumped_frames = Some(utils::convert_hex_to_u64(&dumped_frames)?);
        }

        let error_frames =
            utils::collect_info_string("error_frames", &fc_host_item_path)?.unwrap_or_default();
        if !error_frames.is_empty() {
            fc_host.statistics.error_frames = Some(utils::convert_hex_to_u64(&error_frames)?);
        }

        let invalid_crc_count =
            utils::collect_info_string("invalid_crc_count", &fc_host_item_path)?
                .unwrap_or_default();
        if !invalid_crc_count.is_empty() {
            fc_host.statistics.invalid_crc_count =
                Some(utils::convert_hex_to_u64(&invalid_crc_count)?);
        }

        let rx_frames =
            utils::collect_info_string("rx_frames", &fc_host_item_path)?.unwrap_or_default();
        if !rx_frames.is_empty() {
            fc_host.statistics.rx_frames = Some(utils::convert_hex_to_u64(&rx_frames)?);
        }

        let rx_words =
            utils::collect_info_string("rx_words", &fc_host_item_path)?.unwrap_or_default();
        if !rx_words.is_empty() {
            fc_host.statistics.rx_words = Some(utils::convert_hex_to_u64(&rx_words)?);
        }

        let tx_frames =
            utils::collect_info_string("tx_frames", &fc_host_item_path)?.unwrap_or_default();
        if !tx_frames.is_empty() {
            fc_host.statistics.tx_frames = Some(utils::convert_hex_to_u64(&tx_frames)?);
        }

        let tx_words =
            utils::collect_info_string("tx_words", &fc_host_item_path)?.unwrap_or_default();
        if !tx_words.is_empty() {
            fc_host.statistics.tx_words = Some(utils::convert_hex_to_u64(&tx_words)?);
        }

        let seconds_since_last_reset =
            utils::collect_info_string("seconds_since_last_reset", &fc_host_item_path)?
                .unwrap_or_default();
        if !seconds_since_last_reset.is_empty() {
            fc_host.statistics.seconds_since_last_reset =
                Some(utils::convert_hex_to_u64(&seconds_since_last_reset)?);
        }

        let invalid_tx_word_count =
            utils::collect_info_string("invalid_tx_word_count", &fc_host_item_path)?
                .unwrap_or_default();
        if !invalid_tx_word_count.is_empty() {
            fc_host.statistics.invalid_tx_word_count =
                Some(utils::convert_hex_to_u64(&invalid_tx_word_count)?);
        }

        let link_failure_count =
            utils::collect_info_string("link_failure_count", &fc_host_item_path)?
                .unwrap_or_default();
        if !link_failure_count.is_empty() {
            fc_host.statistics.link_failure_count =
                Some(utils::convert_hex_to_u64(&link_failure_count)?);
        }

        let loss_of_sync_count =
            utils::collect_info_string("loss_of_sync_count", &fc_host_item_path)?
                .unwrap_or_default();
        if !loss_of_sync_count.is_empty() {
            fc_host.statistics.loss_of_sync_count =
                Some(utils::convert_hex_to_u64(&loss_of_sync_count)?);
        }

        let loss_of_signal_count =
            utils::collect_info_string("loss_of_signal_count", &fc_host_item_path)?
                .unwrap_or_default();
        if !loss_of_signal_count.is_empty() {
            fc_host.statistics.loss_of_signal_count =
                Some(utils::convert_hex_to_u64(&loss_of_signal_count)?);
        }

        let nos_count =
            utils::collect_info_string("nos_count", &fc_host_item_path)?.unwrap_or_default();
        if !nos_count.is_empty() {
            fc_host.statistics.nos_count = Some(utils::convert_hex_to_u64(&nos_count)?);
        }

        let fcp_packet_aborts =
            utils::collect_info_string("fcp_packet_aborts", &fc_host_item_path)?
                .unwrap_or_default();
        if !fcp_packet_aborts.is_empty() {
            fc_host.statistics.fcp_packet_aborts =
                Some(utils::convert_hex_to_u64(&fcp_packet_aborts)?);
        }

        fc_hosts.insert(fc_host_item, fc_host);
    }

    Ok(fc_hosts)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fchost_information() {
        let fc_hosts = collect_from("test_data/fixtures/sys/class/fc_host/")
            .expect("collecting fibrechannels information");

        for (name, fc_host) in fc_hosts {
            match name.as_str() {
                "host0" => {
                    assert_eq!(fc_host.speed, Some("16 Gbit".to_string()));
                    assert_eq!(fc_host.port_state, Some("Online".to_string()));
                    assert_eq!(
                        fc_host.port_type,
                        Some("Point-To-Point (direct nport connection)".to_string()),
                    );
                    assert_eq!(
                        fc_host.symbolic_name,
                        Some(
                            "Emulex SN1100E2P FV12.4.270.3 DV12.4.0.0. HN:gotest. OS:Linux"
                                .to_string()
                        )
                    );
                    assert_eq!(fc_host.node_name, Some("0x2000e0071bce95f2".to_string()));
                    assert_eq!(fc_host.port_id, Some("0x000002".to_string()));
                    assert_eq!(fc_host.port_name, Some("0x1000e0071bce95f2".to_string()));
                    assert_eq!(fc_host.fabric_name, Some("0x0".to_string()));
                    assert_eq!(fc_host.dev_loss_tmo, Some("30".to_string()));
                    assert_eq!(fc_host.supported_classes, Some("Class 3".to_string()));
                    assert_eq!(
                        fc_host.supported_speeds,
                        Some("4 Gbit, 8 Gbit, 16 Gbit".to_string()),
                    );
                    assert_eq!(fc_host.statistics.dumped_frames, Some(18446744073709551615));
                    assert_eq!(fc_host.statistics.error_frames, Some(0));
                    assert_eq!(fc_host.statistics.invalid_crc_count, Some(2));
                    assert_eq!(fc_host.statistics.rx_frames, Some(3));
                    assert_eq!(fc_host.statistics.rx_words, Some(4));
                    assert_eq!(fc_host.statistics.tx_frames, Some(5));
                    assert_eq!(fc_host.statistics.tx_words, Some(6));
                    assert_eq!(fc_host.statistics.seconds_since_last_reset, Some(7));
                    assert_eq!(fc_host.statistics.invalid_tx_word_count, Some(8));
                    assert_eq!(fc_host.statistics.link_failure_count, Some(9));
                    assert_eq!(fc_host.statistics.loss_of_sync_count, Some(16));
                    assert_eq!(fc_host.statistics.loss_of_signal_count, Some(17));
                    assert_eq!(fc_host.statistics.nos_count, Some(18));
                    assert_eq!(fc_host.statistics.fcp_packet_aborts, Some(19));
                }
                "host1" => {
                    assert_eq!(fc_host.speed, None);
                    assert_eq!(fc_host.port_state, Some("Online".to_string()));
                    assert_eq!(fc_host.port_type, None);
                    assert_eq!(fc_host.symbolic_name, None);
                    assert_eq!(fc_host.node_name, None);
                    assert_eq!(fc_host.port_id, None);
                    assert_eq!(fc_host.port_name, None);
                    assert_eq!(fc_host.fabric_name, None);
                    assert_eq!(fc_host.dev_loss_tmo, None);
                    assert_eq!(fc_host.supported_classes, None);
                    assert_eq!(fc_host.supported_speeds, None);
                    assert_eq!(fc_host.statistics.dumped_frames, Some(0));
                    assert_eq!(fc_host.statistics.error_frames, Some(18446744073709551615));
                    assert_eq!(fc_host.statistics.invalid_crc_count, Some(32));
                    assert_eq!(fc_host.statistics.rx_frames, Some(48));
                    assert_eq!(fc_host.statistics.rx_words, Some(64));
                    assert_eq!(fc_host.statistics.tx_frames, Some(80));
                    assert_eq!(fc_host.statistics.tx_words, Some(96));
                    assert_eq!(fc_host.statistics.seconds_since_last_reset, Some(112));
                    assert_eq!(fc_host.statistics.invalid_tx_word_count, Some(128));
                    assert_eq!(fc_host.statistics.link_failure_count, Some(144));
                    assert_eq!(fc_host.statistics.loss_of_sync_count, Some(256));
                    assert_eq!(fc_host.statistics.loss_of_signal_count, Some(272));
                    assert_eq!(fc_host.statistics.nos_count, Some(288));
                    assert_eq!(fc_host.statistics.fcp_packet_aborts, Some(304));
                }
                _ => panic!("invalid fibrechannel name: {}", name),
            }
        }
    }
}
