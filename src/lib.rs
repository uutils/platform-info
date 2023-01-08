// This file is part of the uutils coreutils package.
//
// (c) Alex Lyon <arcterus@mail.com>
//
// For the full copyright and license information, please view the LICENSE file
// that was distributed with this source code.
//

/*!
This crate provides the ability to retrieve various information specific to your current platform
without having to use platform-specific methods to so.  Currently, only information pertinent to
a utility like [`uname`](https://github.com/uutils/coreutils/blob/main/src/uu/uname/src/uname.rs) is
provided; however, in the future, more functionality may become available.

# Usage

This crate is available on [crate.io](https://crates.io/crates/platform-info), so using it in your
project is as simple as adding `platform-info` to your project's `Cargo.toml`, like so:

```toml
[dependencies]
platform-info = "1"
```

To see specific usage details, look at the `uname` utility linked above as it makes
use of every feature.
*/

pub use self::sys::*;

use std::borrow::Cow;

#[cfg(unix)]
#[path = "unix.rs"]
mod sys;
#[cfg(windows)]
#[path = "windows.rs"]
mod sys;
#[cfg(not(any(unix, windows)))]
#[path = "unknown.rs"]
mod sys;

/// `Uname` is meant for types that can provide information relevant to `uname`.
pub trait Uname {
    /// The name of this implementation of the operating system.
    fn sysname(&self) -> Cow<str>;

    /// The node name (network node hostname) of this machine.
    fn nodename(&self) -> Cow<str>;

    /// The current release level of the operating system.
    fn release(&self) -> Cow<str>;

    /// The current version level of the current release.
    fn version(&self) -> Cow<str>;

    /// The name of the current system's hardware.
    fn machine(&self) -> Cow<str>;

    /// The name of the current OS.
    fn osname(&self) -> Cow<str>;
}

// private platform-specific HOST_OS_NAME * ref: [`uname` info](https://en.wikipedia.org/wiki/Uname)
#[cfg(all(target_os = "linux", any(target_env = "gnu", target_env = "")))]
const HOST_OS_NAME: &str = "GNU/Linux";
#[cfg(all(target_os = "linux", not(any(target_env = "gnu", target_env = ""))))]
const HOST_OS_NAME: &str = "Linux";
#[cfg(target_os = "android")]
const HOST_OS_NAME: &str = "Android";
#[cfg(target_os = "windows")]
pub const HOST_OS_NAME: &str = "MS/Windows"; // prior art == `busybox`
#[cfg(target_os = "freebsd")]
const HOST_OS_NAME: &str = "FreeBSD";
#[cfg(target_os = "netbsd")]
const HOST_OS_NAME: &str = "NetBSD";
#[cfg(target_os = "openbsd")]
const HOST_OS_NAME: &str = "OpenBSD";
#[cfg(target_vendor = "apple")]
const HOST_OS_NAME: &str = "Darwin";
#[cfg(target_os = "fuchsia")]
const HOST_OS_NAME: &str = "Fuchsia";
#[cfg(target_os = "redox")]
const HOST_OS_NAME: &str = "Redox";
