PYENV := .venv

.PHONY: help
help:
	@grep -E '^[0-9a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | \
	  sort | \
	  awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-30s\033[0m %s\n", $$1, $$2}'

$(PYENV)/bin/python3:
	python3 -m venv $(PYENV)
	$@ -m pip install --upgrade pip

setup: $(PYENV)/bin/python3 ## Setup a Python virtual environment
	@:

dev: setup deps ## Setup package for development
	$(PYENV)/bin/python3 -m pip install -e .

deps: ## Install required dependencies
	$(PYENV)/bin/python3 -m pip install -r requirements.txt

run: dev  ## Run the sample app
	$(PYENV)/bin/python3 -m glean-sample
