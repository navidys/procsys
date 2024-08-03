use std::path::Path;

use crate::{error::CollectResult, utils};

/// collects information about system boot cmdline
/// # Example
/// ```
/// use procsys::cmdline;
///
/// let sys_cmdline = cmdline::collect().expect("system boot cmdline");
/// println!("{:?}", sys_cmdline);
///
pub fn collect() -> CollectResult<Vec<String>> {
    let mut boot_cmdline: Vec<String> = Vec::new();

    let bootcmd = utils::collect_info_string("cmdline", Path::new("/proc"))?;

    if bootcmd.is_some() {
        boot_cmdline = bootcmd
            .unwrap()
            .split_whitespace()
            .map(str::to_string)
            .collect();
    }

    Ok(boot_cmdline)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sys_bootcmd() {
        let bootcmd = collect().expect("system boot cmdline");
        assert!(!bootcmd.is_empty())
    }
}
