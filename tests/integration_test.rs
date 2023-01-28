// spell-checker:ignore (API) nodename osname sysname

use regex;

use platform_info::*;

#[test]
fn platform() -> Result<(), String> {
    let uname = PlatformInfo::new().unwrap();

    let sysname = uname.sysname();
    let nodename = uname.nodename();
    let release = uname.release();
    let version = uname.version();
    let machine = uname.machine();
    let osname = uname.osname();

    println!("sysname = [{}]'{}'", sysname.len(), sysname);
    println!("nodename = [{}]'{}'", nodename.len(), nodename);
    println!("release = [{}]'{}'", release.len(), release);
    println!("version = [{}]'{}'", version.len(), version);
    println!("machine = [{}]'{}'", machine.len(), machine);
    println!("osname = [{}]'{}'", osname.len(), osname);

    assert!(!uname.sysname().is_empty());
    assert!(!uname.nodename().is_empty());
    assert!(!uname.release().is_empty());
    assert!(!uname.version().is_empty());
    assert!(!uname.machine().is_empty());
    assert!(!uname.osname().is_empty());

    Ok(())
}

#[test]
fn platform_no_invisible_contents() -> Result<(), String> {
    let uname = PlatformInfo::new().unwrap();

    let sysname = uname.sysname();
    let nodename = uname.nodename();
    let release = uname.release();
    let version = uname.version();
    let machine = uname.machine();
    let osname = uname.osname();

    let s = format!("sysname='{sysname}';nodename='{nodename}';release='{release}';version='{version}';machine='{machine}';osname={osname}");
    println!("s = [{}]\"{}\"", s.len(), s);

    // let re = regex::Regex::new("[^[[:print:]]]").unwrap(); // matches invisible (and emojis)
    let re = regex::Regex::new("[^[[:print:]]\\p{Other_Symbol}]").unwrap(); // matches invisible only (not emojis)
    assert_eq!(re.find(&s), None);

    Ok(())
}
