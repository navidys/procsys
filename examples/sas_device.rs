use procsys::sysfs::class_sas_device;

fn main() {
    let sasdevices = class_sas_device::collect().expect("sas devices information");

    match serde_json::to_string_pretty(&sasdevices) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
