[![latest release](https://img.shields.io/crates/v/wireplumber.svg?style=flat-square)](https://crates.io/crates/wireplumber) [![docs](https://img.shields.io/badge/API-docs-blue.svg?style=flat-square)](https://arcnmx.github.io/wireplumber.rs/main/wireplumber/) [![MIT](https://img.shields.io/badge/license-MIT-ff69b4.svg?style=flat-square)](../COPYING)

This crate provides a high-level interface to [PipeWire](https://pipewire.org/)'s [API](https://docs.pipewire.org/page_api.html) via [libwireplumber](https://pipewire.pages.freedesktop.org/wireplumber/index.html). Explore the [crate documentation](https://arcnmx.github.io/wireplumber.rs/main/wireplumber/) and the various [modules](https://arcnmx.github.io/wireplumber.rs/main/wireplumber/#modules) for information on how to start using WirePlumber with Rust.

# Examples

Some [examples](../examples/) are provided that can be built and run via Cargo:

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

# Development

Helper commands are available via [`cargo wp`](../ci/bin/cargo-wp) to facilitate development. Adding [ci/bin](../ci/bin) to your `PATH` is recommended - the provided [direnv shell](https://direnv.net/) is set up to do this by default.

- `cargo wp install gir` will install the pinned [GIR](https://github.com/gtk-rs/gir) version into ci/bin

  - `cargo wp install gir-files` for the associated data required by GIR

- `cargo wp gir` will update the [auto-generated source](./src/auto) for the main crate

- `cargo wp sys gir` will update the [sys bindings source](./sys/generate)

- `cargo wp todo` will display incomplete interfaces (this is just an alias for `gir -m not_bound`)

- `cargo wp fmt` will rustfmt the codebase

## GIR Schema

The WirePlumber GIR data is kept in [Wp-0.4.gir](../sys/generate/src/Wp-0.4.gir). A series of fixes must be applied to the upstream XML via the [wp-gir-filter](../ci/wp-gir-filter.sh) script.
