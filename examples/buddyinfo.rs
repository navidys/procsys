use procsys::buddyinfo;

fn main() {
    let binfo = buddyinfo::collect().expect("buddy information");

    match serde_json::to_string_pretty(&binfo) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
