ifeq ($(ANDROID_HOME),)
	ANDROID_HOME := ~/Library/Android/sdk
endif

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

lint:
	cargo clippy --all
	./gradlew ktlint detekt
.PHONY: lint

fmt:
	cargo fmt --all
.PHONY: fmt

test:
	RUST_TEST_THREADS=1 cargo test --all
	./gradlew test
.PHONY: test
