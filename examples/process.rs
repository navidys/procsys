use procsys::process;

fn main() {
    let procs = process::collect_all().expect("system processes");

    for proc in procs {
        println!("pid: {}", proc.pid());
        println!("\t comm: {}", proc.comm().unwrap_or_default());
        println!("\t wchan: {}", proc.wchan().unwrap_or_default());
        println!("\t executable: {:?}", proc.executable().unwrap_or_default());
        println!("\t cwd: {:?}", proc.cwd().unwrap_or_default());
        println!("\t root: {:?}", proc.root_dir().unwrap_or_default());
    }
}
