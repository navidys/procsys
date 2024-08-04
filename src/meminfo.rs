use serde::Serialize;

use crate::{
    error::{CollectResult, MetricError},
    utils,
};

enum MeminfoType {
    MemTotal,
    MemFree,
    MemAvailable,
    Buffers,
    Cached,
    SwapCached,
    Active,
    Inactive,
    ActiveAnon,
    InactiveAnon,
    ActiveFile,
    InactiveFile,
    Unevictable,
    Mlocked,
    SwapTotal,
    SwapFree,
    Zswap,
    Zswapped,
    Dirty,
    Writeback,
    AnonPages,
    Mapped,
    Shmem,
    KReclaimable,
    Slab,
    SReclaimable,
    SUnreclaim,
    KernelStack,
    PageTables,
    SecPageTables,
    NfsUnstable,
    Bounce,
    WritebackTmp,
    CommitLimit,
    CommittedAs,
    VmallocTotal,
    VmallocUsed,
    VmallocChunk,
    Percpu,
    HardwareCorrupted,
    AnonHugePages,
    ShmemHugePages,
    ShmemPmdMapped,
    FileHugePages,
    FilePmdMapped,
    CmaTotal,
    CmaFree,
    Unaccepted,
    HugePagesTotal,
    HugePagesFree,
    HugePagesRsvd,
    HugePagesSurp,
    HugepageSize,
    Hugetlb,
    DirectMap4k,
    DirectMap2M,
    DirectMap1G,
    Unknown,
}

impl MeminfoType {
    fn from(name: &str) -> MeminfoType {
        match name {
            "MemTotal" => MeminfoType::MemTotal,
            "MemFree" => MeminfoType::MemFree,
            "MemAvailable" => MeminfoType::MemAvailable,
            "Buffers" => MeminfoType::Buffers,
            "Cached" => MeminfoType::Cached,
            "SwapCached" => MeminfoType::SwapCached,
            "Active" => MeminfoType::Active,
            "Inactive" => MeminfoType::Inactive,
            "Active(anon)" => MeminfoType::ActiveAnon,
            "Inactive(anon)" => MeminfoType::InactiveAnon,
            "Active(file)" => MeminfoType::ActiveFile,
            "Inactive(file)" => MeminfoType::InactiveFile,
            "Unevictable" => MeminfoType::Unevictable,
            "Mlocked" => MeminfoType::Mlocked,
            "SwapTotal" => MeminfoType::SwapTotal,
            "SwapFree" => MeminfoType::SwapFree,
            "Zswap" => MeminfoType::Zswap,
            "Zswapped" => MeminfoType::Zswapped,
            "Dirty" => MeminfoType::Dirty,
            "Writeback" => MeminfoType::Writeback,
            "AnonPages" => MeminfoType::AnonPages,
            "Mapped" => MeminfoType::Mapped,
            "Shmem" => MeminfoType::Shmem,
            "KReclaimable" => MeminfoType::KReclaimable,
            "Slab" => MeminfoType::Slab,
            "SReclaimable" => MeminfoType::SReclaimable,
            "SUnreclaim" => MeminfoType::SUnreclaim,
            "KernelStack" => MeminfoType::KernelStack,
            "PageTables" => MeminfoType::PageTables,
            "SecPageTables" => MeminfoType::SecPageTables,
            "NFS_Unstable" => MeminfoType::NfsUnstable,
            "Bounce" => MeminfoType::Bounce,
            "WritebackTmp" => MeminfoType::WritebackTmp,
            "CommitLimit" => MeminfoType::CommitLimit,
            "Committed_AS" => MeminfoType::CommittedAs,
            "VmallocTotal" => MeminfoType::VmallocTotal,
            "VmallocUsed" => MeminfoType::VmallocUsed,
            "VmallocChunk" => MeminfoType::VmallocChunk,
            "Percpu" => MeminfoType::Percpu,
            "HardwareCorrupted" => MeminfoType::HardwareCorrupted,
            "AnonHugePages" => MeminfoType::AnonHugePages,
            "ShmemHugePages" => MeminfoType::ShmemHugePages,
            "ShmemPmdMapped" => MeminfoType::ShmemPmdMapped,
            "FileHugePages" => MeminfoType::FileHugePages,
            "FilePmdMapped" => MeminfoType::FilePmdMapped,
            "CmaTotal" => MeminfoType::CmaTotal,
            "CmaFree" => MeminfoType::CmaFree,
            "Unaccepted" => MeminfoType::Unaccepted,
            "HugePages_Total" => MeminfoType::HugePagesTotal,
            "HugePages_Free" => MeminfoType::HugePagesFree,
            "HugePages_Rsvd" => MeminfoType::HugePagesRsvd,
            "HugePages_Surp" => MeminfoType::HugePagesSurp,
            "Hugepagesize" => MeminfoType::HugepageSize,
            "Hugetlb" => MeminfoType::Hugetlb,
            "DirectMap4k" => MeminfoType::DirectMap4k,
            "DirectMap2M" => MeminfoType::DirectMap2M,
            "DirectMap1G" => MeminfoType::DirectMap1G,
            _ => MeminfoType::Unknown,
        }
    }
}

