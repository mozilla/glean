# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from pathlib import Path
import pkgutil
import sys
from typing import Any, List, Optional
import weakref

from cffi import FFI  # type: ignore


def get_shared_object_filename() -> str:  # pragma: no cover
    """
    Get the extension used for shared objects on the current platform.
    """
    if sys.platform == "linux":
        return "libglean_ffi.so"
    elif sys.platform == "darwin":
        return "libglean_ffi.dylib"
    elif sys.platform.startswith("win"):
        return "glean_ffi.dll"
    raise ValueError("The platform {} is not supported.".format(sys.platform))


_global_weakkeydict = weakref.WeakKeyDictionary()  # type: Any


def _load_header(path: str) -> str:
    """
    Load a C header file and convert it to something parseable by cffi.
    """
    data = pkgutil.get_data(__name__, path)
    if data is None:  # pragma: no cover
        raise RuntimeError("Couldn't load 'glean.h'")
    data_str = data.decode("utf-8")
    return "\n".join(
        line for line in data_str.splitlines() if not line.startswith("#include")
    )


ffi = FFI()
ffi.cdef(_load_header("glean.h"))
lib = ffi.dlopen(str(Path(__file__).parent / get_shared_object_filename()))
lib.glean_enable_logging()


def make_config(
    data_dir: Path, package_name: str, upload_enabled: bool, max_events: int,
) -> Any:
    """
    Make an `FfiConfiguration` object.

    Args:
        data_dir (pathlib.Path): Path to the Glean data directory.
        package_name (str): The name of the package to report in Glean's pings.
    """
    data_dir = ffi.new("char[]", ffi_encode_string(str(data_dir)))
    package_name = ffi.new("char[]", ffi_encode_string(package_name))
    max_events = ffi.new("int32_t *", max_events)

    cfg = ffi.new("FfiConfiguration *")

    cfg.data_dir = data_dir
    cfg.package_name = package_name
    cfg.upload_enabled = upload_enabled
    cfg.max_events = max_events
    cfg.delay_ping_lifetime_io = False

    _global_weakkeydict[cfg] = (data_dir, package_name, max_events)

    return cfg


def ffi_encode_string(value: Optional[str]) -> Optional[bytes]:
    """
    Convert a Python string to a UTF-8 encoded char* for sending over FFI.
    """
    if value is None:
        return ffi.NULL
    return value.encode("utf-8")


def ffi_encode_vec_string(strings: List[str]) -> Any:
    """
    Convert a list of str in Python to a vector of char* suitable for sending over FFI.
    """
    values = [ffi.new("char[]", ffi_encode_string(x)) for x in strings]
    values.append(ffi.NULL)

    result = ffi.new("char *[]", values)

    _global_weakkeydict[result] = values

    return result


def ffi_encode_vec_int32(values: List[int]) -> Any:
    """
    Convert a list of int in Python to a vector of int32_t suitable for sending over FFI.
    """
    return ffi.new("int32_t []", values)


def ffi_decode_string(cdata) -> str:
    """
    Convert a string returned from Rust to a Python string, and free the Rust
    string.
    """
    try:
        return ffi.string(cdata).decode("utf-8")
    finally:
        lib.glean_str_free(cdata)


__all__ = [
    "ffi",
    "ffi_decode_string",
    "ffi_encode_string",
    "ffi_encode_vec_string",
    "lib",
    "make_config",
]
