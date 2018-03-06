// This file is part of the uutils coreutils package.
//
// (c) Alex Lyon <arcterus@mail.com>
//
// For the full copyright and license information, please view the LICENSE file
// that was distributed with this source code.
//

extern crate winapi;

use self::winapi::um::sysinfoapi::{SYSTEM_INFO, GetSystemInfo};
use self::winapi::um::winnt::*;
use std::mem;
use std::borrow::Cow;
use std::io;

pub struct Uname {
    inner: SYSTEM_INFO
}

impl Uname {
    pub fn new() -> io::Result<Uname> {
        unsafe {
            let mut info = mem::uninitialized();
            GetSystemInfo(&mut info);
            Ok(Uname { inner: info })
        }
    }

    // FIXME: need to implement more architectures (e.g. ARM)
    pub fn machine(&self) -> Cow<str> {
        let arch = unsafe {
            match self.inner.u.s().wProcessorArchitecture {
                PROCESSOR_ARCHITECTURE_AMD64 => "x86_64",
                PROCESSOR_ARCHITECTURE_INTEL => "x86",
                _ => unimplemented!()
            }
        };
        Cow::from(arch)
    }
}
