# procsys [![][img_crates]][crates] [![][img_doc]][doc]

Rust library to retrieve system, kernel, and process metrics from the pseudo-filesystems /proc and /sys.

See the docs for more information about supported features, or view the [features.md](https://github.com/navidys/procsys/blob/main/FEATURES.md) file of the project repository.

## Examples

There are several examples in the documents and also in the [examples](https://github.com/navidys/procsys/tree/main/examples) directory of project repository.


```
use procsys::sysfs::class_watchdog;

let watchdog_devices = class_watchdog::collect();

for wdev in &watchdog_devices {
    println!("name: {}", wdev.name());
    println!("boot status: {}", wdev.boot_status().unwrap_or_default());
    println!("timeout: {}", wdev.timeout().unwrap_or_default());
    println!("min_timeout: {}", wdev.min_timeout().unwrap_or_default());
    println!("max_timeout: {}", wdev.max_timeout().unwrap_or_default());
}

// print all watchdog devices information in json output
match serde_json::to_string_pretty(&watchdog_devices) {
    Ok(output) => println!("{}", output),
    Err(err) => {
        log::error!("{}", err);
        std::process::exit(1);
    }
}
```

## License

Licensed under the [MIT License](https://github.com/navidys/procsys/blob/main/LICENSE).

[img_crates]: https://img.shields.io/crates/v/procsys.svg
[img_doc]: https://img.shields.io/badge/rust-documentation-blue.svg

[crates]: https://crates.io/crates/procsys
[doc]: https://docs.rs/procsys/
