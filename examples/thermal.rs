use procsys::sysfs::class_thermal;

fn main() {
    env_logger::init();

    let thermal_devices = class_thermal::collect();

    for tdev in &thermal_devices {
        println!("name: {}", tdev.name);
        println!("temperature: {}", tdev.temp);
        println!("type: {}", tdev.zone_type);
        println!("policy: {}", tdev.zone_type);
    }

    // print all thermal devices information in json output
    match serde_json::to_string_pretty(&thermal_devices) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
