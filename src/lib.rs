// This file is part of the uutils coreutils package.
//
// (c) Alex Lyon <arcterus@mail.com>
//
// For the full copyright and license information, please view the LICENSE file
// that was distributed with this source code.

// spell-checker:ignore (abbrev/names) MSVC POSIX (names) rivy (rust) rustdoc RUSTDOCFLAGS

// Documentation
// See <https://docs.rs/platform-info> or <https://docs.rs/crate/platform-info>.
// Use `cargo doc --no-deps --open --target=i686-pc-windows-msvc` to view WinOS documentation for this crate.
// Use `cargo doc --no-deps --open --target=i686-unknown-linux-gnu` to view POSIX documentation for this crate.
// * note: `cargo rustdoc` is equivalent to `cargo doc --no-deps` and is what `docs.rs` uses to generate documentation.
// * ref: <https://users.rust-lang.org/t/docs-rs-does-not-show-my-documentation/70414/4> @@ <https://archive.is/W0N8W>

// Enable documentation warnings for missing documentation (for public items) and broken intra-doc links.
// * note: CI documentation linting has all warnings escalated to errors (using `RUSTDOCFLAGS="--deny warnings" cargo doc`)
#![warn(missing_docs)]
#![warn(rustdoc::broken_intra_doc_links)]
#![doc = include_str!("../README.md")]
// spell-checker:ignore (API) nodename osname sysname
// spell-checker:ignore (uutils) coreutils uutils
#![warn(unsafe_op_in_unsafe_fn)] // require explicit unsafe blocks inside unsafe fns
#![warn(unused_results)] // enable warnings for unused results

use std::ffi::OsStr;

mod lib_impl;

//===

// PlatformInfo
// Handles initial retrieval and holds cached information for the current platform.
pub use lib_impl::PlatformInfo;
#[cfg(unix)]
pub use lib_impl::UTSName;
#[cfg(windows)]
pub use lib_impl::{WinApiSystemInfo, WinOsVersionInfo};

// PlatformInfoError
/// The common error type for [`PlatformInfoAPI`].
pub use lib_impl::BoxedThreadSafeStdError as PlatformInfoError;

// PlatformInfoAPI
/// Defines the full API for [`PlatformInfo`].
// * includes `UNameAPI`
pub trait PlatformInfoAPI: UNameAPI {
    /// Creates a new instance of [`PlatformInfo`].
    /// <br> On some platforms, it is possible for this function to fail.
    fn new() -> Result<Self, PlatformInfoError>
    where
        Self: Sized;
}

// UNameAPI
/// Defines a trait API providing `uname` (aka "Unix name") style platform information.
// ref: <https://www.gnu.org/software/libc/manual/html_node/Platform-Type.html> @@ <https://archive.is/YjjWJ>
pub trait UNameAPI {
    /// The name of this implementation of the operating system.
    fn sysname(&self) -> &OsStr;

    /// The node name (network node hostname) of this machine.
    fn nodename(&self) -> &OsStr;

    /// The current release level of the operating system.
    fn release(&self) -> &OsStr;

    /// The current version level of the current release.
    fn version(&self) -> &OsStr;

    /// The name of the current system's hardware.
    fn machine(&self) -> &OsStr;

    /// The processor type (architecture) of the current system.
    ///
    /// Maps machine architecture strings to GNU coreutils-compatible processor types.
    /// For example, "arm64" may map to "arm", "x86_64" to "x86_64", etc.
    /// This provides more semantically meaningful processor information than the
    /// raw machine string in some contexts.
    fn processor(&self) -> &OsStr;

    /// The name of the current OS.
    fn osname(&self) -> &OsStr;
}
