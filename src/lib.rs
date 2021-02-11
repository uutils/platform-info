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
a utility like [`uname`](https://github.com/uutils/coreutils/blob/master/src/uname/uname.rs) is
provided; however, in the future, more functionality may become available.

# Usage

This crate is available on [crate.io](https://crates.io/crates/platform-info), so using it in your
project is as simple as adding `platform-info` to your project's `Cargo.toml`, like so:

```toml
[dependencies]
platform-info = "0.1"
```

To see specific usage details, I recommend looking at the `uname` utility linked above as it makes
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
}
