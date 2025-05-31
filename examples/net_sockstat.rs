use procsys::net_sockstat;

fn main() {
    let net_sockstat_info = net_sockstat::collect().expect("network sockstat information");

    // print all network sockstat information in json output
    match serde_json::to_string_pretty(&net_sockstat_info) {
        Ok(output) => println!("sockstat:\n{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }

    let net_sockstat6_info = net_sockstat::collect6().expect("network sockstat6 information");

    // print all network sockstat information in json output
    match serde_json::to_string_pretty(&net_sockstat6_info) {
        Ok(output) => println!("sockstat6:\n{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
