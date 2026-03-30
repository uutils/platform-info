# platform-info

<!-- spell-checker:ignore (API) nodename osname sysname (rust) println -->
<!-- spell-checker:ignore (wasm) wasip wasmtime -->

[![Crates.io](https://img.shields.io/crates/v/platform-info.svg)](https://crates.io/crates/platform-info)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![CodeCov](https://codecov.io/gh/uutils/platform-info/branch/main/graph/badge.svg)](https://codecov.io/gh/uutils/platform-info/tree/main)

A simple cross-platform way to get information about the currently running system.

## Examples

This simple example:

```rust
// examples/ex.rs
// * use `cargo run --example ex` to execute this example

// spell-checker:ignore (API) nodename osname sysname

use platform_info::*;

fn main() {
    let info = PlatformInfo::new().expect("Unable to determine platform info");
    // println!("info={:#?}", info);

    println!("{}", info.sysname().to_string_lossy());
    println!("{}", info.nodename().to_string_lossy());
    println!("{}", info.release().to_string_lossy());
    println!("{}", info.version().to_string_lossy());
    println!("{}", info.machine().to_string_lossy());
    println!("{}", info.processor().to_string_lossy());
    println!("{}", info.osname().to_string_lossy());
}
```

should display something like:

```text
Linux
hostname
5.10.0-8-amd64
#1 SMP Debian 5.10.46-4 (2021-08-03)
x86_64
x86_64
GNU/Linux
```

> Using `cargo run --example ex` will build and execute this [example code](examples/ex.rs).

### WASI

To cross-compile and run the example under [WASI](https://wasi.dev/), first install
[wasmtime](https://wasmtime.dev/):

```shell
curl https://wasmtime.dev/install.sh -sSf | bash
```

Then build and run:

```shell
cargo build --target wasm32-wasip1 --example ex
wasmtime target/wasm32-wasip1/debug/examples/ex.wasm
```

This should display:

```text
wasi
localhost
0.0.0
0.0.0
wasm32
wasm32
WASI
```

Other examples can be found in the [examples](examples) directory.

## License

`platform-info` is licensed under the [MIT License](LICENSE).
