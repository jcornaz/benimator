# Benimator

[![License](https://img.shields.io/badge/license-Unlicense%20OR%20MIT-green)](#License)
[![Crates.io](https://img.shields.io/crates/v/benimator)](https://crates.io/crates/benimator)
[![Docs](https://docs.rs/benimator/badge.svg)](https://docs.rs/benimator)
[![Bevy tracking](https://img.shields.io/badge/Bevy%20tracking-released%20version-lightblue)](https://github.com/bevyengine/bevy/blob/main/docs/plugins_guidelines.md#main-branch-tracking)
[![Build](https://img.shields.io/github/workflow/status/jcornaz/benimator/build)](https://github.com/jcornaz/benimator/actions/workflows/build.yml)

A sprite sheet animation plugin for [bevy](https://bevyengine.org)


## Features

* A `SpriteSheetAnimation` asset
* Automatic update indices of a `TextureAtlasSprite`
* Animation modes: `once`, `repeat` and `ping_pong`
* An animation is playing if, and only if, a `Play` component is present in the entity
  * Simply remove/insert the `Play` component to pause/resume an animation
* The animation can be defined from an index-range, or an arbitrary list of indices
* Each frame may have a different duration


## Usage

```rust
fn main() {
  App::new()
    .add_plugins(DefaultPlugins)
    .add_plugin(AnimationPlugin::default()) // <-- Add the plugin
    .add_startup_system(spawn)
    .run();
}

fn spawn(
  mut commands: Commands,
  asset_server: Res<AssetServer>,
  mut textures: ResMut<Assets<TextureAtlas>>,
  mut animations: ResMut<Assets<SpriteSheetAnimation>>,
) {
  // Don't forget the camera ;-)
  commands.spawn_bundle(OrthographicCameraBundle::new_2d());

  // Create an animation
  // Here we use an index-range (from 0 to 4) where each frame has the same duration
  let animation_handle = animations.add(SpriteSheetAnimation::from_range(
    0..=4,
    Duration::from_millis(100),
  ));

  commands
          // Spawn a bevy sprite-sheet
          .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.add(TextureAtlas::from_grid(asset_server.load("coin.png"), Vec2::new(16.0, 16.0), 5, 1)),
            transform: Transform::from_scale(Vec3::splat(10.0)),
            ..Default::default()
          })
          // Insert the asset handle of the animation
          .insert(animation_handle)
          // Start the animation immediately. Remove this component in order to pause the animation.
          .insert(Play);
}
```

Here is the result:

![Example result](doc/images/coin.gif)

*(Asset by [La Red Games](https://laredgames.itch.io/gems-coins-free) - CC0)*

For more details see the [documentation](https://docs.rs/benimator)


## Installation

Add the dependency to your project

```sh
cargo add benimator
```

## Cargo features

| Feature | Description |
|---------|-------------|
| `serde` | Implementation of serde traits for deserializaion 
| `yaml` | Asset loader for yaml files
| `ron` | Asset loader for ron files
| `bevy-08` | Integration with bevy 0.8

*Feature flags not mentioned here are **NOT** part of the public API and are subject to breaking changes.*

## MSRV

The minimum supported rust version is currently: `1.62`

**It *may* be increased to a newer stable version in a minor release.** (but only if needed)

It *will* be increased to the latest stable version in a major release. (even if not needed)


## Bevy Version Compatibility

| bevy | benimator |
|------|-----------|
| 0.7  | 3         |
| 0.6  | 1, 2      |
| 0.5  | 0.1 - 0.3 |

*Note: Only the latest published version of benimator is supported* 


## Contribute / Contact

Discussions, issues and pull requests are welcome.

It is possible to directly discuss with me (`Jomag#2675`) via the [bevy discord](https://discord.com/invite/gMUk5Ph).

If you want to understand the "architecture" decisions made you may look at the [doc/adr](doc/adr) directory.


## License

Licensed under either of

* The Unlicense ([UNLICENSE](UNLICENSE) or https://opensource.org/licenses/Unlicense)
* MIT License ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
