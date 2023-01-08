// `cargo run --example ex` (executes this example)

use platform_info::*;

fn main() {
    let uname = PlatformInfo::new().unwrap();
    println!("{}", uname.sysname());
    println!("{}", uname.nodename());
    println!("{}", uname.release());
    println!("{}", uname.version());
    println!("{}", uname.machine());
    println!("{}", uname.osname());
}
