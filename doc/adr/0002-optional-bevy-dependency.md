# 2. Optional Bevy Dependency

Date: 2022-07-13

## Status

Accepted

## Context

Bevy is a very unstable project, and a new major (breaking) version is released every 3 months.

But not everybody always update to the latest bevy version immediatly. [this statistic](https://lib.rs/crates/bevy/rev) give a hint of how many project choose to stay behind (at least temporarily) when a new major version of bevy comes out.

In order to increase benimator's stability, we need a way to be more stable than bevy.


## Decision

All bevy crates will be an optional dependency and disabled by default. The cargo feature name will contain the major bevy version they refer to (for example `bevy-07`).

This is a breacking change and will be timed with the update to bevy 0.8, which would have also required breaking the API even if this decision was not taken.

## Consequences

It will be possible to add support for newer major versions of bevy without removing support for older versions. A new major version of bevy will no longer force benimator to also break its API. 

Users will be able to update the version of benimator while staying on an old version of bevy until they have the time to update bevy.

It would be possible to add support for older versions of bevy (0.1 - 0.7) if necessary. (But that won't be added eagerly)

Though, it'll enable greater stability of benimator's API, to make bevy an optional dependency is a breaking change.

It will increase the overall complexity.
