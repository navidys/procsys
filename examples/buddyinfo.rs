use procsys::buddyinfo;

fn main() {
    env_logger::init();

    let binfo = buddyinfo::collect();

    match serde_json::to_string_pretty(&binfo) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
