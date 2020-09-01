# Dependency Management Guidelines

This repository uses third-party code from a variety of sources, so we need to be mindful
of how these dependencies will affect our consumers.  Considerations include:

* General code quality.
* [Licensing compatibility](https://www.mozilla.org/en-US/MPL/license-policy/#Licenses_Compatible_with_the_MPL).
* Handling of security vulnerabilities.
* The potential for [supply-chain compromise](https://medium.com/intrinsic/compromised-npm-package-event-stream-d47d08605502).

We're still evolving our policies in this area, but these are the
guidelines we've developed so far.

## Rust Code

Unlike [Firefox](https://firefox-source-docs.mozilla.org/build/buildsystem/rust.html),
we do not vendor third-party source code directly into the repository.  Instead we rely on
`Cargo.lock` and its hash validation to ensure that each build uses an identical copy
of all third-party crates.  These are the measures we use for ongoing maintenance of our
existing dependencies:

* Check `Cargo.lock` into the repository.
* Generate built artifacts using the `--locked` flag to `cargo build`, as an additional
  assurance that the existing `Cargo.lock` will be respected.
* Use [cargo-deny](https://github.com/EmbarkStudios/cargo-deny) for a basic license-compatibility
  check as part of CI, to guard against human error.

Adding a new dependency, whether we like it or not, is a big deal - that dependency and everything
it brings with it will become part of Firefox-branded products that we ship to end users.
We try to balance this responsibility against the many benefits of using existing code, as follows:

* In general, be conservative in adding new third-party dependencies.
  * For trivial functionality, consider just writing it yourself.
    Remember the cautionary tale of [left-pad](https://www.theregister.co.uk/2016/03/23/npm_left_pad_chaos/).
  * Check if we already have a crate in our dependency tree that can provide the needed functionality.
* Prefer crates that have a a high level of due-diligence already applied, such as:
  * Crates that are [already vendored into Firefox](https://dxr.mozilla.org/mozilla-central/source/third_party/rust).
  * Crates from [rust-lang-nursery](https://github.com/rust-lang-nursery) or [rust-lang](https://github.com/rust-lang).
  * Crates that appear to be widely used in the Rust community.
* Check that it is clearly licensed and is [MPL-2.0 compatible](https://www.mozilla.org/en-US/MPL/license-policy/#Licenses_Compatible_with_the_MPL).
* Take the time to investigate the crate's source and ensure it is suitably high-quality.
  * Be especially wary of uses of `unsafe`, or of code that is unusually resource-intensive to build.
  * Development dependencies do not require as much scrutiny as dependencies that will ship in consuming applications,
    but should still be given some thought.
    * There is still the potential for supply-chain compromise with development dependencies!
* Explicitly describe your consideration of these points in the PR that introduces the new dependency.

Updating to new versions of existing dependencies is a normal part of software development
and is not accompanied by any particular ceremony.

### Dependency updates

We use [Dependabot] to automatically open pull requests when dependencies release a new version.
All CI tasks run on those pull requests to ensure updates don't break anything.

As `glean-core` is now also vendored into mozilla-central, including all of its Rust dependencies, we need to be a bit careful about these updates.
Landing upgrades of the Glean SDK itself in mozilla-central is a separate process.
See [Updating the Glean SDK][updating-sdk] in the Firefox Source Docs.
Following are some guidelines to ensure compatibility with mozilla-central:

* Patch releases of dependencies should be fine in most cases.
* Minor releases should be compatible. Best to check the changelog of the updated dependency to make sure. These updates should only land in `Cargo.lock`. No updates in `Cargo.toml` are necessary
* Major releases will always need a check against mozilla-central. See [Updating the Glean SDK][updating-sdk].

In case of uncertainty defer a decision to :janerik or :chutten.

#### Manual dependency updates

You can manually check for outdated dependencies using [cargo-outdated].
Individual crate updates for version compatible with the requirement in `Cargo.toml` can be done using:

```
cargo update -p $cratename
```

To update all transitive dependencies to the latest semver-compatible version use:

```
cargo update
```

[Dependabot]: https://dependabot.com/
[updating-sdk]: https://firefox-source-docs.mozilla.org/toolkit/components/glean/updating_sdk.html
[cargo-outdated]: https://crates.io/crates/cargo-outdated
