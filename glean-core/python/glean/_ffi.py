# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

import json
import logging
import os
import threading


from ._uniffi import glean_enable_logging_to_fd

# Filter out UniFFI generated logs by default.
# TODO: Should this just be on the Rust side?
# It might sometimes help with debugging, but maybe this should be trace-level
FILTERED_TARGETS = ["glean_core::ffi"]


def setup_logging():
    """
    Sets up a pipe for communication of logging messages from the Rust core to
    the Python logging system. A thread is created to listen for messages on
    the pipe, convert them from JSON and send them to the Python stdlib logging
    module.

    Must be called after the Glean core has been dlopen'd.
    """
    r, w = os.pipe()
    glean_enable_logging_to_fd(w)

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
            if data:
                json_content = json.loads(data)
                target = json_content["target"]
                if target not in FILTERED_TARGETS:
                    level = level_map.get(json_content["level"], 0)
                    log.log(level, json_content["message"])

    logging_thread = threading.Thread(target=log_handler)
    logging_thread.daemon = True
    logging_thread.start()
