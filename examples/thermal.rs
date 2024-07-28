use procsys::sysfs::class_thermal;

fn main() {
    env_logger::init();

    let thermal_devices = class_thermal::collect().expect("thermal information");

    // print all thermal devices information in json output
    match serde_json::to_string_pretty(&thermal_devices) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
