// This file is part of the uutils coreutils package.
//
// (c) Alex Lyon <arcterus@mail.com>
//
// For the full copyright and license information, please view the LICENSE file
// that was distributed with this source code.
//

pub use self::sys::*;

use std::borrow::Cow;

#[cfg(unix)]
#[path = "unix.rs"]
mod sys;
#[cfg(windows)]
#[path = "windows.rs"]
mod sys;
#[cfg(target_os = "redox")]
#[path = "redox.rs"]
mod sys;

pub trait Uname {
    fn sysname(&self) -> Cow<str>;
    fn nodename(&self) -> Cow<str>;
    fn release(&self) -> Cow<str>;
    fn version(&self) -> Cow<str>;
    fn machine(&self) -> Cow<str>;
}
