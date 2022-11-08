platform-info
=============

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CodeCov](https://codecov.io/gh/uutils/platform-info/branch/master/graph/badge.svg)](https://codecov.io/gh/uutils/platform-info)

A simple cross-platform way to get information about the currently running
system.

Example
-------

This simple example:
```
use platform_info::*;

fn main() {
    let uname = PlatformInfo::new().unwrap();
    println!("{}", uname.sysname());
    println!("{}", uname.nodename());
    println!("{}", uname.release());
    println!("{}", uname.version());
    println!("{}", uname.machine());
}
```
should return something like:
```
Linux
hostname
5.10.0-8-amd64
#1 SMP Debian 5.10.46-4 (2021-08-03)
x86_64
```

License
-------

`platform-info` is licensed under the MIT License - see the LICENSE file for details.
