# 3. cargo-smart-release

Date: 2022-07-19

## Status

Accepted

## Context

I have tried many processes and tools for releasing libraries and softwares. They all have some flaws.

Every commit in the main branch should be releasable. That is already guaranteed by automated CI checks.

The release process should be as simple and as fast as possible, by using as much automation as possible.
If it is not fully automated, it should be very easy and fast to cut a release at any time.

At least the following should be automated:
 * Changelog scafolding
 * Version number infered from changes (according to the conventional commits)
 * Tag, github release and publication to crates.io

Though, that doesn't mean I want every single commit to be released immediatly. 

Bug fixes should be released as soon as the fix is merged in the main branch.
Features may wait 1 or 2 weeks to be bundled with other features in a minor release, to avoid spamming users with release notes.
Breaking changes should happen as rarely as possible.

Changelog generation is helpful, but I often would like to reviewe and modify it before release.
Being able to add a "release-note" prose for bigger releases (major and minor) would be especially appreciable.

## Decision

I'll use `cargo-smart-release` to drive the release process.

0nly commit that should appear in the changelog will follow conventional-commits.

## Consequences

Releases will be triggered manually when it makes sense.

It is still cheap to release and can be done at any time on-demand.

The changelog will be reviewed and adapted when releasing.

To get the list of unreleased changes, one will need to use `cargo changelog` or the git log.

Only a single `main` branch is needed (unlike when using `semantic-release` that required a branch per pre-release stage)
