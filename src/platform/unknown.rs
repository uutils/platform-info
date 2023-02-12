// This file is part of the uutils coreutils package.
//
// (c) Ingvar Stepanyan <me@rreverser.com>
//
// For the full copyright and license information, please view the LICENSE file
// that was distributed with this source code.

// spell-checker:ignore (API) nodename osname sysname
// spell-checker:ignore (names) Ingvar Stepanyan * me@rreverser.com
// spell-checker:ignore (uutils) coreutils uutils

use std::borrow::Cow;
use std::error::Error;
use std::ffi::OsString;

use crate::PlatformInfoAPI;

// PlatformInfo
/// Handles initial retrieval and holds information for the current platform ("unknown" in this case).
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct PlatformInfo(());

impl PlatformInfo {
    pub fn new() -> Result<Self, Box<dyn Error>> {
        Ok(Self(()))
    }
}

impl PlatformInfoAPI for PlatformInfo {
    fn sysname(&self) -> Result<Cow<str>, &OsString> {
        Ok(Cow::from("unknown"))
    }

    fn nodename(&self) -> Result<Cow<str>, &OsString> {
        Ok(Cow::from("unknown"))
    }

    fn release(&self) -> Result<Cow<str>, &OsString> {
        Ok(Cow::from("unknown"))
    }

    fn version(&self) -> Result<Cow<str>, &OsString> {
        Ok(Cow::from("unknown"))
    }

    fn machine(&self) -> Result<Cow<str>, &OsString> {
        Ok(Cow::from("unknown"))
    }

    fn osname(&self) -> Result<Cow<str>, &OsString> {
        Ok(Cow::from("unknown"))
    }
}

#[test]
fn test_unknown() {
    let platform_info = PlatformInfo::new().unwrap();

    assert_eq!(platform_info.sysname().unwrap(), "unknown");
    assert_eq!(platform_info.nodename().unwrap(), "unknown");
    assert_eq!(platform_info.release().unwrap(), "unknown");
    assert_eq!(platform_info.version().unwrap(), "unknown");
    assert_eq!(platform_info.machine().unwrap(), "unknown");
    assert_eq!(platform_info.osname().unwrap(), "unknown");
}
