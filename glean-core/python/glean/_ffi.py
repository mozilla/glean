# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

import json
import logging
import os
from pathlib import Path
import sys
import threading
from typing import Any, List, Optional
import weakref

from ._glean_ffi import ffi  # type: ignore


# The name of the language used by this Glean binding.
LANGUAGE_BINDING_NAME = "Python"


def get_shared_object_filename() -> str:  # pragma: no cover
    """
    Get the extension used for shared objects on the current platform.
    """
    if sys.platform == "darwin":
        return "libglean_ffi.dylib"
    elif sys.platform.startswith("win"):
        return "glean_ffi.dll"
    else:
        return "libglean_ffi.so"


_global_weakkeydict: Any = weakref.WeakKeyDictionary()


lib = ffi.dlopen(str(Path(__file__).parent / get_shared_object_filename()))


def setup_logging():
    """
    Sets up a pipe for communication of logging messages from the Rust core to
    the Python logging system. A thread is created to listen for messages on
    the pipe, convert them from JSON and send them to the Python stdlib logging
    module.

    Must be called after the Glean core has been dlopen'd.
    """
    r, w = os.pipe()
    lib.glean_enable_logging_to_fd(w)

    reader = os.fdopen(r, encoding="utf-8")

    log = logging.getLogger("glean")
    level_map = {
        "CRITICAL": logging.CRITICAL,
        "ERROR": logging.ERROR,
        "WARNING": logging.WARNING,
        "INFO": logging.INFO,
        "DEBUG": logging.DEBUG,
    }

    def log_handler():
        while True:
            data = reader.readline().rstrip()
            json_content = json.loads(data)
            log.log(level_map.get(json_content["level"], 0), json_content["message"])

    logging_thread = threading.Thread(target=log_handler)
    logging_thread.daemon = True
    logging_thread.start()


setup_logging()


def make_config(
    data_dir: Path,
    package_name: str,
    upload_enabled: bool,
    max_events: int,
) -> Any:
    """
    Make an `FfiConfiguration` object.

    Args:
        data_dir (pathlib.Path): Path to the Glean data directory.
        package_name (str): The name of the package to report in Glean's pings.
    """
    data_dir = ffi.new("char[]", ffi_encode_string(str(data_dir)))
    package_name = ffi.new("char[]", ffi_encode_string(package_name))
    language_binding_name = ffi.new("char[]", ffi_encode_string(LANGUAGE_BINDING_NAME))
    max_events = ffi.new("int32_t *", max_events)

    cfg = ffi.new("FfiConfiguration *")

    cfg.data_dir = data_dir
    cfg.package_name = package_name
    cfg.language_binding_name = language_binding_name
    cfg.upload_enabled = upload_enabled
    cfg.max_events = max_events
    cfg.delay_ping_lifetime_io = False

    # This ensures the ffi objects created live as long as cfg lives,
    # otherwise they get garbage collected once this function returns.
    # https://cffi.readthedocs.io/en/latest/using.html#working-with-pointers-structures-and-arrays
    _global_weakkeydict[cfg] = (
        data_dir,
        package_name,
        language_binding_name,
        max_events,
    )

    return cfg


def ffi_encode_string(value: str) -> bytes:
    """
    Convert a Python string to a UTF-8 encoded char* for sending over FFI.
    """
    return value.encode("utf-8")


def ffi_encode_string_or_none(value: Optional[str]) -> Optional[bytes]:
    """
    Convert a Python string (or None) to a UTF-8 encoded char* (or NULL) for
    sending over FFI.
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


def ffi_decode_string(cdata, free_memory=True) -> str:
    """
    Convert a string returned from Rust to a Python string, and optionally free the Rust
    string.

    Args:
        cdata: The C data containing the string value.
        free_memory (bool): Whether or not to free the memory allocated in Rust.
    """
    try:
        return ffi.string(cdata).decode("utf-8")
    finally:
        if free_memory:
            lib.glean_str_free(cdata)


def ffi_decode_byte_buffer(byte_buffer) -> bytes:
    """
    Convert a ByteBuffer returned from Rust to a Python bytes object.
    Does not free the Rust buffer.

    Args:
        byte_buffer: The byte buffer.
    """
    return ffi.buffer(byte_buffer.data, byte_buffer.len)


__all__ = [
    "ffi",
    "ffi_decode_byte_buffer",
    "ffi_decode_string",
    "ffi_encode_string",
    "ffi_encode_vec_string",
    "lib",
    "make_config",
]
