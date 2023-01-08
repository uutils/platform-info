use platform_info::*;

#[test]
fn platform() -> Result<(), String> {
    let uname = match PlatformInfo::new() {
        Ok(info) => info,
        Err(error) => panic!("{}", error),
    };

    println!("sysname = '{}'", uname.sysname());
    println!("nodename = '{}'", uname.nodename());
    println!("release = '{}'", uname.release());
    println!("version = '{}'", uname.version());
    println!("machine = '{}'", uname.machine());
    println!("osname = '{}'", uname.osname());

    assert!(!uname.sysname().is_empty());
    assert!(!uname.nodename().is_empty());
    assert!(!uname.release().is_empty());
    assert!(!uname.version().is_empty());
    assert!(!uname.machine().is_empty());
    assert!(!uname.osname().is_empty());

    Ok(())
}
