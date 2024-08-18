use procsys::process;

fn main() {
    let sys_proc = process::collect(2).expect("system proc 2");
    let sys_proc_limits = sys_proc.limits().expect("system proc 2 limits");

    match serde_json::to_string_pretty(&sys_proc_limits) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
