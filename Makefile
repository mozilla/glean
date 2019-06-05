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

fmt:
	cargo fmt --all
.PHONY: fmt

test:
	cargo test --all
	./gradlew test
.PHONY: test

cbindgen:
	cbindgen glean-core/ffi --lockfile Cargo.lock -o glean-core/ffi/examples/glean.h
.PHONY: cbindgen

docs:
	bin/build-rust-docs.sh
	./gradlew docs
.PHONY: docs

linkcheck: docs
	# Requires https://wummel.github.io/linkchecker/
	linkchecker --ignore-url javadoc --ignore-url docs/glean_core build/docs
.PHONY: linkcheck
