use procsys::sysfs;

fn main() {
    let fc_hosts = sysfs::class_fibrechannel::collect().expect("fibrechannel information");

    // print all fibrechannels information in json output
    match serde_json::to_string_pretty(&fc_hosts) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
