# Changelog

All notable changes are documented in this file.

This project adheres to [Semantic Versioning].

Unreleased changes (if any) can be found in the latest [release pull-request]. 

[Semantic Versioning]: https://semver.org/spec/v2.0.0.html
[release pull-request]: https://github.com/jcornaz/benimator/pulls?q=is%3Apr+is%3Aopen+label%3A%22autorelease%3A+pending%22



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
