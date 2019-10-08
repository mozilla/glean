default: test

# All builds

build: build-rust

build-rust:
	cargo build --all

build-kotlin:
	./gradlew build -x test

build-swift:
	bin/run-ios-build.sh

build-apk:
	./gradlew glean-core:build
	./gradlew glean-sample-app:build

.PHONY: build build-rust build-kotlin build-swift build-apk

# All tests

test: test-rust

test-rust:
	cargo test --all

test-rust-with-logs:
	RUST_LOG=glean_core=debug cargo test --all -- --nocapture --test-threads=1

test-kotlin:
	./gradlew test

test-swift:
	bin/run-ios-tests.sh

.PHONY: test test-rust test-rust-with-logs test-kotlin test-swift

# Linting

lint: clippy

clippy:
	cargo clippy --all --all-targets --all-features -- -D warnings

ktlint:
	./gradlew ktlint detekt

swiftlint:
	swiftlint --strict

.PHONY: lint clippy ktlint swiftlint

# Formatting

fmt: rustfmt

rustfmt:
	cargo fmt --all

swiftfmt:
	swiftformat glean-core/ios samples/ios --swiftversion 5 --verbose

.PHONY: fmt rustfmt swiftfmt

# Docs

docs: rust-docs kotlin-docs

rust-docs:
	bin/build-rust-docs.sh

kotlin-docs:
	./gradlew docs

swift-docs:
	bin/build-swift-docs.sh

.PHONY: docs rust-docs kotlin-docs swift-docs

linkcheck: docs
	# Requires https://wummel.github.io/linkchecker/
	linkchecker --ignore-url javadoc --ignore-url docs/glean_core build/docs
.PHONY: linkcheck

# Utilities

android-emulator:
	$(ANDROID_HOME)/emulator/emulator -avd Nexus_5X_API_P -netdelay none -netspeed full
.PHONY: android-emulator

cbindgen:
	RUSTUP_TOOLCHAIN=nightly \
	cbindgen glean-core/ffi --lockfile Cargo.lock -o glean-core/ffi/glean.h
	cp glean-core/ffi/glean.h glean-core/ios/Glean/GleanFfi.h
.PHONY: cbindgen

rust-coverage:
	# Expects a Rust nightly toolchain to be available.
	# Expects grcov and genhtml to be available in $PATH.
	CARGO_INCREMENTAL=0 \
	RUSTFLAGS='-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads' \
	RUSTUP_TOOLCHAIN=nightly \
		cargo build
	zip -0 ccov.zip `find . \( -name "glean*.gc*" \) -print`
	grcov ccov.zip -s . -t lcov --llvm --branch --ignore-not-existing --ignore-dir "/*" --ignore-dir "glean-core/ffi/*" -o lcov.info
	genhtml -o report/ --show-details --highlight --ignore-errors source --legend lcov.info
.PHONY: rust-coverage
