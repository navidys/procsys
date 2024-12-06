use procsys::sysfs::class_sas_port;

fn main() {
    let sasports = class_sas_port::collect().expect("sas ports information");

    match serde_json::to_string_pretty(&sasports) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
