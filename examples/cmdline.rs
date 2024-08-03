use procsys::cmdline;

fn main() {
    let sys_cmdline = cmdline::collect().expect("system boot cmdline");
    println!("{:?}", sys_cmdline);
}
