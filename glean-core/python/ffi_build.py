# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""
A helper script to build the CFFI wrappers at build time.
Run as part of the setup.py script.

This is the "out-of-line", "ABI mode" option, described in the CFFI docs here:

    https://cffi.readthedocs.io/en/latest/cdef.html
"""


from pathlib import Path


import cffi


ROOT = Path(__file__).parent.absolute()


def _load_header(path: str) -> str:
    """
    Load a C header file and convert it to something parseable by cffi.
    """
    with open(path, encoding="utf-8") as fd:
        data = fd.read()
    return "\n".join(
        line for line in data.splitlines() if not line.startswith("#include")
    )


ffibuilder = cffi.FFI()
ffibuilder.set_source("glean._glean_ffi", None)
ffibuilder.cdef(_load_header(ROOT.parent / "ffi" / "glean.h"))


if __name__ == "__main__":
    ffibuilder.compile()
