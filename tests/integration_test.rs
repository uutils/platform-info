// spell-checker:ignore (API) nodename osname sysname

use regex;

use platform_info::*;

#[test]
fn platform() -> Result<(), String> {
    let uname = match PlatformInfo::new() {
        Ok(info) => info,
        Err(error) => panic!("{}", error),
    };

    let sysname = (uname.sysname()).unwrap_or_else(|os_s| os_s.to_string_lossy());
    let nodename = (uname.nodename()).unwrap_or_else(|os_s| os_s.to_string_lossy());
    let release = (uname.release()).unwrap_or_else(|os_s| os_s.to_string_lossy());
    let version = (uname.version()).unwrap_or_else(|os_s| os_s.to_string_lossy());
    let machine = (uname.machine()).unwrap_or_else(|os_s| os_s.to_string_lossy());
    let osname = (uname.osname()).unwrap_or_else(|os_s| os_s.to_string_lossy());

    println!("sysname = [{}]'{}'", sysname.len(), sysname);
    println!("nodename = [{}]'{}'", nodename.len(), nodename);
    println!("release = [{}]'{}'", release.len(), release);
    println!("version = [{}]'{}'", version.len(), version);
    println!("machine = [{}]'{}'", machine.len(), machine);
    println!("osname = [{}]'{}'", osname.len(), osname);

    assert!(!sysname.is_empty());
    assert!(!nodename.is_empty());
    assert!(!release.is_empty());
    assert!(!version.is_empty());
    assert!(!machine.is_empty());
    assert!(!osname.is_empty());

    // assert!(false);
    Ok(())
}

#[test]
fn platform_no_invisible_contents() -> Result<(), String> {
    let uname = match PlatformInfo::new() {
        Ok(info) => info,
        Err(error) => panic!("{}", error),
    };

    let sysname = (uname.sysname()).unwrap_or_else(|os_str| os_str.to_string_lossy());
    let nodename = (uname.nodename()).unwrap_or_else(|os_str| os_str.to_string_lossy());
    let release = (uname.release()).unwrap_or_else(|os_str| os_str.to_string_lossy());
    let version = (uname.version()).unwrap_or_else(|os_str| os_str.to_string_lossy());
    let machine = (uname.machine()).unwrap_or_else(|os_str| os_str.to_string_lossy());
    let osname = (uname.osname()).unwrap_or_else(|os_str| os_str.to_string_lossy());

    let s = format!("sysname='{sysname}';nodename='{nodename}';release='{release}';version='{version}';machine='{machine}';osname='{osname}'");
    println!("s = [{}]\"{}\"", s.len(), s);

    // let re = regex::Regex::new("[^[[:print:]]]").unwrap(); // matches invisible (and emojis)
    let re = regex::Regex::new("[^[[:print:]]\\p{Other_Symbol}]").unwrap(); // matches invisible only (not emojis)
    assert_eq!(re.find(&s), None);

    Ok(())
}
