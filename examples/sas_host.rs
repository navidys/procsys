use procsys::sysfs::class_sas_host;

fn main() {
    let sashosts = class_sas_host::collect().expect("sas hosts information");

    match serde_json::to_string_pretty(&sashosts) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
