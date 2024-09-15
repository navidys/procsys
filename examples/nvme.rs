use procsys::sysfs;

fn main() {
    let nvme_devices = sysfs::class_nvme::collect().expect("nvme devices information");

    // print all nvme devices information in json output
    match serde_json::to_string_pretty(&nvme_devices) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
