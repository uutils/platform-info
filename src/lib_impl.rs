// "plumbing" setup and connections for `lib.rs`

// spell-checker:ignore (jargon) armv

#![warn(unused_results)] // enable warnings for unused results

#[cfg(target_os = "windows")]
use std::path::Path;
#[cfg(target_os = "windows")]
use std::path::PathBuf;

//=== types

/// Standard thread-safe error type
pub type ThreadSafeStdError = dyn std::error::Error + Send + Sync;
/// Standard thread-safe error type (boxed to allow translation for any `std::error::Error` type)
pub type BoxedThreadSafeStdError = Box<ThreadSafeStdError>;

/// A slice of a path string
/// (akin to [`str`]; aka/equivalent to [`Path`]).
#[cfg(target_os = "windows")]
type PathStr = Path;
/// An owned, mutable path string
/// (akin to [`String`]; aka/equivalent to [`PathBuf`]).
#[cfg(target_os = "windows")]
type PathString = PathBuf;

//=== platform-specific functions

// map_processor
/// *Returns* processor type mapped from machine architecture string.
///
/// Provides GNU coreutils-compatible processor type mappings from machine architecture strings.
/// This function normalizes architecture names to match the output of GNU coreutils' `uname -p`.
///
/// # Difference from `machine()`
///
/// While [`UNameAPI::machine()`] returns the raw architecture string from the OS (e.g., "arm64" on macOS),
/// `processor()` provides a normalized, cross-platform compatible representation:
///
/// | Platform | machine() | processor() | Reason |
/// |----------|-----------|-------------|--------|
/// | macOS ARM | "arm64" | "arm" | GNU coreutils compatibility |
/// | Linux ARM64 | "aarch64" | "aarch64" | Preserve Linux convention |
/// | Windows x86 | "i386"-"i686" | "i686" | Normalize to common name |
///
/// # Architecture Mappings
///
/// * macOS uses "arm64" for ARM-based Macs → maps to "arm"
/// * Linux uses "aarch64" for ARM64 → passes through as "aarch64"
/// * Various i386/i486/i586/i686 variants → normalized to "i686"
/// * ARMv6/v7/v8 variants → mapped to "arm"
/// * Unknown architectures pass through unchanged (better than returning "unknown")
///
/// ref: <https://github.com/uutils/coreutils/issues/8659>
pub(crate) fn map_processor(machine: &str) -> String {
    match machine {
        "arm64" => "arm".to_string(),
        "aarch64" => "aarch64".to_string(),
        "x86_64" | "amd64" => "x86_64".to_string(),
        "i386" | "i486" | "i586" | "i686" => "i686".to_string(),
        "armv7l" | "armv6l" | "armv8l" => "arm".to_string(),
        _ => machine.to_string(),
    }
}

//=== platform-specific const

// HOST_OS_NAME * ref: [`uname` info](https://en.wikipedia.org/wiki/Uname)
const HOST_OS_NAME: &str = if cfg!(all(
    target_os = "linux",
    any(target_env = "gnu", target_env = "")
)) {
    "GNU/Linux"
} else if cfg!(all(
    target_os = "linux",
    not(any(target_env = "gnu", target_env = ""))
)) {
    "Linux"
} else if cfg!(target_os = "android") {
    "Android"
} else if cfg!(target_os = "windows") {
    "MS/Windows" // prior art == `busybox`
} else if cfg!(target_os = "freebsd") {
    "FreeBSD"
} else if cfg!(target_os = "netbsd") {
    "NetBSD"
} else if cfg!(target_os = "openbsd") {
    "OpenBSD"
} else if cfg!(target_vendor = "apple") {
    "Darwin"
} else if cfg!(target_os = "fuchsia") {
    "Fuchsia"
} else if cfg!(target_os = "redox") {
    "Redox"
} else if cfg!(target_os = "illumos") {
    "illumos"
} else if cfg!(target_os = "solaris") {
    "solaris"
} else if cfg!(target_os = "cygwin") {
    "Cygwin"
} else {
    "unknown"
};

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

//=== Tests

#[test]
fn test_map_processor_mappings() {
    // ARM variants
    assert_eq!(map_processor("arm64"), "arm");
    assert_eq!(map_processor("aarch64"), "aarch64");
    assert_eq!(map_processor("armv6l"), "arm");
    assert_eq!(map_processor("armv7l"), "arm");
    assert_eq!(map_processor("armv8l"), "arm");

    // x86 variants
    assert_eq!(map_processor("x86_64"), "x86_64");
    assert_eq!(map_processor("amd64"), "x86_64");
    assert_eq!(map_processor("i386"), "i686");
    assert_eq!(map_processor("i486"), "i686");
    assert_eq!(map_processor("i586"), "i686");
    assert_eq!(map_processor("i686"), "i686");

    // Unknown/passthrough architectures
    assert_eq!(map_processor("riscv64"), "riscv64");
    assert_eq!(map_processor("powerpc64"), "powerpc64");
    assert_eq!(map_processor("unknown"), "unknown");
}
