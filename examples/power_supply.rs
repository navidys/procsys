use procsys::sysfs;

fn main() {
    let power_supplies = sysfs::class_power_supply::collect().expect("power supplies information");

    // print all power supplies information in json output
    match serde_json::to_string_pretty(&power_supplies) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
