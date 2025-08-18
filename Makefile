.PHONY: help
help:
	@grep -E '^[0-9a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
	  sort | \
	  awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

GLEAN_PYENV := $(abspath $(shell python3 -c "import sys; print('.venv' + '.'.join(str(x) for x in sys.version_info[:2]))"))
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

setup-python: $(GLEAN_PYENV)/bin/python3 ## Setup a Python virtual environment
	@:

$(GLEAN_PYENV)/bin/python3:
	python3 -m venv $(GLEAN_PYENV)
	$(GLEAN_PYENV)/bin/pip install --upgrade pip wheel setuptools
	$(GLEAN_PYENV)/bin/pip install -r glean-core/python/requirements_dev.txt

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

build-python: setup-python ## Build the Python bindings
	VIRTUAL_ENV=$(GLEAN_PYENV) \
		$(GLEAN_PYENV)/bin/maturin develop

build-python-wheel: setup-python  ## Build a Python wheel
	VIRTUAL_ENV=$(GLEAN_PYENV) \
		$(GLEAN_PYENV)/bin/maturin build --release $(addprefix --target ,$(GLEAN_BUILD_TARGET)) $(GLEAN_BUILD_EXTRA)

build-python-sdist: setup-python ## Build a Python source distribution
	VIRTUAL_ENV=$(GLEAN_PYENV) \
		$(GLEAN_PYENV)/bin/maturin build --release --sdist

build-xcframework:
	./bin/build-xcframework.sh

bindgen-python: glean-core/python/glean/_uniffi/glean.py glean-core/python/glean/_uniffi/__init__.py # Generate the uniffi wrapper code manually

glean-core/python/glean/_uniffi/glean.py: glean-core/src/glean.udl
	cargo build -p glean-bundle
	cargo uniffi-bindgen generate $< --language python --out-dir $(@D)

glean-core/python/glean/_uniffi/__init__.py:
	echo 'from .glean import *  # NOQA' > $@

.PHONY: build build-rust build-kotlin build-swift build-apk build-python build-python-wheel build-python-sdist bindgen-python build-xcframework glean-core/python/glean/_uniffi/__init__.py

# All tests

test: test-rust

test-rust: ## Run Rust tests for glean-core and glean-ffi
ifeq (, $(shell command -v cargo-nextest))
	cargo test --all $(addprefix --target ,$(GLEAN_BUILD_TARGET))
else
	cargo nextest run --all $(addprefix --target ,$(GLEAN_BUILD_TARGET))
endif

test-rust-examples: glean-core/rlb/tests/*.sh ## Run Rust example tests
	@for file in $^; do \
		echo "=== $${file} ==="; \
		./$$file || exit $?; \
	done

test-rust-with-logs: ## Run all Rust tests with debug logging and single-threaded
	RUST_LOG=glean,glean_core cargo test --all -- --nocapture --test-threads=1 $(addprefix --target ,$(GLEAN_BUILD_TARGET))

test-kotlin: ## Run all Kotlin tests
	./gradlew :glean:testDebugUnitTest

test-swift: ## Run all Swift tests
	bin/run-ios-tests.sh

test-android-sample: build-apk ## Run the Android UI tests on the sample app
	./gradlew :glean-sample-app:connectedAndroidTest

test-ios-sample: ## Run the iOS UI tests on the sample app
	bin/run-ios-sample-app-test.sh

test-python: build-python ## Run all Python tests
	$(GLEAN_PYENV)/bin/py.test -v glean-core/python/tests $(PYTEST_ARGS)

.PHONY: test test-rust test-rust-with-logs test-kotlin test-swift test-ios-sample

# Linting

lint: lint-rust lint-kotlin lint-swift lint-yaml lint-python

lint-rust: ## Run cargo-clippy to lint Rust code
	cargo clippy --all --all-targets --all-features -- -D warnings -A unknown-lints

lint-kotlin: ## Run ktlint to lint Kotlin code
	./gradlew lint ktlint detekt

lint-swift: ## Run swiftlint to lint Swift code
	swiftlint --strict

lint-yaml: ## Run yamllint to lint YAML files
	yamllint glean-core .circleci

shellcheck: ## Run shellcheck against important shell scripts
	shellcheck glean-core/ios/sdk_generator.sh
	shellcheck bin/check-artifact.sh

lint-python: setup-python ## Run ruff and mypy to lint Python code
	$(GLEAN_PYENV)/bin/python3 -m ruff format --diff glean-core/python/glean glean-core/python/tests
	$(GLEAN_PYENV)/bin/python3 -m ruff check glean-core/python/glean glean-core/python/tests
	$(GLEAN_PYENV)/bin/python3 -m mypy glean-core/python/glean

lint-python-fix: setup-python ## Run ruff and mypy to lint Python code
	$(GLEAN_PYENV)/bin/python3 -m ruff check --fix glean-core/python/glean glean-core/python/tests

.PHONY: lint-rust lint-kotlin lint-swift lint-yaml

# Formatting

fmt-rust: ## Format all Rust code
	cargo fmt --all

fmt-python: setup-python ## Run ruff to format Python code
	$(GLEAN_PYENV)/bin/python3 -m ruff format glean-core/python/glean glean-core/python/tests

fmt-kotlin:  ## Run ktlint to format KOtlin code
	./gradlew ktlintFormat

.PHONY: fmt-rust fmt-python fmt-kotlin

# Docs

docs: docs-rust ## Build the Rust API documentation

docs-rust: ## Build the Rust documentation
	bin/build-rust-docs.sh

docs-swift: ## Build the Swift documentation
	bin/build-swift-docs.sh

docs-python: build-python ## Build the Python documentation
	$(GLEAN_PYENV)/bin/python3 -m pdoc --html glean --force -o build/docs/python --config show_type_annotations=True

.PHONY: docs docs-rust docs-swift

docs-metrics: setup-python ## Build the internal metrics documentation
	$(GLEAN_PYENV)/bin/pip install glean_parser~=17.2
	$(GLEAN_PYENV)/bin/glean_parser translate --allow-reserved \
		 -f markdown \
		 -o ./docs/user/user/collected-metrics \
		 glean-core/metrics.yaml glean-core/pings.yaml glean-core/android/metrics.yaml

		 cat ./docs/user/_includes/glean-js-redirect-collected-metrics.md ./docs/user/user/collected-metrics/metrics.md > ./docs/user/user/collected-metrics/metrics.tmp.md
		 mv ./docs/user/user/collected-metrics/metrics.tmp.md ./docs/user/user/collected-metrics/metrics.md

linkcheck: docs linkcheck-raw  ## Run link-checker on the generated docs

linkcheck-raw:
	# Requires https://www.npmjs.com/package/link-checker
	npx link-checker \
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
    --url-ignore ".*/docs/glean_.*" \
    --url-ignore ".*/docs/glean/.*"
.PHONY: linkcheck linkcheck-raw

spellcheck: ## Spellcheck the docs
	# Requires http://aspell.net/
	bin/spellcheck.sh

# Utilities

android-emulator: ## Start the Android emulator with a predefined image
	$(ANDROID_HOME)/emulator/emulator -avd Nexus_5X_API_P -netdelay none -netspeed full
.PHONY: android-emulator

coverage-python: build-python ## Generate a code coverage report for Python
	GLEAN_COVERAGE=1 $(GLEAN_PYENV)/bin/python3 -m coverage run --parallel-mode -m pytest
	$(GLEAN_PYENV)/bin/python3 -m coverage combine
	$(GLEAN_PYENV)/bin/python3 -m coverage html
.PHONY: coverage-python
