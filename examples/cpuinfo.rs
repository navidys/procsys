use procsys::cpuinfo;

fn main() {
    env_logger::init();

    let sys_cpuinfo = cpuinfo::collect();

    match serde_json::to_string_pretty(&sys_cpuinfo) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
