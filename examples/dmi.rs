use procsys::sysfs;

fn main() {
    env_logger::init();

    let dmi_info = sysfs::class_dmi::collect();

    println!(
        "bios date: {}",
        dmi_info.bios_date().to_owned().unwrap_or_default(),
    );

    println!(
        "bios release: {}",
        dmi_info.bios_release().to_owned().unwrap_or_default(),
    );

    // print all DMI information in json output
    match serde_json::to_string_pretty(&dmi_info) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
