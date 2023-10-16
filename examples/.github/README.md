Aside from those found in the [crate documentation](https://arcnmx.github.io/wireplumber.rs/main/wireplumber/), these stand-alone examples can be built and run via Cargo:

``` bash
$ cargo run -p wp-examples --bin wpexec -- --help
... snip ...

# try out the default lua example:
$ cargo run -p wp-examples --bin wpexec

# or load the example plugin module:
$ cargo build --workspace --examples &&
  cargo run -p wp-examples --bin wpexec -- --type wireplumber
```

It’s recommended to poke around their source code in a local checkout, but you can also view their generated documentation and source code online:

- [wpexec](https://arcnmx.github.io/wireplumber.rs/main/wpexec/)

- [static-link module](https://arcnmx.github.io/wireplumber.rs/main/static_link_module/)
