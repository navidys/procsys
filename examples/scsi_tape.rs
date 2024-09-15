use procsys::sysfs;

fn main() {
    let scsi_tapes = sysfs::class_scsi_tape::collect().expect("scsi tape statistics information");

    // print all scsi tapes statistics in json output
    match serde_json::to_string_pretty(&scsi_tapes) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
