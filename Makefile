build-apk:
	./gradlew glean-core:build
	./gradlew glean-sample-app:build
.PHONY: build-apk

install:
	$(ANDROID_HOME)/platform-tools/adb install -r ./samples/android/app/build/outputs/apk/debug/glean-sample-app-debug.apk
.PHONY: install

emulator:
	$(ANDROID_HOME)/emulator/emulator -avd Nexus_5X_API_P -netdelay none -netspeed full
.PHONY: install

clippy: fmt
	cargo clippy --all --all-targets --all-features -- -D warnings
.PHONY: lint

ktlint:
	./gradlew ktlint detekt
.PHONY: lint

lint: clippy ktlint
.PHONY: lint

swiftlint:
	swiftlint --strict
	swiftformat glean-core/ios samples/ios --swiftversion 5 --verbose --lint
.PHONY: swiftlint

fmt:
	cargo fmt --all
.PHONY: fmt

swiftfmt:
	swiftformat glean-core/ios samples/ios --swiftversion 5 --verbose
.PHONY: swiftfmt

test:
	cargo test --all
	./gradlew test
.PHONY: test

test-rust-with-logs:
	RUST_LOG=glean_core=debug cargo test --all -- --nocapture --test-threads=1
.PHONY: test-rust-with-logs

test-ios:
	./bin/run-ios-tests.sh

cbindgen:
	RUSTUP_TOOLCHAIN=nightly \
	cbindgen glean-core/ffi --lockfile Cargo.lock -o glean-core/ffi/glean.h
	cp glean-core/ffi/glean.h glean-core/ios/Glean/GleanFfi.h
.PHONY: cbindgen

docs:
	bin/build-rust-docs.sh
	./gradlew docs
.PHONY: docs

linkcheck: docs
	# Requires https://wummel.github.io/linkchecker/
	linkchecker --ignore-url javadoc --ignore-url docs/glean_core build/docs
.PHONY: linkcheck

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
