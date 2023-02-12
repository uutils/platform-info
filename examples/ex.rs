// examples/ex.rs
// * use `cargo run --example ex` to execute this example

// spell-checker:ignore (API) nodename osname sysname

use platform_info::*;

fn main() {
    let info = PlatformInfo::new().unwrap();
    // println!("info={:#?}", info);

    println!(
        "{}",
        (info.sysname()).unwrap_or_else(|os_s| os_s.to_string_lossy())
    );
    println!(
        "{}",
        (info.nodename()).unwrap_or_else(|os_s| os_s.to_string_lossy())
    );
    println!(
        "{}",
        (info.release()).unwrap_or_else(|os_s| os_s.to_string_lossy())
    );
    println!(
        "{}",
        (info.version()).unwrap_or_else(|os_s| os_s.to_string_lossy())
    );
    println!(
        "{}",
        (info.machine()).unwrap_or_else(|os_s| os_s.to_string_lossy())
    );
    println!(
        "{}",
        (info.osname()).unwrap_or_else(|os_s| os_s.to_string_lossy())
    );
}
