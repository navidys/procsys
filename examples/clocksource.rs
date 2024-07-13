use procsys::sysfs;

fn main() {
    env_logger::init();

    let clocksources = sysfs::clocksource::collect();

    for clock_src in &clocksources {
        println!("name: {}", clock_src.name);
        println!(
            "available clocksource: {}",
            clock_src.available_clocksource.join(" "),
        );
        println!("current clocksource: {}", clock_src.current_clocksource);
    }

    // print all clocksources information in json output
    match serde_json::to_string_pretty(&clocksources) {
        Ok(output) => println!("{}", output),
        Err(err) => {
            log::error!("{}", err);
            std::process::exit(1);
        }
    }
}
