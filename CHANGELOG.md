# Changelog

All notable changes are documented in this file.

Unreleased changes (if any) can be found in the latest [release pull-request]. 

[release pull-request]: https://github.com/jcornaz/benimator/pulls?q=is%3Apr+is%3Aopen+label%3A%22autorelease%3A+pending%22



## [3.0.0](https://github.com/jcornaz/benimator/compare/v2.2.0...v3.0.0) (2022-04-15)


### ⚠ BREAKING CHANGES

* **deps:** require bevy 0.7 (#51)

### chore

* **deps:** require bevy 0.7 ([#51](https://github.com/jcornaz/benimator/issues/51)) ([4403388](https://github.com/jcornaz/benimator/commit/4403388154881dbc17629aa23bf884577e573f84))

## [2.2.0](https://github.com/jcornaz/benimator/compare/v2.1.0...v2.2.0) (2022-03-06)


### Features

* add `current_frame_index` getter from animation state ([#49](https://github.com/jcornaz/benimator/issues/49)) ([1e30613](https://github.com/jcornaz/benimator/commit/1e306136a78baddbb2ae6fec660a4684acd851c5))



## [2.1.0](https://github.com/jcornaz/benimator/compare/v2.0.1...v2.1.0) (2022-03-02)


### Features

* add repeat-from animation mode ([#40](https://github.com/jcornaz/benimator/issues/40)) ([586859c](https://github.com/jcornaz/benimator/commit/586859c06cc3a63081d0f13156927c3a1db2bc0e))


### Deprecations

* deprecate the `AnimationMode` enum which wasn't used in any public API (#40)


## [2.0.1](https://github.com/jcornaz/benimator/compare/v2.0.0-rc.1...v2.0.1) (2022-01-29)


### Performance

* faster insertion/removal of the `Play` component by using `SparseStorage` ([#34](https://github.com/jcornaz/benimator/issues/34)) ([7f37562](https://github.com/jcornaz/benimator/commit/7f37562e19b3ff48388f2f359ce562ae153fb0d5))



## [2.0.0](https://github.com/jcornaz/benimator/compare/v2.0.0-rc.1...v2.0.0) (2022-01-24)

This is a stabilization release of `2.0.0-rc.1`. There is no new change since `2.0.0-rc.1`.


## [2.0.0-rc.1](https://github.com/jcornaz/benimator/compare/v1.1.0...v2.0.0-rc.1) (2022-01-19)


### ⚠ BREAKING CHANGES

* All struct fields are now private
* All enums are marked with `#[non_exhaustive]`
* The constructor of `AnimationPlugin` is now private. Use `AnimationPlugin::default()` instead.
* the `mode` field of `SpriteSheetAnimation` is no longer public
* require rust 1.58

### Features

* add ping pong animation mode ([#25](https://github.com/jcornaz/benimator/issues/25)) ([76a6306](https://github.com/jcornaz/benimator/commit/76a6306c6becb3f1ea6c1bbfabf36cb8bd9e2de8))


## [1.1.0](https://github.com/jcornaz/benimator/compare/v1.0.0...v1.1.0) (2022-01-17)


### Features

* create animation from iterator ([#23](https://github.com/jcornaz/benimator/issues/23)) ([6e670db](https://github.com/jcornaz/benimator/commit/6e670db5f162a963318fab2759cdb4a5f3fd18b0))


## [1.0.0](https://github.com/jcornaz/benimator/compare/v0.3.1...v1.0.0) (2022-01-08)


### ⚠ BREAKING CHANGES

* **Dependencies**: Require bevy version 0.6 ([#15](https://github.com/jcornaz/benimator/issues/15)) ([99da92b](https://github.com/jcornaz/benimator/commit/99da92b094f6ba855e6ce1de592ed483aa2c7064))
* **Dependencies**: Require rust 1.57
* The cargo feature `warnings` is removed, as it is no longer possible to add the animation as component by mistake
* Remove `Reflect` implementation from `SpriteSheetAnimation`, `AnimationMode` and `Frame`
* Update animation during the `CoreStage::Update` stage (#14)

### Features

* Update animation during the `CoreStage::Update` stage ([#14](https://github.com/jcornaz/benimator/issues/14)) ([2bcee87](https://github.com/jcornaz/benimator/commit/2bcee87fee72460755af1ff562838e431d8d0cb9))


## [0.3.1] - 2021-08-02

### Features

* Allow to reset animation (#8)


### Bug fixes

* impossiblity to restart an animation ran with 'Once' mode (#7)



## [0.3.0] - 2021-07-19

### Breaking changes

* The SpriteSheetAnimation is now an asset (#4)


### Features

* The SpriteSheetAnimation is now an asset (#4)



## [0.2.0] - 2021-07-12

### Breaking changes

* The system label enum is renamed to
AnimationPostUpdateSystem, as the animation now runs during the
post-update phase



## [0.1.1] - 2021-06-16

### Bug fixes

* Fix project title in readme



## [0.1.0] - 2021-06-16

### Features

* `SpriteSheetAnimation` component
* Create animation from index range
* Run animation once or repeated


[Unreleased]: ../../compare/v0.3.1...HEAD
[0.3.1]: ../../compare/v0.3.0...v0.3.1
[0.3.0]: ../../compare/v0.2.0...v0.3.0
[0.2.0]: ../../compare/v0.1.1...v0.2.0
[0.1.1]: ../../compare/v0.1.0...v0.1.1
[0.1.0]: ../../compare/...v0.1.0
