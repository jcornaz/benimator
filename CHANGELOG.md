# Changelog

All notable changes are documented in this file.

Unreleased changes (if any) can be found in the latest [release pull-request]. 

[release pull-request]: https://github.com/jcornaz/benimator/pulls?q=is%3Apr+is%3Aopen+label%3A%22autorelease%3A+pending%22



### 2.0.1 (2022-05-18)


### ⚠ BREAKING CHANGES

* **deps:** require bevy 0.7 (#51)
* All struct fields are now private
* All enums are marked with `#[non_exhaustive]`
* The constructor of `AnimationPlugin` is now private. Use `AnimationPlugin::default()` instead.
* require rust 1.58
* the `mode` field of `SpriteSheetAnimation` is no longer public
* **deps:** the cargo feature `warnings` is removed, as it is no longer possible to add the animation as component by mistake
* Remove `Reflect` implementation from `SpriteSheetAnimation`, `AnimationMode` and `Frame`
* Update animation during the `CoreStage::Update` stage (#14)
* The SpriteSheetAnimation is now an asset (#4)
* That will make possible to move the animation definition in the assets.

### Features

* add `current_frame_index` getter from animation state ([#49](https://github.com/jcornaz/benimator/issues/49)) ([1e30613](https://github.com/jcornaz/benimator/commit/1e306136a78baddbb2ae6fec660a4684acd851c5))
* add ping pong animation mode ([#25](https://github.com/jcornaz/benimator/issues/25)) ([76a6306](https://github.com/jcornaz/benimator/commit/76a6306c6becb3f1ea6c1bbfabf36cb8bd9e2de8))
* add repeat-from animation mode ([#40](https://github.com/jcornaz/benimator/issues/40)) ([586859c](https://github.com/jcornaz/benimator/commit/586859c06cc3a63081d0f13156927c3a1db2bc0e))
* Allow to reset animation ([#8](https://github.com/jcornaz/benimator/issues/8)) ([1d54790](https://github.com/jcornaz/benimator/commit/1d547900cb9879e9cc678ab1573e557923f0d848))
* Create animaion from index range ([46a0320](https://github.com/jcornaz/benimator/commit/46a03202781c6fe45bb73a10da31678ca1dc751c))
* create animation from iterator ([#23](https://github.com/jcornaz/benimator/issues/23)) ([6e670db](https://github.com/jcornaz/benimator/commit/6e670db5f162a963318fab2759cdb4a5f3fd18b0))
* Run animation once or repeated ([0efff6b](https://github.com/jcornaz/benimator/commit/0efff6bf5ded1e83505c1bdcad05122588d8e296))
* sprite-sheet animation ([e13a69c](https://github.com/jcornaz/benimator/commit/e13a69c7923279f5e26e761426ea62d797f4d4b3))
* The SpriteSheetAnimation is now an asset ([#4](https://github.com/jcornaz/benimator/issues/4)) ([2a895a5](https://github.com/jcornaz/benimator/commit/2a895a5417f9ce60efc81449f12868d5cf73883b))
* Update animation during the `CoreStage::Update` stage ([#14](https://github.com/jcornaz/benimator/issues/14)) ([2bcee87](https://github.com/jcornaz/benimator/commit/2bcee87fee72460755af1ff562838e431d8d0cb9))


### Bug Fixes

* Fix project title in readme ([1e3ddb1](https://github.com/jcornaz/benimator/commit/1e3ddb164810d8d944095d753d2a4cb07de0c83c))
* impossiblity to restart an animation ran with 'Once' mode ([#7](https://github.com/jcornaz/benimator/issues/7)) ([4c7b5ad](https://github.com/jcornaz/benimator/commit/4c7b5ad6c008d9893be90c4ae366f9afbe8006ff))


### refactor

* Extract animation state into a dedicated component ([7d9c9ac](https://github.com/jcornaz/benimator/commit/7d9c9aca7cce35300325d8b919d89c56fad7dac6))


### Performance

* faster insertion/removal of the `Play` component by using `SparseStorage` ([#34](https://github.com/jcornaz/benimator/issues/34)) ([7f37562](https://github.com/jcornaz/benimator/commit/7f37562e19b3ff48388f2f359ce562ae153fb0d5))


### chore

* All enums are marked with `#[non_exhaustive]` ([249fc68](https://github.com/jcornaz/benimator/commit/249fc6816f1984b85d7410a32d5f0b493a0216c6))
* All struct fields are now private ([249fc68](https://github.com/jcornaz/benimator/commit/249fc6816f1984b85d7410a32d5f0b493a0216c6))
* **deps:** require bevy 0.7 ([#51](https://github.com/jcornaz/benimator/issues/51)) ([4403388](https://github.com/jcornaz/benimator/commit/4403388154881dbc17629aa23bf884577e573f84))
* **deps:** require bevy version 0.6 ([#15](https://github.com/jcornaz/benimator/issues/15)) ([99da92b](https://github.com/jcornaz/benimator/commit/99da92b094f6ba855e6ce1de592ed483aa2c7064))
* prepare release ([517cc9b](https://github.com/jcornaz/benimator/commit/517cc9ba42f5bf1b170f27e68dc3812fc0aac01b))
* prepare release ([0ac9a15](https://github.com/jcornaz/benimator/commit/0ac9a1576b5e35736198a0ce0f5ae9e61c57b74c))
* reduce API surface ([#27](https://github.com/jcornaz/benimator/issues/27)) ([249fc68](https://github.com/jcornaz/benimator/commit/249fc6816f1984b85d7410a32d5f0b493a0216c6))
* Remove `Reflect` implementation from `SpriteSheetAnimation`, `AnimationMode` and `Frame` ([04af5e1](https://github.com/jcornaz/benimator/commit/04af5e1e8b45ad5717f2798880dfb6de85d8af0d))
* require rust 1.58 ([8acb6d2](https://github.com/jcornaz/benimator/commit/8acb6d24927f7fea7a79009001be07148149e0e9))
* The constructor of `AnimationPlugin` is now private. Use `AnimationPlugin::default()` instead. ([249fc68](https://github.com/jcornaz/benimator/commit/249fc6816f1984b85d7410a32d5f0b493a0216c6))


### Documentation

* **changelog:** remove reference to semver ([bbc666d](https://github.com/jcornaz/benimator/commit/bbc666d43d592d1a00acc037ff21f7ade102852a))
* **changelog:** Update changelog format for release-please ([1ebf47e](https://github.com/jcornaz/benimator/commit/1ebf47e3b746cfb1adb8fa33bed42028c780c692))
* **examples:** Add a demo ([27bccf4](https://github.com/jcornaz/benimator/commit/27bccf4e92bb43974c7a248b3b3669c0d62ab4a4))
* **examples:** Add change animation example ([#3](https://github.com/jcornaz/benimator/issues/3)) ([00164ff](https://github.com/jcornaz/benimator/commit/00164ffeb2944fdbcf7cd80b64c767c1b7a3941f))
* hide deprecated members ([89e02e8](https://github.com/jcornaz/benimator/commit/89e02e8c283d0c047b7f397d9e6d8bd2ef974391))
* **readme:** add MSRV ([0164227](https://github.com/jcornaz/benimator/commit/0164227f9449b82d7088d155cb5a225d943c1291))
* **readme:** add ping-pong to the list of animation modes ([dfa1946](https://github.com/jcornaz/benimator/commit/dfa194683686ce5156e4c882e4dfd386197be68d))
* **readme:** Clarify that SpriteSheetAnimation is a asset (and not a component) ([d77209b](https://github.com/jcornaz/benimator/commit/d77209b6c1986de194f202c48e99c6645a3420ee))
* **readme:** Credit author of the coin asset ([d32180f](https://github.com/jcornaz/benimator/commit/d32180f410aca1e463c893888825407e625495cb))
* **readme:** fix bevy compatiblity matrix ([2bc89f7](https://github.com/jcornaz/benimator/commit/2bc89f71a07064c11b793ac2de7818623101fe95))
* **readme:** improve format of the bevy compatibility matrix ([6be327f](https://github.com/jcornaz/benimator/commit/6be327fbeff7bb1234e0214c436f2faffd2b873b))
* **readme:** Minor updates after upgrade to bevy 0.6 ([439c37d](https://github.com/jcornaz/benimator/commit/439c37d86fe0916443b3768b98044741647d8ef9))
* **readme:** Remove (broken) audit badge ([f2e6dac](https://github.com/jcornaz/benimator/commit/f2e6daccfc50479ff8e9525a744054ad0f37481a))
* **readme:** remove obsolete recomendation ([9da0890](https://github.com/jcornaz/benimator/commit/9da0890d37f4046ec28e117f8bce6372193976c3))
* **readme:** remove some unnecessary badges ([1b00af4](https://github.com/jcornaz/benimator/commit/1b00af46717612099bb64540f2aa00e0d89adb4a))
* **readme:** remove version number from readme ([54583e0](https://github.com/jcornaz/benimator/commit/54583e06adafdf500b3443e973b588a9283a6d62))
* **readme:** some rewording in readme ([9f7f687](https://github.com/jcornaz/benimator/commit/9f7f6874046272ffa8dc12ada5176983d7fcf3d5))
* **readme:** update bevy compatibility matrix ([5de0b4b](https://github.com/jcornaz/benimator/commit/5de0b4bba4447ef983f1b2ae0435d2349955b989))
* **readme:** Update contact section ([edc6936](https://github.com/jcornaz/benimator/commit/edc6936f88220d360a97a5a9e8a6b515d703b8a5))
* **readme:** update version ([91f9aba](https://github.com/jcornaz/benimator/commit/91f9aba814fb8af0ce1210e6a25c2b3eb597615e))
* **readme:** Write a readme ([694409c](https://github.com/jcornaz/benimator/commit/694409cfbe7cefb5dd5708d9768902fec1676e79))

## [3.0.0](https://github.com/jcornaz/benimator/compare/v2.2.0...v3.0.0) (2022-04-15)


### ⚠ BREAKING CHANGES

* **dependencies:** require bevy 0.7 (#51)


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
