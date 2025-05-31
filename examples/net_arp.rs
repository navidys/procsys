use procsys::net_arp;

fn main() {
    let net_arp_entries = net_arp::collect().expect("network arp entries information");

    // print all network arp entries information in json output
    match serde_json::to_string_pretty(&net_arp_entries) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
