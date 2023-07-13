extern crate sys_info;
use std::fs;
use std::io::ErrorKind;

fn main() {
     println!("\n ------CPU and memory details from host \n");

    let cpu_num = sys_info::cpu_num().unwrap();
    println!("Number of logical CPUs is {}", cpu_num);

    let mem_info = sys_info::mem_info().unwrap();
    println!("Total RAM: {} KB", mem_info.total);
    println!("Free RAM: {} KB", mem_info.free);
    println!("Used RAM: {} KB", mem_info.total - mem_info.avail);

    println!("\n ------ CPU and memory details from container using control groups v2 \n");

    match fs::read_to_string("/sys/fs/cgroup/cpu.weight") {
        Ok(cpu_weight) => println!("CPU Weight: {}", cpu_weight),
        Err(ref error) if error.kind() == ErrorKind::NotFound => println!("CPU weight file not found, possibly not running in a container"),
        Err(error) => panic!("Unexpected error: {}", error),
    }

    match fs::read_to_string("/sys/fs/cgroup/cpu.max") {
        Ok(cpu_max) => println!("CPU Max: {}", cpu_max),
        Err(ref error) if error.kind() == ErrorKind::NotFound => println!("CPU max file not found, possibly not running in a container"),
        Err(error) => panic!("Unexpected error: {}", error),
    }

    match fs::read_to_string("/sys/fs/cgroup/memory.max") {
        Ok(memory_max) => println!("Memory Max: {}", memory_max),
        Err(ref error) if error.kind() == ErrorKind::NotFound => println!("Memory max file not found, possibly not running in a container"),
        Err(error) => panic!("Unexpected error: {}", error),
    }

    match fs::read_to_string("/sys/fs/cgroup/memory.current") {
        Ok(memory_current) => println!("Current Memory Usage: {}", memory_current),
        Err(ref error) if error.kind() == ErrorKind::NotFound => println!("Current memory usage file not found, possibly not running in a container"),
        Err(error) => panic!("Unexpected error: {}", error),
    }
}
