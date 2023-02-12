// spell-checker:ignore (API) nodename osname sysname

use regex;

use platform_info::*;

#[test]
fn platform() -> Result<(), String> {
    let uname = match PlatformInfo::new() {
        Ok(info) => info,
        Err(error) => panic!("{}", error),
    };

    let sysname = match uname.sysname().to_os_string().into_string() {
        Ok(s) => {
            println!("sysname = [{}]'{}'", s.len(), s);
            s
        }
        Err(os_s) => {
            let s = os_s.to_string_lossy();
            println!("sysname = [{}]'{:?}' => '{}'", os_s.len(), os_s, s);
            String::from(s)
        }
    };
    let nodename = match uname.nodename().to_os_string().into_string() {
        Ok(s) => {
            println!("nodename = [{}]'{}'", s.len(), s);
            s
        }
        Err(os_s) => {
            let s = os_s.to_string_lossy();
            println!("nodename = [{}]'{:?}' => '{}'", os_s.len(), os_s, s);
            String::from(s)
        }
    };
    let release = match uname.release().to_os_string().into_string() {
        Ok(s) => {
            println!("release = [{}]'{}'", s.len(), s);
            s
        }
        Err(os_s) => {
            let s = os_s.to_string_lossy();
            println!("release = [{}]'{:?}' => '{}'", os_s.len(), os_s, s);
            String::from(s)
        }
    };
    let version = match uname.version().to_os_string().into_string() {
        Ok(s) => {
            println!("version = [{}]'{}'", s.len(), s);
            s
        }
        Err(os_s) => {
            let s = os_s.to_string_lossy();
            println!("version = [{}]'{:?}' => '{}'", os_s.len(), os_s, s);
            String::from(s)
        }
    };
    let machine = match uname.machine().to_os_string().into_string() {
        Ok(s) => {
            println!("machine = [{}]'{}'", s.len(), s);
            s
        }
        Err(os_s) => {
            let s = os_s.to_string_lossy();
            println!("machine = [{}]'{:?}' => '{}'", os_s.len(), os_s, s);
            String::from(s)
        }
    };
    let osname = match uname.osname().to_os_string().into_string() {
        Ok(s) => {
            println!("osname = [{}]'{}'", s.len(), s);
            s
        }
        Err(os_s) => {
            let s = os_s.to_string_lossy();
            println!("osname = [{}]'{:?}' => '{}'", os_s.len(), os_s, s);
            String::from(s)
        }
    };

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

    let sysname = uname.sysname().to_string_lossy();
    let nodename = uname.nodename().to_string_lossy();
    let release = uname.release().to_string_lossy();
    let version = uname.version().to_string_lossy();
    let machine = uname.machine().to_string_lossy();
    let osname = uname.osname().to_string_lossy();

    let s = format!("sysname='{sysname}';nodename='{nodename}';release='{release}';version='{version}';machine='{machine}';osname='{osname}'");
    println!("s = [{}]\"{}\"", s.len(), s);

    // let re = regex::Regex::new("[^[[:print:]]]").unwrap(); // matches invisible (and emojis)
    let re = regex::Regex::new("[^[[:print:]]\\p{Other_Symbol}]").unwrap(); // matches invisible only (not emojis)
    assert_eq!(re.find(&s), None);

    Ok(())
}
