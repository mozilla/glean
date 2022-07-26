# Using locally-published Glean in Fenix

> **Note**: this substitution-based approach will not work for testing updates to `glean_parser` as shipped in the Glean Gradle Plugin.
> For replacing `glean_parser` in a local build, see [Substituting `glean_parser`](glean-parser-substitution.md).

## Preparation

Clone the Glean SDK, android-components and Fenix repositories:

```sh
git clone https://github.com/mozilla/glean
git clone https://github.com/mozilla-mobile/fenix
```

## Substituting projects

android-components has custom build logic for dealing with composite builds,
so you should be able to configure it by simply adding the path to the Glean repository in the correct `local.properties` file:

In `fenix/local.properties`:

```groovy
autoPublish.glean.dir=../glean
```

This will auto-publish Glean SDK changes to a local repository and consume them in Fenix.
You should now be able to build and run Fenix (assuming you could before all this).

## Caveats

1. This assumes you have followed the [Android/Rust build setup](setup-android-build-environment.md)
2. Make sure you're fully up to date in all repositories, unless you know you need to not be.
3. This omits the steps if changes needed because, e.g. Glean made a breaking change to an API used in android-components.
   These should be understandable to fix, you usually should be able to find a PR with the fixes somewhere in the android-component's list of pending PRs
   (or, failing that, a description of what to do in the Glean changelog).
4. Ask in the [#glean channel on chat.mozilla.org](https://chat.mozilla.org/#/room/#glean:mozilla.org).
