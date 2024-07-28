use procsys::cpuinfo;

fn main() {
    let sys_cpuinfo = cpuinfo::collect().expect("cpu information");

    match serde_json::to_string_pretty(&sys_cpuinfo) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
