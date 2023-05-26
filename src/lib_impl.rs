// "plumbing" setup and connections for `lib.rs`

#![warn(unused_results)]

#[cfg(target_os = "windows")]
use std::path::Path;
#[cfg(target_os = "windows")]
use std::path::PathBuf;

//=== types

pub type ThreadSafeStdError = dyn std::error::Error + Send + Sync;
pub type BoxedThreadSafeStdError = Box<ThreadSafeStdError>;

/// A slice of a path string
/// (akin to [`str`]; aka/equivalent to [`Path`]).
#[cfg(target_os = "windows")]
type PathStr = Path;
/// An owned, mutable path string
/// (akin to [`String`]; aka/equivalent to [`PathBuf`]).
#[cfg(target_os = "windows")]
type PathString = PathBuf;

//=== platform-specific const

// HOST_OS_NAME * ref: [`uname` info](https://en.wikipedia.org/wiki/Uname)
#[cfg(all(target_os = "linux", any(target_env = "gnu", target_env = "")))]
const HOST_OS_NAME: &str = "GNU/Linux";
#[cfg(all(target_os = "linux", not(any(target_env = "gnu", target_env = ""))))]
pub const HOST_OS_NAME: &str = "Linux";
#[cfg(target_os = "android")]
pub const HOST_OS_NAME: &str = "Android";
#[cfg(target_os = "windows")]
pub const HOST_OS_NAME: &str = "MS/Windows"; // prior art == `busybox`
#[cfg(target_os = "freebsd")]
pub const HOST_OS_NAME: &str = "FreeBSD";
#[cfg(target_os = "netbsd")]
pub const HOST_OS_NAME: &str = "NetBSD";
#[cfg(target_os = "openbsd")]
pub const HOST_OS_NAME: &str = "OpenBSD";
#[cfg(target_vendor = "apple")]
pub const HOST_OS_NAME: &str = "Darwin";
#[cfg(target_os = "fuchsia")]
pub const HOST_OS_NAME: &str = "Fuchsia";
#[cfg(target_os = "redox")]
pub const HOST_OS_NAME: &str = "Redox";
#[cfg(not(any(unix, windows)))]
pub const HOST_OS_NAME: &str = "unknown";

//=== platform-specific module code

#[cfg(unix)]
#[path = "platform/unix.rs"]
mod target;
#[cfg(windows)]
#[path = "platform/windows.rs"]
mod target;
#[cfg(not(any(unix, windows)))]
#[path = "platform/unknown.rs"]
mod target;

pub use target::*;
