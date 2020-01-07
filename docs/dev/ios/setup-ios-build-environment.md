# Setup the iOS Build Environment

## Prepare your build environment

1. Install Xcode 11.0
2. Install [Carthage](https://github.com/Carthage/Carthage): `brew install carthage`
3. Ensure you have Python 3 installed: `brew install python`
4. Install linting and formatting tools: `brew install swiftlint swiftformat`
5. Run `bin/bootstrap.sh` to download dependencies.
6. (Optional, only required for building on the CLI) Install [xcpretty](https://github.com/xcpretty/xcpretty): `gem install xcpretty`

To build the Swift documentation:

1. Install the latest Ruby: `brew install ruby`
2. Make the installed Ruby available: `export PATH=/usr/local/opt/ruby/bin:$PATH` (and add that line to your `.bashrc`)
3. Install the documentation tool [jazzy](https://github.com/realm/jazzy): `gem install jazzy`

### Setting up Rust

Rust can be installed using `rustup`, with the following commands:

- `curl https://sh.rustup.rs -sSf | sh`
- `rustup update`

Platform specific toolchains need to be installed for Rust. This can be
done using the `rustup target` command. In order to enable building to real
devices and iOS emulators, the following targets need to be installed:

```
rustup target add aarch64-apple-ios x86_64-apple-ios
```

The `mdbook` crate is required in order to build documentation:

- `cargo install mdbook`

## Building

This should be relatively straightforward and painless:

1. Ensure your repository is up-to-date.

2. Ensure Rust is up-to-date by running `rustup update`.

3. Run a build using the command `make build-swift`
    * To run tests use `make test-swift`

The above can be skipped if using Xcode.
The project directory can be imported and all the build details can be left to the IDE.
