use procsys::loadavg;

fn main() {
    env_logger::init();

    let sysload = loadavg::collect();

    println!("load average 1 : {}", sysload.load1);
    println!("load average 5 : {}", sysload.load5);
    println!("load average 15: {}", sysload.load15);
}
