use procsys::sysfs;

fn main() {
    let cooling_devices = sysfs::class_cooling::collect().expect("cooling information");

    // print all cooling devices information in json output
    match serde_json::to_string_pretty(&cooling_devices) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
