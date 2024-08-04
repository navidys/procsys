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
    collect_from("cmdline", Path::new("/proc"))
}

fn collect_from(filename: &str, base_path: &Path) -> CollectResult<Vec<String>> {
    let mut boot_cmdline: Vec<String> = Vec::new();

    let bootcmd = utils::collect_info_string(filename, base_path)?;

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
        let bootcmd = collect_from("cmdline", Path::new("test_data/fixtures/proc"))
            .expect("system boot cmdline");

        assert_eq!(
            bootcmd,
            [
                "BOOT_IMAGE=/vmlinuz-5.11.0-22-generic",
                "root=UUID=456a0345-450d-4f7b-b7c9-43e3241d99ad",
                "ro",
                "quiet",
                "splash",
                "vt.handoff=7",
            ]
        );
    }
}
