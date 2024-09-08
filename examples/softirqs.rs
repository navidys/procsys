use procsys::softirqs;

fn main() {
    let sys_softirqs = softirqs::collect().expect("softirqs information");

    match serde_json::to_string_pretty(&sys_softirqs) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
