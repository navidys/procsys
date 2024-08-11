use procsys::proc;

fn main() {
    let procs = proc::collect_all().expect("system processes");

    for proc in procs {
        println!("pid: {}", proc.pid());
        println!("\t comm: {}", proc.comm().unwrap_or_default());
        println!("\t wchan: {}", proc.wchan().unwrap_or_default());
        println!("\t executable: {:?}", proc.executable().unwrap_or_default());
    }

    let proc = proc::collect(1).expect("process exist");
    println!("pid: {}", proc.pid());
    println!("\t comm: {}", proc.comm().unwrap_or_default());
    println!("\t wchan: {}", proc.wchan().unwrap_or_default());
    println!("\t executable: {:?}", proc.executable().unwrap_or_default());
}
