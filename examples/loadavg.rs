use procsys::loadavg;

fn main() {
    let sysload = loadavg::collect().expect("system load average");

    println!("load average 1 : {}", sysload.load1);
    println!("load average 5 : {}", sysload.load5);
    println!("load average 15: {}", sysload.load15);
}
