use procsys::net_wireless;

fn main() {
    let netwireless = net_wireless::collect().expect("network wireless information");

    // print all network devices information in json output
    match serde_json::to_string_pretty(&netwireless) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
