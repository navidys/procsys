use procsys::net_dev;

fn main() {
    env_logger::init();

    let net_devices = net_dev::collect();

    // print all network devices information in json output
    match serde_json::to_string_pretty(&net_devices) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
