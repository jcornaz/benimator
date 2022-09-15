# How to contribute

## Ask for help, propose a feature request a feature or report a bug

Feel free to create an [issue](https://github.com/jcornaz/benimator/issues) or open a [discussion](https://github.com/jcornaz/benimator/discussions)


## Choose an issue to work on

You don't *need* to find an open issue to contribute a PR. But it is better to make sure the change is actually desirable.

I assign myself to issues when I am working on them. So you can safely pick any
[unassigned issue](https://github.com/jcornaz/benimator/issues?utf8=%E2%9C%93&q=is%3Aissue+is%3Aopen+no%3Aassignee+).

You may (but don't have to) write a message in the issue to say you are working on it.

## Build from source

Run the tests
```sh
cargo test --all-features
```

Check code style
```
cargo fmt
cargo clippy --all-targets --all-features
```

Build (and open) documentation
```
cargo doc --no-deps --all-features --open
```

## Coding standards

### Tests

This is a test-driven project. Every new feature and bug fixes must come with tests.
If you need help to test your feature or fix, you can ask in a draft PR, and I'll do my best to help you.

### API stability

#### Add stable API

When writting new code make sure you don't expose any unecessary technical detail:
* Never expose struct fields!
* Use `#[non_exhaustive]` for enums and unit structs
* Be very carefull with public enums. In doubt keep them private.
* Don't expose types and functions that do no need to be public
* Don't eagerly implement traits that are not yet needed or related to the use-case
  * except for `Debug`, `Clone`, `Eq`, `PartialEq` and `Default` that may be implemented eagerly when it makes sense
  * note that if a `new()` constuctor does not make sense, `Default` should **not** be implemented
  * in case of doubt, don't implement what you don't need, don't worry we can add it later
* Avoid promissing too much in return types. (e.g. `&[T]` are better that `&Vec<T>`)

New API may be gated behind a `unstable-` cargo flag until it is stabilized.

#### Do not break existing API

Do not break public API. (See https://github.com/rust-lang/rfcs/blob/master/text/1105-api-evolution.md to understand what constitute a breaking change)

Instead create a new API. Eventually we may deprecate the old one and hide it from the doc.

The API may eventually be broken (in a new major version). But I want to avoid that for as long as possible (forever would be perfect).
I see a breaking change as good only if it makes future breaking changes less likely to be needed. 
For example, to make struct field privates is a an acceptable breaking change.

If you don't see how to improve an API without breaking it, you can start a discussion in the issues.

## Open a pull request

Don't be afraid of small steps. I'd rather review 5 tiny pull-requests than 1 big.

But to be merged a pull-request needs to be in state ready for release:
* New features and Bug fixes must comes with automated tests
* The build must pass
* The documentation must be up-to-date
