#![warn(
    future_incompatible,
    nonstandard_style,
    rust_2018_idioms,
    clippy::pedantic
)]
#![allow(clippy::needless_pass_by_value, clippy::module_name_repetitions)]
#![cfg_attr(nightly, feature(doc_auto_cfg))]

#[cfg(test)]
#[macro_use]
extern crate rstest;

pub use animation::{Animation, Frame};
pub use player::State;

mod animation;
mod player;
