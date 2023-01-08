# Benimator

[![License](https://img.shields.io/badge/license-Unlicense%20OR%20MIT-green)](#License)
[![Crates.io](https://img.shields.io/crates/v/benimator)](https://crates.io/crates/benimator)
[![Docs](https://docs.rs/benimator/badge.svg)](https://docs.rs/benimator)

A sprite animation library for rust game development.

Initially designed for [bevy], it is now engine agnostic.

## Goals

This project aim to provide the building the blocks to facilitate 2d sprite animation
with any game engine.

## Non-goals

Benimator is not rendering anything. It only keeps track of sprite indices.

One is expected to use benimator with a game engine such as [bevy].

[bevy]: https://bevyengine.org

## How it looks like

At its core benimator is an `Animation` data structure
and a `State` to track the frame-index as time pass.

```rust
// Create an animation
let animation = Animation::from_indices(0..=3, FrameRate::from_fps(10.0));

// Create a new animation state
let mut state = State::new();

// In the game loop, for each update, tell the state how much time has elapsed
let delta_time = Duration::from_millis(250);
state.update(&animation, delta_time);

// Then get the current frame index.
// (so that we can tell our engine to render the sprite at that index)
assert_eq!(state.frame_index(), 2);
```

Have a look at the [examples](https://github.com/jcornaz/benimator/tree/main/examples) for complete examples using the [bevy] game engine.

## Installation

benimator is published on [crates.io](https://crates.io/crates/benimator)

You can add the dependency to your cargo file with:

```sh
cargo add benimator
```

## Cargo features

| Feature | Description                                      |
|---------|--------------------------------------------------|
| `serde` | Implementations of `Serialize` and `Deserialize` |

*Feature flags not mentioned here are **NOT** part of the public API and are subject to breaking changes!*

## MSRV

The minimum supported rust version is, at all times, the latest stable.

## License

Licensed under either of

* The Unlicense ([UNLICENSE](UNLICENSE) or https://opensource.org/licenses/Unlicense)
* MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
