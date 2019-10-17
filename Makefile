.PHONY: help
help:
	@grep -E '^[0-9a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
	  sort | \
	  awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

# All builds

build: build-rust

build-rust: ## Build all Rust code
	cargo build --all

build-kotlin: ## Build all Kotlin code
	./gradlew build -x test

build-swift: ## Build all Swift code
	bin/run-ios-build.sh

build-apk: ## Build an apk of the Glean sample app
	./gradlew glean-core:build
	./gradlew glean-sample-app:build

.PHONY: build build-rust build-kotlin build-swift build-apk

# All tests

test: test-rust

test-rust: ## Run all Rust tests
	cargo test --all

test-rust-with-logs: ## Run all Rust tests with debug logging and single-threaded
	RUST_LOG=glean_core=debug cargo test --all -- --nocapture --test-threads=1

test-kotlin: ## Run all Kotlin tests
	./gradlew test

test-swift: ## Run all Swift tests
	bin/run-ios-tests.sh

.PHONY: test test-rust test-rust-with-logs test-kotlin test-swift

# Linting

lint: clippy

clippy: ## Run cargo-clippy to lint Rust code
	cargo clippy --all --all-targets --all-features -- -D warnings

ktlint: ## Run ktlint to lint Kotlin code
	./gradlew ktlint detekt

swiftlint: ## Run swiftlint to lint Swift code
	swiftlint --strict

.PHONY: lint clippy ktlint swiftlint

# Formatting

fmt: rustfmt

rustfmt: ## Format all Rust code
	cargo fmt --all

swiftfmt: ## Format all Swift code
	swiftformat glean-core/ios samples/ios --swiftversion 5 --verbose

.PHONY: fmt rustfmt swiftfmt

# Docs

docs: rust-docs kotlin-docs ## Build the Rust and Kotlin API documentation

rust-docs: ## Build the Rust documentation
	bin/build-rust-docs.sh

kotlin-docs: ## Build the Kotlin documentation
	./gradlew docs

swift-docs: ## Build the Swift documentation
	bin/build-swift-docs.sh

.PHONY: docs rust-docs kotlin-docs swift-docs

linkcheck: docs ## Run linkchecker on the generated docs
	# Requires https://wummel.github.io/linkchecker/
	linkchecker --ignore-url javadoc --ignore-url docs/glean_core build/docs
.PHONY: linkcheck

# Utilities

android-emulator: ## Start the Android emulator with a predefined image
	$(ANDROID_HOME)/emulator/emulator -avd Nexus_5X_API_P -netdelay none -netspeed full
.PHONY: android-emulator

cbindgen: ## Regenerate the FFI header file
	RUSTUP_TOOLCHAIN=nightly \
	cbindgen glean-core/ffi --lockfile Cargo.lock -o glean-core/ffi/glean.h
	cp glean-core/ffi/glean.h glean-core/ios/Glean/GleanFfi.h
.PHONY: cbindgen

rust-coverage: ## Generate code coverage information for Rust code
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
