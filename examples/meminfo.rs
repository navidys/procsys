use procsys::meminfo;

fn main() {
    let sys_meminfo = meminfo::collect().expect("memory information");

    match serde_json::to_string_pretty(&sys_meminfo) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
