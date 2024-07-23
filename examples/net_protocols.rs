use procsys::net_protocols;

fn main() {
    env_logger::init();

    let netprotocols = net_protocols::collect();

    match serde_json::to_string_pretty(&netprotocols) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
