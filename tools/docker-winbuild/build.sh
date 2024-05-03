#!/bin/bash

cd ~/project

make setup-python
make build-python-wheel GLEAN_BUILD_TARGET=x86_64-pc-windows-gnu

# Bit of a cleanup to reduce install time
sed \
  -e 'g/^mypy/d' \
  -e 'g/^ruff/d' \
  -e 'g/^twine/d' \
  -e 'g/^coverage/d' \
  -e 'g/^jsonschema/d' \
  -i \
  glean-core/python/requirements_dev.txt
 
find ~/.cache/pip -name "*.whl" -exec $WINPYTHON -m pip install {} \;
$WINPYTHON -m pip install -r glean-core/python/requirements_dev.txt --no-warn-script-location
$WINPYTHON -m pip install --force-reinstall target/wheels/*.whl --no-warn-script-location

$WINPYTHON -m pytest -s glean-core/python/tests
