use procsys::net_dev;

fn main() {
    let net_devices = net_dev::collect().expect("network devices information");

    // print all network devices information in json output
    match serde_json::to_string_pretty(&net_devices) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
