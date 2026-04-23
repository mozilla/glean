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
run:  ## Run the sample app
	cargo build
	env $(LIB_PATH_NAME)=target/debug python3 app.py
