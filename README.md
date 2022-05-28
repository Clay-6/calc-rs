# calc-rs

A command-line calculator written in Rust

## Installation

### Through Cargo

Ensure you have [rustup](https://rustup.rs) installed & the latest stable compiler through
`rustup update stable`. Then, run

```shell
cargo install calcrs
```

### Manual

Download the [latest GitHub release](https://github.com/Clay-6/calc-rs/releases/latest) and
move the executable to a directory on you `PATH` environment variable. Currently only Windows
executables are available. (See [Through Cargo](#through-cargo) or [Building from Source](#building-from-source)
for other platforms)

### Windows Installer

Download the installer from the [latest GitHub release](https://github.com/Clay-6/calc-rs/releases/latest)
and run it. All necessary changes to your `PATH` will be made for you.

## Building from Source

Ensure you have the latest stable Rust version. For example, using [rustup](https://rustup.rs) run
`rustup update stable`. Then, simply run `cargo run` to run a debug version, or `cargo run --release`
for a release version. Run `cargo build` or `cargo build --release` to generaten executable file without
immediately running it.
