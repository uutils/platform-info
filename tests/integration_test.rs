use platform_info::*;

#[test]
fn platform() -> Result<(), String> {
    let uname = match PlatformInfo::new() {
        Ok(info) => info,
        Err(error) => panic!("{}", error),
    };

    println!("sysname = {}", uname.sysname());
    println!("nodename = {}", uname.nodename());
    println!("release = {}", uname.release());
    println!("version = {}", uname.version());
    println!("machine = {}", uname.machine());

    assert!(!uname.sysname().is_empty());
    assert!(!uname.nodename().is_empty());
    assert!(!uname.release().is_empty());
    #[cfg(not(windows))] // empty on windows
    assert!(!uname.version().is_empty());
    assert!(!uname.machine().is_empty());
    Ok(())
}

#[cfg(windows)]
#[test]
fn platform_windows_specific() -> Result<(), String> {
    let uname = match PlatformInfo::new() {
        Ok(info) => info,
        Err(error) => panic!("{}", error),
    };
    println!("computer_name = {}", uname.nodename());
    println!("version = {}", uname.version());
    println!("release = {}", uname.release());

    assert!(!uname.nodename().is_empty());
    //    assert!(!uname.version().is_empty());
    assert!(!uname.release().is_empty());

    Ok(())
}