/// Meminfo represents memory statistics (bytes) retrieve from /proc/meminfo
#[derive(Debug, Serialize, Clone, Default)]
pub struct Meminfo {
    pub mem_total: Option<u64>,
    pub mem_free: Option<u64>,
    pub mem_available: Option<u64>,
    pub buffers: Option<u64>,
    pub cached: Option<u64>,
    pub swap_cached: Option<u64>,
    pub active: Option<u64>,
    pub inactive: Option<u64>,
    pub active_anon: Option<u64>,
    pub inactive_anon: Option<u64>,
    pub active_file: Option<u64>,
    pub inactive_file: Option<u64>,
    pub unevictable: Option<u64>,
    pub mlocked: Option<u64>,
    pub swap_total: Option<u64>,
    pub swap_free: Option<u64>,
    pub z_swap: Option<u64>,
    pub z_swapped: Option<u64>,
    pub dirty: Option<u64>,
    pub writeback: Option<u64>,
    pub annon_pages: Option<u64>,
    pub mapped: Option<u64>,
    pub shmem: Option<u64>,
    pub k_reclaimable: Option<u64>,
    pub slap: Option<u64>,
    pub s_reclaimable: Option<u64>,
    pub s_unreclaim: Option<u64>,
    pub kernel_stack: Option<u64>,
    pub page_tables: Option<u64>,
    pub sec_page_tables: Option<u64>,
    pub nfs_unstable: Option<u64>,
    pub bounce: Option<u64>,
    pub writeback_tmp: Option<u64>,
    pub commit_limit: Option<u64>,
    pub committed_as: Option<u64>,
    pub vmalloc_total: Option<u64>,
    pub vmalloc_used: Option<u64>,
    pub vmalloc_chunk: Option<u64>,
    pub per_cpu: Option<u64>,
    pub hardware_corrupted: Option<u64>,
    pub annon_huge_pages: Option<u64>,
    pub shmem_huge_pages: Option<u64>,
    pub shmem_pmd_mapped: Option<u64>,
    pub file_huge_pages: Option<u64>,
    pub file_pmd_mapped: Option<u64>,
    pub cma_total: Option<u64>,
    pub cma_free: Option<u64>,
    pub unaccepted: Option<u64>,
    pub huge_pages_total: Option<u64>,
    pub huge_pages_free: Option<u64>,
    pub huge_pages_rsvd: Option<u64>,
    pub huge_pages_surp: Option<u64>,
    pub huge_page_size: Option<u64>,
    pub huge_tlb: Option<u64>,
    pub direct_map_4k: Option<u64>,
    pub direct_map_2m: Option<u64>,
    pub direct_map_1g: Option<u64>,
}

impl Meminfo {
    fn new() -> Self {
        Default::default()
    }
}

/// collects the memory statistics in bytes
/// # Example
/// ```
/// use procsys::meminfo;
///
/// let sys_meminfo = meminfo::collect().expect("memory information");
/// let json_output = serde_json::to_string_pretty(&sys_meminfo).unwrap();
/// println!("{}", json_output);
///
/// ```
pub fn collect() -> CollectResult<Meminfo> {
    collect_from("/proc/meminfo")
}

