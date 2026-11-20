 UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
	LIB_PATH_NAME = LD_LIBRARY_PATH
endif
ifeq ($(UNAME_S),Darwin)
	LIB_PATH_NAME = DYLD_LIBRARY_PATH
endif

.PHONY: help
help:
	@grep -E '^[0-9a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
	  sort | \
	  awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

.PHONY: run
run: .venv/bin/python3 ## Run the sample app
	cargo build --locked --features active
	env \
		LD_LIBRARY_PATH=target/debug \
		$(LIB_PATH_NAME)=target/debug \
		RUST_LOG=uv=error,debug \
		.venv/bin/python3 -m pytest --capture=no app.py

.PHONY: run-noop
run-noop: .venv/bin/python3  ## Run the sample app, but with glean-sym in noop mode
	cargo build --locked
	env \
		GLEAN_NOOP=1 \
		LD_LIBRARY_PATH=target/debug \
		$(LIB_PATH_NAME)=target/debug \
		RUST_LOG=uv=error,debug \
		.venv/bin/python3 -m pytest --capture=no app.py

.venv/bin/python3:
	uv sync
