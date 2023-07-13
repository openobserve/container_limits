extern crate sys_info;

fn main() {
    let cpu_num = sys_info::cpu_num().unwrap();
    println!("Number of logical CPUs is {}", cpu_num);

    let load_avg = sys_info::loadavg().unwrap();
    println!("Load average in the past 1, 5, and 15 minutes: {}, {}, {}", load_avg.one, load_avg.five, load_avg.fifteen);

    let mem_info = sys_info::mem_info().unwrap();
    println!("Total RAM: {} KB", mem_info.total);
    println!("Free RAM: {} KB", mem_info.free);
    println!("Used RAM: {} KB", mem_info.total - mem_info.avail);
}