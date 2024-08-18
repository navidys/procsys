use std::{collections::HashMap, fs::read_link};

use serde::Serialize;

use crate::{
    error::{CollectResult, MetricError},
    process::Process,
    utils,
};

/// ProcessNamespace represents a single namespace of a process
#[derive(Debug, Serialize, Clone, Default)]
pub struct ProcessNamespace {
    pub ns_type: String,
    pub inode: u32,
}

impl ProcessNamespace {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Process {
    /// Namespaces reads from /proc/\<pid\>/ns/* to get the namespaces of which the process is a member
    pub fn namespaces(&self) -> CollectResult<HashMap<String, ProcessNamespace>> {
        let mut proc_namespaces: HashMap<String, ProcessNamespace> = HashMap::new();

        let mut proc_ns_path = self.path();
        proc_ns_path.push("ns");

        for ns_item in utils::list_dir_content(&proc_ns_path, "", "ns") {
            let mut ns_item_path = proc_ns_path.clone();
            ns_item_path.push(&ns_item);

            match read_link(&ns_item_path) {
                Ok(c) => {
                    let item_fields: Vec<&str> = c
                        .to_str()
                        .unwrap_or_default()
                        .trim()
                        .split(':')
                        .filter(|s| !s.is_empty())
                        .collect();

                    if item_fields.len() != 2 {
                        return Err(MetricError::InvalidFieldNumberError(
                            "process ns item".to_string(),
                            item_fields.len(),
                            c.to_str().unwrap_or_default().to_string(),
                        ));
                    }

                    let mut proc_ns = ProcessNamespace::new();
                    let proc_ns_inode = item_fields[1].trim().trim_matches('[').trim_matches(']');

                    proc_ns.ns_type = item_fields[0].trim().to_string();
                    proc_ns.inode = proc_ns_inode.parse::<u32>().unwrap_or_default();

                    proc_namespaces.insert(ns_item, proc_ns);
                }
                Err(e) => return Err(MetricError::IOError(ns_item_path, e)),
            }
        }

        Ok(proc_namespaces)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::process::*;

    #[test]
    fn proc_ns() {
        let proc_path = Path::new("test_data/fixtures/proc");
        let sys_proc = collect_from(proc_path, 26231).expect("running proc 26231");
        let sys_proc_ns = sys_proc
            .namespaces()
            .expect("running proc 26231 namespaces");

        assert_eq!(sys_proc_ns.len(), 2);
        assert_eq!(sys_proc_ns.contains_key("mnt"), true);
        assert_eq!(sys_proc_ns.contains_key("net"), true);

        for (key, proc_ns) in &sys_proc_ns {
            match key.as_str() {
                "mnt" => {
                    assert_eq!(proc_ns.ns_type, "mnt");
                    assert_eq!(proc_ns.inode, 4026531840);
                }
                "net" => {
                    assert_eq!(proc_ns.ns_type, "net");
                    assert_eq!(proc_ns.inode, 4026531993);
                }
                _ => panic!("invalid namespace key: {}", key),
            }
        }
    }
}
