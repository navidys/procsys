use procsys::sysfs::class_sas_phy;

fn main() {
    let sasphys = class_sas_phy::collect().expect("sas phys information");

    match serde_json::to_string_pretty(&sasphys) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
