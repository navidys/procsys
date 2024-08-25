use procsys::process;

fn main() {
    let sys_proc = process::collect(2).expect("system proc 2");
    let sys_proc_netsnmp6 = sys_proc.net_snmp6().expect("system proc 2 net snmp6");

    match serde_json::to_string_pretty(&sys_proc_netsnmp6) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
