#!/bin/bash

if ! command -v wine64 >/dev/null; then
  echo "wine64 required."
  echo "Use your package manager to install it."
  exit 1
fi

if ! python3 --version | grep -q 3.8; then
  echo "Python 3.8 required."
  echo "Use pyenv or your package manager to install it."
  exit 1
fi

if ! command -v x86_64-w64-mingw32-gcc >/dev/null; then
  echo "x86_64-w64-mingw32-gcc not found."
  echo "Install mingw64 using your package manger."
  exit 1
fi

set -e # exit on failure
set -x # show all commands

make setup-python
pushd glean-core/python
GLEAN_BUILD_TARGET=x86_64-pc-windows-gnu \
GLEAN_BUILD_VARIANT=release \
  .venv3.8/bin/python3 setup.py bdist_wheel
popd

export CARGO_TARGET_X86_64_PC_WINDOWS_GNU_LINKER="$(command -v x86_64-w64-mingw32-gcc)"
export WINPYTHON="wine64 winpython/python.exe"
export WINEDEBUG=-all

if [ ! -d "winpython" ]; then
  mkdir winpython

  wget https://www.python.org/ftp/python/3.8.2/python-3.8.2-embed-amd64.zip -O winpython/python-3.8.2-embed-amd64.zip
  unzip winpython/python-3.8.2-embed-amd64.zip -d winpython
fi

if [ ! -f "winpython/Scripts/pip.exe" ]; then
  wget https://bootstrap.pypa.io/get-pip.py -O winpython/get-pip.py
  $WINPYTHON winpython/get-pip.py
  echo "import site" >> winpython/python38._pth
  echo "import sys; sys.path.insert(0, '')" >> winpython/sitecustomize.py
fi

# The Windows-Python-installed-inside-Wine thing can't actually build wheels,
# so just install all of the wheels that were created as part of creating the
# environment on the host system before attempting to install everything in
# requirements_dev.txt
find ~/.cache/pip -name "*win_amd64.whl" -exec $WINPYTHON -m pip install {} \;
$WINPYTHON -m pip install -r glean-core/python/requirements_dev.txt --no-warn-script-location
$WINPYTHON -m pip install glean-core/python/dist/*win_amd64.whl --no-warn-script-location

# run tests
$WINPYTHON -m pytest -s glean-core/python/tests
