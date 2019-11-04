# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from pathlib import Path
from typing import Optional


from .config import Configuration
from . import _ffi


class Glean:
    """
    The main Glean API.

    Before any data collection can take place, the Glean SDK **must** be
    initialized from the application.

    >>> Glean.set_upload_enabled(True)
    >>> Glean.initialize(cfg, data_dir)
    """

    _handle: int = 0
    _configuration: Optional[Configuration] = None
    _data_dir: Optional[Path] = None

    @classmethod
    def initialize(cls, configuration: Configuration, data_dir: Path):
        """
        Initialize the Glean SDK.

        This should only be initialized once by the application, and not by
        libraries using the Glean SDK. A message is logged to error and no
        changes are made to the state if initialize is called a more than
        once.

        Args:
            configuration (glean.config.Configuration): An object with global
                settings.
            data_dir (pathlib.Path): The path to the Glean data directory.
        """
        if cls.is_initialized():
            return

        cls._configuration = configuration
        cls._data_dir = data_dir

        cfg = _ffi.make_config(cls._data_dir, "glean")

        cls._handle = _ffi.lib.glean_initialize(cfg)

    @classmethod
    def is_initialized(cls) -> bool:
        """
        Returns True if the Glean SDK has been initialized.
        """
        return cls._handle != 0
