// examples/ex.rs
// * use `cargo run --example ex` to execute this example

// spell-checker:ignore (API) nodename osname sysname

use platform_info::*;

fn main() {
    let info = PlatformInfo::new().unwrap();
    // println!("info={:#?}", info);

    println!("{}", info.sysname().to_string_lossy());
    println!("{}", info.nodename().to_string_lossy());
    println!("{}", info.release().to_string_lossy());
    println!("{}", info.version().to_string_lossy());
    println!("{}", info.machine().to_string_lossy());
    println!("{}", info.osname().to_string_lossy());
}
