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

// spell-checker:ignore (API) nodename osname sysname
// spell-checker:ignore (uutils) coreutils uutils

#![warn(unused_results)]

use std::borrow::Cow;
use std::ffi::OsString;

mod lib_impl;
pub use lib_impl::*;

/// `PlatformInfoAPI` defines a trait API providing `uname` (aka "Unix name") style platform information.
// ref: <https://www.gnu.org/software/libc/manual/html_node/Platform-Type.html> @@ <https://archive.is/YjjWJ>
pub trait PlatformInfoAPI {
    /// The name of this implementation of the operating system.
    fn sysname(&self) -> Result<Cow<str>, &OsString>;

    /// The node name (network node hostname) of this machine.
    fn nodename(&self) -> Result<Cow<str>, &OsString>;

    /// The current release level of the operating system.
    fn release(&self) -> Result<Cow<str>, &OsString>;

    /// The current version level of the current release.
    fn version(&self) -> Result<Cow<str>, &OsString>;

    /// The name of the current system's hardware.
    fn machine(&self) -> Result<Cow<str>, &OsString>;

    /// The name of the current OS.
    fn osname(&self) -> Result<Cow<str>, &OsString>;
}
