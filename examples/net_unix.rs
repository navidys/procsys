use procsys::net_unix;

fn main() {
    let netunix = net_unix::collect().expect("network unix information");

    // print all network unix information in json output
    match serde_json::to_string_pretty(&netunix) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
