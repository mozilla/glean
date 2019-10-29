# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

import weakref

from cffi import FFI
import pkg_resources


_global_weakkeydict = weakref.WeakKeyDictionary()


def _load_header(path):
    data = pkg_resources.resource_string(__name__, path).decode("utf-8")
    return "\n".join(
        line for line in data.splitlines() if not line.startswith("#include")
    )


ffi = FFI()
ffi.cdef(_load_header("glean.h"))
lib = ffi.dlopen(pkg_resources.resource_filename(__name__, "libglean_ffi.so"))


def make_config(data_dir, package_name):
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
