# Setup the iOS Build Environment

## Prepare your build environment

1. Install Xcode 13.0 or higher.
2. Ensure you have Python 3 installed: `brew install python`
3. Install linting and formatting tools: `brew install swiftlint`
4. (Optional, only required for building on the CLI) Install [xcpretty](https://github.com/xcpretty/xcpretty): `gem install xcpretty`

### Setting up Rust

Rust can be installed using `rustup`, with the following commands:

- `curl https://sh.rustup.rs -sSf | sh`
- `rustup update`

Platform specific toolchains need to be installed for Rust. This can be
done using the `rustup target` command. In order to enable building to real
devices and iOS emulators, the following targets need to be installed:

```
rustup target add aarch64-apple-ios x86_64-apple-ios aarch64-apple-ios-sim
```

## Building

This should be relatively straightforward and painless:

1. Ensure your repository is up-to-date.
2. Ensure Rust is up-to-date by running `rustup update`.
3. Run a build using the command `make build-swift`
    * To run tests use `make test-swift`

The above can be skipped if using Xcode.
The project directory can be imported and all the build details can be left to the IDE.
