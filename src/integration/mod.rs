//! Integration with third party crates

/// Integration with bevy 0.7
#[cfg(any(feature = "bevy-app-07", feature = "bevy-sprite-07"))]
pub mod bevy_07;
