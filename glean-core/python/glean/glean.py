# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from . import _ffi


class Glean:
    """
    The main Glean API.

    This is exposed through the global `Glean` object.
    """

    _handle = 0

    @classmethod
    def initialize(cls, configuration, data_dir):
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
    def is_initialized(cls):
        """
        Returns True if the Glean SDK has been initialized.
        """
        return cls._handle != 0
