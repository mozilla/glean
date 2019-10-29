# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

import sys
import weakref

from cffi import FFI
import pkg_resources


def get_shared_object_extension():
    """
    Get the extension used for shared objects on the current platform.
    """
    if sys.platform == "linux":
        return "so"
    elif sys.platform == "darwin":
        return "dylib"
    elif sys.platform.startswith("win"):
        return "dll"
    raise ValueError(f"The platform {sys.platform} is not supported.")


_global_weakkeydict = weakref.WeakKeyDictionary()


def _load_header(path):
    """
    Load a C header file and convert it to something parseable by cffi.
    """
    data = pkg_resources.resource_string(__name__, path).decode("utf-8")
    return "\n".join(
        line for line in data.splitlines() if not line.startswith("#include")
    )


ffi = FFI()
ffi.cdef(_load_header("glean.h"))
lib = ffi.dlopen(
    pkg_resources.resource_filename(
        __name__, f"libglean_ffi.{get_shared_object_extension()}"
    )
)


def make_config(data_dir, package_name):
    """
    Make an `FfiConfiguration` object.

    Args:
        data_dir (pathlib.Path): Path to the Glean data directory.
        package_name (str): The name of the package to report in Glean's pings.
    """
    data_dir = ffi.new("char[]", str(data_dir).encode("utf-8"))
    package_name = ffi.new("char[]", package_name.encode("utf-8"))
    cfg = ffi.new("FfiConfiguration *")

    cfg.data_dir = data_dir
    cfg.package_name = package_name
    cfg.upload_enabled = 1
    cfg.max_events = ffi.NULL

    _global_weakkeydict[cfg] = (data_dir, package_name)

    return cfg


__all__ = ["ffi", "lib"]