fn collect_from(filename: &str) -> CollectResult<Meminfo> {
    let mut meminfo = Meminfo::new();

    for line in utils::read_file_lines(filename)? {
        let item_fields: Vec<&str> = line.trim().split(':').filter(|s| !s.is_empty()).collect();

        if item_fields.len() != 2 {
            return Err(MetricError::InvalidFieldNumberError(
                "meminfo".to_string(),
                item_fields.len(),
                line,
            ));
        }

        let value_fields: Vec<&str> = item_fields[1]
            .trim()
            .split(' ')
            .filter(|s| !s.is_empty())
            .collect();

        let item_value = value_fields[0].parse::<u64>().unwrap_or_default();
        let mut item_unit = "B";
        if value_fields.len() == 2 {
            item_unit = value_fields[1];
        }

        let metric_value = utils::convert_to_bytes(item_value, item_unit)?;

        match MeminfoType::from(item_fields[0]) {
            MeminfoType::MemTotal => meminfo.mem_total = metric_value,
            MeminfoType::MemFree => meminfo.mem_free = metric_value,
            MeminfoType::MemAvailable => meminfo.mem_available = metric_value,
            MeminfoType::Buffers => meminfo.buffers = metric_value,
            MeminfoType::Cached => meminfo.cached = metric_value,
            MeminfoType::SwapCached => meminfo.swap_cached = metric_value,
            MeminfoType::Active => meminfo.active = metric_value,
            MeminfoType::Inactive => meminfo.inactive = metric_value,
            MeminfoType::ActiveAnon => meminfo.active_anon = metric_value,
            MeminfoType::InactiveAnon => meminfo.inactive_anon = metric_value,
            MeminfoType::ActiveFile => meminfo.active_file = metric_value,
            MeminfoType::InactiveFile => meminfo.inactive_file = metric_value,
            MeminfoType::Unevictable => meminfo.unevictable = metric_value,
            MeminfoType::Mlocked => meminfo.mlocked = metric_value,
            MeminfoType::SwapTotal => meminfo.swap_total = metric_value,
            MeminfoType::SwapFree => meminfo.swap_free = metric_value,
            MeminfoType::Zswap => meminfo.z_swap = metric_value,
            MeminfoType::Zswapped => meminfo.z_swapped = metric_value,
            MeminfoType::Dirty => meminfo.dirty = metric_value,
            MeminfoType::Writeback => meminfo.writeback = metric_value,
            MeminfoType::AnonPages => meminfo.annon_pages = metric_value,
            MeminfoType::Mapped => meminfo.mapped = metric_value,
            MeminfoType::Shmem => meminfo.shmem = metric_value,
            MeminfoType::KReclaimable => meminfo.k_reclaimable = metric_value,
            MeminfoType::Slab => meminfo.slap = metric_value,
            MeminfoType::SReclaimable => meminfo.s_reclaimable = metric_value,
            MeminfoType::SUnreclaim => meminfo.s_unreclaim = metric_value,
            MeminfoType::KernelStack => meminfo.kernel_stack = metric_value,
            MeminfoType::PageTables => meminfo.page_tables = metric_value,
            MeminfoType::SecPageTables => meminfo.sec_page_tables = metric_value,
            MeminfoType::NfsUnstable => meminfo.nfs_unstable = metric_value,
            MeminfoType::Bounce => meminfo.bounce = metric_value,
            MeminfoType::WritebackTmp => meminfo.writeback_tmp = metric_value,
            MeminfoType::CommitLimit => meminfo.commit_limit = metric_value,
            MeminfoType::CommittedAs => meminfo.committed_as = metric_value,
            MeminfoType::VmallocTotal => meminfo.vmalloc_total = metric_value,
            MeminfoType::VmallocUsed => meminfo.vmalloc_used = metric_value,
            MeminfoType::VmallocChunk => meminfo.vmalloc_chunk = metric_value,
            MeminfoType::Percpu => meminfo.per_cpu = metric_value,
            MeminfoType::HardwareCorrupted => meminfo.hardware_corrupted = metric_value,
            MeminfoType::AnonHugePages => meminfo.annon_huge_pages = metric_value,
            MeminfoType::ShmemHugePages => meminfo.shmem_huge_pages = metric_value,
            MeminfoType::ShmemPmdMapped => meminfo.shmem_pmd_mapped = metric_value,
            MeminfoType::FileHugePages => meminfo.file_huge_pages = metric_value,
            MeminfoType::FilePmdMapped => meminfo.file_pmd_mapped = metric_value,
            MeminfoType::CmaTotal => meminfo.cma_total = metric_value,
            MeminfoType::CmaFree => meminfo.cma_free = metric_value,
            MeminfoType::Unaccepted => meminfo.unaccepted = metric_value,
            MeminfoType::HugePagesTotal => meminfo.huge_pages_total = metric_value,
            MeminfoType::HugePagesFree => meminfo.huge_pages_free = metric_value,
            MeminfoType::HugePagesRsvd => meminfo.huge_pages_rsvd = metric_value,
            MeminfoType::HugePagesSurp => meminfo.huge_pages_surp = metric_value,
            MeminfoType::HugepageSize => meminfo.huge_page_size = metric_value,
            MeminfoType::Hugetlb => meminfo.huge_tlb = metric_value,
            MeminfoType::DirectMap4k => meminfo.direct_map_4k = metric_value,
            MeminfoType::DirectMap2M => meminfo.direct_map_2m = metric_value,
            MeminfoType::DirectMap1G => meminfo.direct_map_1g = metric_value,
            MeminfoType::Unknown => {}
        }
    }

    Ok(meminfo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mem_stats() {
        let meminfo =
            collect_from("test_data/fixtures/proc/meminfo").expect("collecting memory information");

        assert_eq!(meminfo.mem_total.unwrap(), 16042172416);
        assert_eq!(meminfo.mem_free.unwrap(), 450891776);
        assert_eq!(meminfo.mem_available, None);
        assert_eq!(meminfo.buffers.unwrap(), 1044611072);
        assert_eq!(meminfo.cached.unwrap(), 12295823360);
        assert_eq!(meminfo.swap_cached.unwrap(), 0);
        assert_eq!(meminfo.active.unwrap(), 6923546624);
        assert_eq!(meminfo.inactive.unwrap(), 6689492992);
        assert_eq!(meminfo.active_anon.unwrap(), 273670144);
        assert_eq!(meminfo.inactive_anon.unwrap(), 274432);
        assert_eq!(meminfo.active_file.unwrap(), 6649876480);
        assert_eq!(meminfo.inactive_file.unwrap(), 6689218560);
        assert_eq!(meminfo.unevictable.unwrap(), 0);
        assert_eq!(meminfo.mlocked.unwrap(), 0);
        assert_eq!(meminfo.swap_total.unwrap(), 0);
        assert_eq!(meminfo.swap_free.unwrap(), 0);
        assert_eq!(meminfo.z_swap, None);
        assert_eq!(meminfo.z_swapped, None);
        assert_eq!(meminfo.dirty.unwrap(), 786432);
        assert_eq!(meminfo.writeback.unwrap(), 0);
        assert_eq!(meminfo.annon_pages.unwrap(), 272605184);
        assert_eq!(meminfo.mapped.unwrap(), 45264896);
        assert_eq!(meminfo.shmem.unwrap(), 1339392);
        assert_eq!(meminfo.k_reclaimable, None);
        assert_eq!(meminfo.slap.unwrap(), 1850638336);
        assert_eq!(meminfo.s_reclaimable.unwrap(), 1779838976);
        assert_eq!(meminfo.s_unreclaim.unwrap(), 70799360);
        assert_eq!(meminfo.kernel_stack.unwrap(), 1654784);
        assert_eq!(meminfo.page_tables.unwrap(), 5414912);
        assert_eq!(meminfo.sec_page_tables, None);
        assert_eq!(meminfo.nfs_unstable.unwrap(), 0);
        assert_eq!(meminfo.bounce.unwrap(), 0);
        assert_eq!(meminfo.writeback_tmp.unwrap(), 0);
        assert_eq!(meminfo.commit_limit.unwrap(), 8021086208);
        assert_eq!(meminfo.committed_as.unwrap(), 543584256);
        assert_eq!(meminfo.vmalloc_total.unwrap(), 34359738367 * 1024);
        assert_eq!(meminfo.vmalloc_used.unwrap(), 37474304);
        assert_eq!(meminfo.vmalloc_chunk.unwrap(), 34359637840 * 1024);
        assert_eq!(meminfo.per_cpu.unwrap(), 26804224);
        assert_eq!(meminfo.hardware_corrupted.unwrap(), 0);
        assert_eq!(meminfo.annon_huge_pages.unwrap(), 12582912);
        assert_eq!(meminfo.shmem_huge_pages, None);
        assert_eq!(meminfo.shmem_pmd_mapped, None);
        assert_eq!(meminfo.file_huge_pages, None);
        assert_eq!(meminfo.file_pmd_mapped, None);
        assert_eq!(meminfo.cma_total, None);
        assert_eq!(meminfo.cma_free, None);
        assert_eq!(meminfo.unaccepted, None);
        assert_eq!(meminfo.huge_pages_total.unwrap(), 0);
        assert_eq!(meminfo.huge_pages_free.unwrap(), 0);
        assert_eq!(meminfo.huge_pages_rsvd.unwrap(), 0);
        assert_eq!(meminfo.huge_pages_surp.unwrap(), 0);
        assert_eq!(meminfo.huge_page_size.unwrap(), 2097152);
        assert_eq!(meminfo.huge_tlb, None);
        assert_eq!(meminfo.direct_map_4k.unwrap(), 93323264);
        assert_eq!(meminfo.direct_map_2m.unwrap(), 16424894464);
        assert_eq!(meminfo.direct_map_1g, None);
    }
}
