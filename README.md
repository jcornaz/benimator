# Benimator

[![License](https://img.shields.io/badge/license-Unlicense%20OR%20MIT-green)](#License)
[![Crates.io](https://img.shields.io/crates/v/benimator)](https://crates.io/crates/benimator)
[![Docs](https://docs.rs/benimator/badge.svg)](https://docs.rs/benimator)
[![Build](https://img.shields.io/github/workflow/status/jcornaz/benimator/build)](https://github.com/jcornaz/benimator/actions/workflows/build.yml)

A sprite animation library for rust game development.

Initially designed for [bevy](https://bevyengine.org), it is now engine agnostic.

*See [example of usage with bevy](examples/bevy.rs)*

## Installation

Add the dependency to your project

```sh
cargo add benimator
```

## Cargo features

| Feature | Description                                      |
|---------|--------------------------------------------------|
| `serde` | Implementations of `Serialize` and `Deserialize` |

*Feature flags not mentioned here are **NOT** part of the public API and are subject to breaking changes.*

## MSRV

The minimum supported rust version is, at all time, the latest stable.

Make sure to always use the latest stable rust version: `rustup update stable`.

## License

Licensed under either of

* The Unlicense ([UNLICENSE](UNLICENSE) or https://opensource.org/licenses/Unlicense)
* MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
