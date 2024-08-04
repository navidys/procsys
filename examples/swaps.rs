use procsys::swaps;

fn main() {
    let sys_swapsinfo = swaps::collect().expect("swaps information");

    match serde_json::to_string_pretty(&sys_swapsinfo) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
