extern crate sys_info;
use std::fs;
use std::io::ErrorKind;

fn main() {
    println!(" ------ Details from the host ------ ");

    let cpu_num = sys_info::cpu_num().unwrap();
    println!("Number of logical CPUs is {}", cpu_num);

    let load_avg = sys_info::loadavg().unwrap();
    println!(
        "Load average in the past 1, 5, and 15 minutes: {}, {}, {}",
        load_avg.one, load_avg.five, load_avg.fifteen
    );

    let mem_info = sys_info::mem_info().unwrap();
    println!("Total RAM: {} KB", mem_info.total);
    println!("Free RAM: {} KB", mem_info.free);
    println!("Used RAM: {} KB", mem_info.total - mem_info.avail);

    println!(" ------ Details from inside the container ------ ");

    match fs::read_to_string("/sys/fs/cgroup/memory/memory.limit_in_bytes") {
        Ok(memory_limit) => println!("Memory Limit: {}", memory_limit),
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            println!("Memory limit file not found, possibly not running in a container")
        }
        Err(error) => panic!("Unexpected error: {}", error),
    }

    match fs::read_to_string("/sys/fs/cgroup/cpu/cpu.cfs_quota_us") {
        Ok(cfs_quota_us) => println!("CPU Shares: {}", cfs_quota_us),
        Err(ref error) if error.kind() == ErrorKind::NotFound => {
            println!("CPU quota file not found, possibly not running in a container")
        }
        Err(error) => panic!("Unexpected error: {}", error),
    }
}
