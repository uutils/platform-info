// This file is part of the uutils coreutils package.
//
// (c) Alex Lyon <arcterus@mail.com>
//
// For the full copyright and license information, please view the LICENSE file
// that was distributed with this source code.
//

use std::borrow::Cow;
use std::io::{self, Read};
use std::fs::File;

pub struct Uname {
    kernel_name: String,
    nodename: String,
    kernel_release: String,
    kernel_version: String,
    machine: String
}

impl Uname {
    pub fn new() -> io::Result<Uname> {
        let mut inner = Box::new(String::new());
        File::open("sys:uname")?.read_to_string(&mut inner)?;

        let mut lines = inner.lines();

        let kernel_name = lines.next().unwrap();
        let nodename = lines.next().unwrap();
        let kernel_release = lines.next().unwrap();
        let kernel_version = lines.next().unwrap();
        let machine = lines.next().unwrap();

        // FIXME: don't actually duplicate the data as doing so is wasteful
        Ok(Uname {
            kernel_name: kernel_name.to_owned(),
            nodename: nodename.to_owned(),
            kernel_release: kernel_release.to_owned(),
            kernel_version: kernel_version.to_owned(),
            machine: machine.to_owned()
        })
    }

    pub fn sysname(&self) -> Cow<str> {
        Cow::from(self.kernel_name.as_str())
    }

    pub fn nodename(&self) -> Cow<str> {
        Cow::from(self.nodename.as_str())
    }

    pub fn release(&self) -> Cow<str> {
        Cow::from(self.kernel_release.as_str())
    }

    pub fn version(&self) -> Cow<str> {
        Cow::from(self.kernel_version.as_str())
    }

    pub fn machine(&self) -> Cow<str> {
        Cow::from(self.machine.as_str())
    }
}