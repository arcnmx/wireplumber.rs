[![latest release](https://img.shields.io/crates/v/wireplumber.svg?style=flat-square)](https://crates.io/crates/wireplumber) [![docs](https://img.shields.io/badge/API-docs-blue.svg?style=flat-square)](https://arcnmx.github.io/wireplumber.rs/v0.1.0/wireplumber/) [![MIT](https://img.shields.io/badge/license-MIT-ff69b4.svg?style=flat-square)](https://github.com/arcnmx/wireplumber.rs/blob/v0.1.0/COPYING)

This crate provides a high-level interface to [PipeWire](https://pipewire.org/)'s [API](https://docs.pipewire.org/page_api.html) via [libwireplumber](https://pipewire.pages.freedesktop.org/wireplumber/index.html). Explore the [crate documentation](https://arcnmx.github.io/wireplumber.rs/v0.1.0/wireplumber/) and the various [modules](https://arcnmx.github.io/wireplumber.rs/v0.1.0/wireplumber/#modules) for information on how to start using WirePlumber with Rust.

``` toml
[dependencies]
wireplumber = { version = "0.1", features = ["v0_4_15"] }
```

# Examples

Some [examples](https://github.com/arcnmx/wireplumber.rs/tree/v0.1.0/examples/) are provided that can be built and run via Cargo:

``` bash
$ cargo run -p wp-examples --bin wpexec -- --help
... snip ...

# try out the default lua example:
$ cargo run -p wp-examples --bin wpexec

# or load the example plugin module:
$ cargo build --workspace --examples &&
  cargo run -p wp-examples --bin wpexec -- --type wireplumber
```

## External

Projects using wireplumber.rs:

- [WirePlumber Scripts](https://github.com/arcnmx/wireplumber-scripts) is a personal collection of plugins, some previously written as Lua scripts.

# Use Cases

This project aims to facilitate the following applications:

- Enabling Rust to be a viable language for writing session management logic as an alternative to the officially supported Lua scripting engine or GObject C APIs

- WirePlumber plugins that can augment or expose APIs for Lua configuration scripts to use

- Stand-alone pipewire clients as an alternative to [pipewire-rs](https://gitlab.freedesktop.org/pipewire/pipewire-rs)
