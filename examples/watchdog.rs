use procsys::sysfs::class_watchdog;

fn main() {
    env_logger::init();

    let watchdog_devices = class_watchdog::collect();

    for wdev in &watchdog_devices {
        println!("name: {}", wdev.name);
        println!("boot status: {}", wdev.boot_status.unwrap_or_default());
        println!("timeout: {}", wdev.timeout.unwrap_or_default());
        println!("min_timeout: {}", wdev.min_timeout.unwrap_or_default());
        println!("max_timeout: {}", wdev.max_timeout.unwrap_or_default());
    }

    // print all watchdog devices information in json output
    match serde_json::to_string_pretty(&watchdog_devices) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
