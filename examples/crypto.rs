use procsys::crypto;

fn main() {
    let crypto_info = crypto::collect().expect("crypto information");

    match serde_json::to_string_pretty(&crypto_info) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
