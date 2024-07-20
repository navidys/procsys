use procsys::kernel_random;

fn main() {
    env_logger::init();

    let krandom = kernel_random::collect();

    match serde_json::to_string_pretty(&krandom) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
