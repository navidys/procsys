use procsys::sysfs;

fn main() {
    let infiniband_devices = sysfs::class_infiniband::collect().expect("infiniband information");

    // print all infiniband information in json output
    match serde_json::to_string_pretty(&infiniband_devices) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
