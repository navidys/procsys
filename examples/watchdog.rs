use procsys::sysfs::class_watchdog;

fn main() {
    env_logger::init();

    let watchdog_devices = class_watchdog::collect().expect("watchdog information");

    // print all watchdog devices information in json output
    match serde_json::to_string_pretty(&watchdog_devices) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
