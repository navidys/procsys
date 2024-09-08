use serde::Serialize;

use crate::{error::CollectResult, utils};

/// Softirqs represents the softirq statistics
#[derive(Debug, Serialize, Clone, Default)]
pub struct Softirqs {
    pub hi: Vec<u64>,
    pub timer: Vec<u64>,
    pub net_tx: Vec<u64>,
    pub net_rx: Vec<u64>,
    pub block: Vec<u64>,
    pub irq_poll: Vec<u64>,
    pub tasklet: Vec<u64>,
    pub sched: Vec<u64>,
    pub hr_timer: Vec<u64>,
    pub rcu: Vec<u64>,
}

impl Softirqs {
    fn new() -> Self {
        Default::default()
    }
}

/// collects the the softirq statistics
/// # Example
/// ```
/// use procsys::softirqs;
///
/// let sys_softirqs = softirqs::collect().expect("softirqs information");
/// let json_output = serde_json::to_string_pretty(&sys_softirqs).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<Softirqs> {
    collect_from("/proc/softirqs")
}

fn collect_from(filename: &str) -> CollectResult<Softirqs> {
    let mut proc_softirqs = Softirqs::new();

    let irqsdata = utils::read_file_lines(filename)?;

    for line in &irqsdata[1..] {
        let irqdata = line.to_owned();
        let irq_info: Vec<&str> = irqdata
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect();

        match irq_info[0] {
            "HI:" => {
                for value in &irq_info[1..] {
                    proc_softirqs.hi.push(utils::convert_str_to_u64(value)?);
                }
            }
            "TIMER:" => {
                for value in &irq_info[1..] {
                    proc_softirqs.timer.push(utils::convert_str_to_u64(value)?);
                }
            }
            "NET_TX:" => {
                for value in &irq_info[1..] {
                    proc_softirqs.net_tx.push(utils::convert_str_to_u64(value)?);
                }
            }
            "NET_RX:" => {
                for value in &irq_info[1..] {
                    proc_softirqs.net_rx.push(utils::convert_str_to_u64(value)?);
                }
            }
            "BLOCK:" => {
                for value in &irq_info[1..] {
                    proc_softirqs.block.push(utils::convert_str_to_u64(value)?);
                }
            }
            "IRQ_POLL:" => {
                for value in &irq_info[1..] {
                    proc_softirqs
                        .irq_poll
                        .push(utils::convert_str_to_u64(value)?);
                }
            }
            "TASKLET:" => {
                for value in &irq_info[1..] {
                    proc_softirqs
                        .tasklet
                        .push(utils::convert_str_to_u64(value)?);
                }
            }
            "SCHED:" => {
                for value in &irq_info[1..] {
                    proc_softirqs.sched.push(utils::convert_str_to_u64(value)?);
                }
            }
            "HRTIMER:" => {
                for value in &irq_info[1..] {
                    proc_softirqs
                        .hr_timer
                        .push(utils::convert_str_to_u64(value)?);
                }
            }
            "RCU:" => {
                for value in &irq_info[1..] {
                    proc_softirqs.rcu.push(utils::convert_str_to_u64(value)?);
                }
            }
            _ => {}
        }
    }

    Ok(proc_softirqs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn softirqs_stats() {
        let sys_softirqs = collect_from("test_data/fixtures/proc/softirqs")
            .expect("collecting softirqs information");

        assert_eq!(sys_softirqs.hi, [3, 0]);
        assert_eq!(sys_softirqs.timer, [2776180, 247490]);
        assert_eq!(sys_softirqs.net_tx, [2419, 772]);
        assert_eq!(sys_softirqs.net_rx, [55919, 28694]);
        assert_eq!(sys_softirqs.block, [174915, 262755]);
        assert_eq!(sys_softirqs.irq_poll, [0, 0]);
        assert_eq!(sys_softirqs.tasklet, [209, 75]);
        assert_eq!(sys_softirqs.sched, [2278692, 815209]);
        assert_eq!(sys_softirqs.hr_timer, [1281, 220]);
        assert_eq!(sys_softirqs.rcu, [605871, 532783]);
    }
}
