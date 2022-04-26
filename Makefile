.PHONY: help
help:
	@grep -E '^[0-9a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
	  sort | \
	  awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

GLEAN_PYENV := $(shell python3 -c "import sys; print('glean-core/python/.venv' + '.'.join(str(x) for x in sys.version_info[:2]))")
GLEAN_PYDEPS := ${GLEAN_PYDEPS}
# Read the `GLEAN_BUILD_VARIANT` variable, default to debug.
# If set it is passed as a flag to cargo, so we prefix it with `--`
ifeq ($(GLEAN_BUILD_VARIANT),)
GLEAN_BUILD_PROFILE :=
else ifeq ($(GLEAN_BUILD_VARIANT),debug)
# `--debug` is invalid and `--profile debug` is unstable.
GLEAN_BUILD_PROFILE :=
else
GLEAN_BUILD_PROFILE := --$(GLEAN_BUILD_VARIANT)
endif

# Setup environments

python-setup: $(GLEAN_PYENV)/bin/python3 ## Setup a Python virtual environment
	@:

$(GLEAN_PYENV)/bin/python3:
	python3 -m venv $(GLEAN_PYENV)
	$(GLEAN_PYENV)/bin/pip install --upgrade pip wheel setuptools
	$(GLEAN_PYENV)/bin/pip install -r glean-core/python/requirements_dev.txt
	sh -c "if [ \"$(GLEAN_PYDEPS)\" = \"min\" ]; then \
		$(GLEAN_PYENV)/bin/pip install requirements-builder; \
		$(GLEAN_PYENV)/bin/requirements-builder --level=min glean-core/python/setup.py > min_requirements.txt; \
		$(GLEAN_PYENV)/bin/pip install -r min_requirements.txt; \
	fi"

# All builds

build: build-rust

build-rust: ## Build all Rust code
	cargo build --all $(GLEAN_BUILD_PROFILE) $(addprefix --target ,$(GLEAN_BUILD_TARGET))

build-kotlin: ## Build all Kotlin code
	./gradlew build -x test

build-swift: ## Build all Swift code
	bin/run-ios-build.sh

build-apk: build-kotlin ## Build an apk of the Glean sample app
	./gradlew glean-sample-app:build glean-sample-app:assembleAndroidTest

build-python: python-setup ## Build the Python bindings
	$(GLEAN_PYENV)/bin/python3 glean-core/python/setup.py build install

.PHONY: build build-rust build-kotlin build-swift build-apk

# All tests

test: test-rust

test-rust: ## Run Rust tests for glean-core and glean-ffi
	cargo test --all $(addprefix --target ,$(GLEAN_BUILD_TARGET))

test-rust-with-logs: ## Run all Rust tests with debug logging and single-threaded
	RUST_LOG=glean_core=debug cargo test --all -- --nocapture --test-threads=1 $(addprefix --target ,$(GLEAN_BUILD_TARGET))

test-kotlin: ## Run all Kotlin tests
	./gradlew :glean:testDebugUnitTest

test-swift: ## Run all Swift tests
	bin/run-ios-tests.sh

test-android-sample: build-apk ## Run the Android UI tests on the sample app
	./gradlew :glean-sample-app:connectedAndroidTest

test-ios-sample: ## Run the iOS UI tests on the sample app
	bin/run-ios-sample-app-test.sh

test-python: build-python ## Run all Python tests
	$(GLEAN_PYENV)/bin/py.test glean-core/python/tests $(PYTEST_ARGS)

.PHONY: test test-rust test-rust-with-logs test-kotlin test-swift test-ios-sample

# Benchmarks

bench-rust: ## Run Rust benchmarks
	cargo bench -p benchmark $(addprefix --target ,$(GLEAN_BUILD_TARGET))

.PHONY: bench-rust

# Linting

lint-rust: ## Run cargo-clippy to lint Rust code
	cargo clippy --all --all-targets --all-features -- -D warnings

lint-kotlin: ## Run ktlint to lint Kotlin code
	./gradlew lint ktlint detekt

lint-swift: ## Run swiftlint to lint Swift code
	swiftlint --strict

lint-yaml: ## Run yamllint to lint YAML files
	yamllint glean-core .circleci

shellcheck: ## Run shellcheck against important shell scripts
	shellcheck glean-core/ios/sdk_generator.sh
	shellcheck bin/check-artifact.sh

lint-python: python-setup ## Run flake8 and black to lint Python code
	$(GLEAN_PYENV)/bin/python3 -m flake8 glean-core/python/glean glean-core/python/tests
	$(GLEAN_PYENV)/bin/python3 -m black --check --exclude \(.venv\*\)\|\(.eggs\) glean-core/python
	$(GLEAN_PYENV)/bin/python3 -m mypy glean-core/python/glean

