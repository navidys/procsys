use procsys::sysfs;

fn main() {
    env_logger::init();

    let cooling_devices = sysfs::class_cooling::collect();

    // print all cooling devices information in json output
    match serde_json::to_string_pretty(&cooling_devices) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
