/*
 * 
 */
use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

pub struct MemoryRegion {
    start: u64,
    end: u64,
    permissions: String,
    offset: u64,
    device: String,
    inode: u64,
    pathname: String,
}

pub fn read_process_memory_maps(pid: u32) -> Vec<MemoryRegion>
{
    let maps_path = format!("/proc/{}/maps", pid);
    let file = File::open(&Path::new(&maps_path)).expect("Failed to open maps file");

    let reader = BufReader::new(file);
    let mut regions = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let mut parts = line.split_whitespace();    // may be wrong here
            let range = parts.next().expect("Invalid maps entry");
            let permissions = parts.next().expect("Invalid maps entry");
            let offset = parts.next().expect("Invalid maps entry");
            let device = parts.next().expect("Invalid maps entry");
            let inode = parts.next().expect("Invalid maps entry");
            let pathname = parts.next().unwrap_or("");

            let range_parts: Vec<&str> = range.split('-').collect();
            let start = u64::from_str_radix(range_parts[0], 16).expect("Invalid memory range");
            let end = u64::from_str_radix(range_parts[1], 16).expect("Invalid memory range");
            let offset = u64::from_str_radix(offset, 16).expect("Invalid offset");
            let inode = u64::from_str_radix(inode, 10).expect("Invalid inode");

            let region = MemoryRegion {
                start,
                end,
                permissions: permissions.to_string(),
                offset,
                device: device.to_string(),
                inode,
                pathname: pathname.to_string(),
            };

            regions.push(region);
        }
    }

    regions
}

pub fn print_memory_regions(regions: &[MemoryRegion]) {
    for region in regions {
        println!(
            "{:016X}-{:016X} {} {:08X} {} {} \t\t{}",
            region.start, region.end, region.permissions, region.offset, region.device, region.inode, region.pathname
        );
    }
}
