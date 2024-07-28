use procsys::net_protocols;

fn main() {
    let netprotocols = net_protocols::collect().expect("network protocols");

    match serde_json::to_string_pretty(&netprotocols) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
