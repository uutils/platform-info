platform-info
=============

[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Build status](https://ci.appveyor.com/api/projects/status/2wogy3wkenwreeq0/branch/master?svg=true)](https://ci.appveyor.com/project/Arcterus/platform-info/branch/master)
[![LoC](https://tokei.rs/b1/github/uutils/platform-info)](https://github.com/uutils/platform-info)

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
