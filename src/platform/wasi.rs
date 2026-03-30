// This file is part of the platform-info package.
//
// For the full copyright and license information, please view the LICENSE file
// that was distributed with this source code.

// spell-checker:ignore (API) nodename osname sysname gethostname
// spell-checker:ignore (runtimes) wasmtime wasmer wazero

#![warn(unused_results)] // enable warnings for unused results

use std::ffi::{OsStr, OsString};

use crate::{PlatformInfoAPI, PlatformInfoError, UNameAPI};

/// Platform information for WASI (WebAssembly System Interface).
///
/// WASI does not provide a `uname()` syscall, so we return values
/// that describe the WebAssembly platform.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PlatformInfo {
    sysname: OsString,
    nodename: OsString,
    release: OsString,
    version: OsString,
    machine: OsString,
    processor: OsString,
    osname: OsString,
}

/// Try to detect the hostname from environment variables.
///
/// WASI (neither preview 1 nor preview 2) provides a `gethostname` syscall or
/// equivalent WIT interface. The `wasi-sockets` proposal lists hostname as a
/// future goal. Until then, the best we can do is check environment variables
/// that the host runtime may pass (e.g. `wasmtime --env HOSTNAME=$(hostname)`).
fn detect_nodename() -> OsString {
    for var in &["HOSTNAME", "COMPUTERNAME"] {
        if let Ok(val) = std::env::var(var) {
            if !val.is_empty() {
                return OsString::from(val);
            }
        }
    }
    OsString::from("localhost")
}

impl PlatformInfoAPI for PlatformInfo {
    /// Create a new `PlatformInfo` for WASI.
    ///
    /// WASI does not provide a `uname()` syscall, and no WASI runtime (wasmtime,
    /// wasmer, WasmEdge, wazero) exposes its identity or version to guest modules.
    /// There is no standard or convention for runtime self-identification.
    ///
    /// We follow the same approach as wasi-libc's `uname()` implementation, which
    /// returns hardcoded values: `sysname="wasi"`, `release="0.0.0"`,
    /// `version="0.0.0"`, `machine="wasm32"`.
    ///
    /// ref: <https://github.com/WebAssembly/wasi-libc/blob/main/libc-top-half/musl/src/misc/uname.c>
    fn new() -> Result<Self, PlatformInfoError> {
        Ok(Self {
            sysname: OsString::from("wasi"),
            nodename: detect_nodename(),
            release: OsString::from("0.0.0"),
            version: OsString::from("0.0.0"),
            machine: OsString::from(std::env::consts::ARCH),
            processor: OsString::from(crate::lib_impl::map_processor(std::env::consts::ARCH)),
            osname: OsString::from(crate::lib_impl::HOST_OS_NAME),
        })
    }
}

impl UNameAPI for PlatformInfo {
    fn sysname(&self) -> &OsStr {
        &self.sysname
    }

    fn nodename(&self) -> &OsStr {
        &self.nodename
    }

    fn release(&self) -> &OsStr {
        &self.release
    }

    fn version(&self) -> &OsStr {
        &self.version
    }

    fn machine(&self) -> &OsStr {
        &self.machine
    }

    fn processor(&self) -> &OsStr {
        &self.processor
    }

    fn osname(&self) -> &OsStr {
        &self.osname
    }
}

#[test]
fn test_wasi() {
    let info = PlatformInfo::new().unwrap();

    assert_eq!(info.sysname().to_string_lossy(), "wasi");
    // nodename comes from env or defaults to "localhost"
    assert!(!info.nodename().to_string_lossy().is_empty());
    assert_eq!(info.release().to_string_lossy(), "0.0.0");
    assert_eq!(info.version().to_string_lossy(), "0.0.0");
    assert_eq!(info.machine().to_string_lossy(), std::env::consts::ARCH);
}

#[test]
fn test_wasi_nodename_env() {
    let orig = std::env::var("HOSTNAME").ok();

    std::env::set_var("HOSTNAME", "my-wasi-host");
    let info = PlatformInfo::new().unwrap();
    assert_eq!(info.nodename().to_string_lossy(), "my-wasi-host");

    std::env::remove_var("HOSTNAME");
    std::env::remove_var("COMPUTERNAME");
    let info = PlatformInfo::new().unwrap();
    assert_eq!(info.nodename().to_string_lossy(), "localhost");

    // Restore
    match orig {
        Some(v) => std::env::set_var("HOSTNAME", v),
        None => std::env::remove_var("HOSTNAME"),
    }
}

#[test]
fn structure_clone() {
    let info = PlatformInfo::new().unwrap();
    println!("{:?}", info);
    let info_copy = info.clone();
    assert_eq!(info_copy, info);
}
