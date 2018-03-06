// This file is part of the uutils coreutils package.
//
// (c) Alex Lyon <arcterus@mail.com>
//
// For the full copyright and license information, please view the LICENSE file
// that was distributed with this source code.
//

#[cfg(unix)]
pub use self::unix::*;
#[cfg(windows)]
pub use self::windows::*;
#[cfg(target_os = "redox")]
pub use self::redox::*;

#[cfg(unix)]
mod unix;
#[cfg(windows)]
mod windows;
#[cfg(target_os = "redox")]
mod redox;
