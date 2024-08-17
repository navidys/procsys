use procsys::process;

fn main() {
    let sys_proc = process::collect(2).expect("system proc 2");
    let sys_proc_io = sys_proc.io().expect("system proc 2 io");

    match serde_json::to_string_pretty(&sys_proc_io) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