.PHONY: lint-rust lint-kotlin lint-swift lint-yaml

# Formatting

fmt-rust: ## Format all Rust code
	cargo fmt --all

fmt-python: python-setup ## Run black to format Python code
	$(GLEAN_PYENV)/bin/python3 -m black glean-core/python/glean glean-core/python/tests

.PHONY: fmt-rust fmt-python

# Docs

docs: rust-docs ## Build the Rust API documentation

rust-docs: ## Build the Rust documentation
	bin/build-rust-docs.sh

swift-docs: ## Build the Swift documentation
	bin/build-swift-docs.sh

python-docs: build-python ## Build the Python documentation
	$(GLEAN_PYENV)/bin/python3 -m pdoc --html glean --force -o build/docs/python --config show_type_annotations=True

.PHONY: docs rust-docs swift-docs

metrics-docs: python-setup ## Build the internal metrics documentation
	$(GLEAN_PYENV)/bin/pip install glean_parser==5.0.1
	$(GLEAN_PYENV)/bin/glean_parser translate --allow-reserved \
		 -f markdown \
		 -o ./docs/user/user/collected-metrics \
		 glean-core/metrics.yaml glean-core/pings.yaml glean-core/android/metrics.yaml

		 cat ./docs/user/_includes/glean-js-redirect-collected-metrics.md ./docs/user/user/collected-metrics/metrics.md > ./docs/user/user/collected-metrics/metrics.tmp.md
		 mv ./docs/user/user/collected-metrics/metrics.tmp.md ./docs/user/user/collected-metrics/metrics.md

linkcheck: docs linkcheck-raw  ## Run link-checker on the generated docs

linkcheck-raw:
	# Requires https://www.npmjs.com/package/link-checker
	link-checker \
		build/docs \
    --disable-external true \
    --allow-hash-href true \
    --file-ignore "swift/.*" \
    --file-ignore "python/.*" \
    --file-ignore "javadoc/.*" \
    --file-ignore "docs/.*" \
    --url-ignore ".*/swift/.*" \
    --url-ignore ".*/python/.*" \
    --url-ignore ".*/javadoc/.*" \
    --url-ignore ".*/docs/glean_.*"
.PHONY: linkcheck linkcheck-raw

spellcheck: ## Spellcheck the docs
	# Requires http://aspell.net/
	bin/spellcheck.sh

# Utilities

android-emulator: ## Start the Android emulator with a predefined image
	$(ANDROID_HOME)/emulator/emulator -avd Nexus_5X_API_P -netdelay none -netspeed full
.PHONY: android-emulator

cbindgen: ## Regenerate the FFI header file
	RUSTUP_TOOLCHAIN=nightly \
	cbindgen \
		--config glean-core/ffi/cbindgen.toml \
		--crate glean-ffi \
		--lockfile Cargo.lock \
		--output glean-core/ffi/glean.h
	cp glean-core/ffi/glean.h glean-core/ios/Glean/GleanFfi.h
.PHONY: cbindgen

rust-coverage: export CARGO_INCREMENTAL=0
rust-coverage: export RUSTFLAGS=-Zprofile -Ccodegen-units=1 -Cinline-threshold=0 -Clink-dead-code -Coverflow-checks=off -Zno-landing-pads
rust-coverage: export RUSTUP_TOOLCHAIN=nightly
rust-coverage: ## Generate code coverage information for Rust code
	# Expects a Rust nightly toolchain to be available.
	# Expects grcov and genhtml to be available in $PATH.
	cargo build --verbose $(addprefix --target ,$(GLEAN_BUILD_TARGET))
	cargo test --verbose $(addprefix --target ,$(GLEAN_BUILD_TARGET))
	zip -0 ccov.zip `find . \( -name "glean*.gc*" \) -print`
	grcov ccov.zip -s . -t lcov --llvm --branch --ignore-not-existing --ignore "/*" --ignore "glean-core/ffi/*" -o lcov.info
	genhtml -o report/ --show-details --highlight --ignore-errors source --legend lcov.info
.PHONY: rust-coverage

python-coverage: build-python ## Generate a code coverage report for Python
	GLEAN_COVERAGE=1 $(GLEAN_PYENV)/bin/python3 -m coverage run --parallel-mode -m pytest
	$(GLEAN_PYENV)/bin/python3 -m coverage combine
	$(GLEAN_PYENV)/bin/python3 -m coverage html
.PHONY: python-coverage
