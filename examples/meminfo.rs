use procsys::meminfo;

fn main() {
    env_logger::init();

    let sys_meminfo = meminfo::collect();

    match serde_json::to_string_pretty(&sys_meminfo) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
