use procsys::kernel_random;

fn main() {
    let krandom = kernel_random::collect().expect("random generator information");

    match serde_json::to_string_pretty(&krandom) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
