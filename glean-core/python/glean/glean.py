# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


from pathlib import Path
import shutil
import tempfile
from typing import Optional


from .config import Configuration
from ._dispatcher import Dispatcher
from . import _ffi


class Glean:
    """
    The main Glean API.

    Before any data collection can take place, the Glean SDK **must** be
    initialized from the application.

    >>> Glean.set_upload_enabled(True)
    >>> Glean.initialize()
    """

    # The handle to the underlying Rust object
    _handle: int = 0

    # The Configuration that was passed to `initialize`
    _configuration: Optional[Configuration] = None

    # The directory that Glean stores data in
    _data_dir: Path = Path()

    # Whether Glean "owns" the data directory and should destroy it upon reset.
    _destroy_data_dir: bool = False

    # Keep track of this setting before Glean is initialized
    _upload_enabled: bool = True

    # TODO: 1594184 ping_type_queue

    @classmethod
    def initialize(
        cls,
        configuration: Optional[Configuration] = None,
        application_id: Optional[str] = None,
        data_dir: Optional[Path] = None,
    ):
        """
        Initialize the Glean SDK.

        This should only be initialized once by the application, and not by
        libraries using the Glean SDK. A message is logged to error and no
        changes are made to the state if initialize is called a more than
        once.

        Args:
            configuration (glean.config.Configuration): An object with global
                settings.
            application_id (str): (optional) The application id to use when
                sending pings. Defaults to 'glean-python'.
            data_dir (pathlib.Path): (optional) The path to the Glean data
                directory. If not provided, uses a temporary directory.
        """
        if cls.is_initialized():
            return

        if configuration is None:
            configuration = Configuration()

        if application_id is None:
            application_id = "glean-python"

        if data_dir is None:
            data_dir = Path(tempfile.TemporaryDirectory().name)

        cls._configuration = configuration
        cls._data_dir = data_dir

        # TODO: 1594184 cls.register_pings(pings)

        cfg = _ffi.make_config(
            cls._data_dir,
            application_id,
            cls._upload_enabled,
            configuration.max_events,
        )

        cls._handle = _ffi.lib.glean_initialize(cfg)

        # If initialization of Glean fails, we bail out and don't initialize
        # further
        if cls._handle == 0:
            return

        # TODO: 1594184 Flush the ping_type_queue

        # Initialize the core metrics
        cls._initialize_core_metrics()

        # Glean Android sets up the metrics ping scheduler here, but we don't
        # have one.

        # Deal with any pending events so we can start recording new ones
        @Dispatcher.launch_at_front
        def send_pending_events():
            if _ffi.lib.glean_on_ready_to_send_pings(cls._handle):
                # TODO: 1591192
                # PingUploadWorker.enqueueWorker()
                pass

        Dispatcher.flush_queued_initial_tasks()

        # Glean Android sets up the lifecycle observer here. We don't really
        # have a lifecycle.

    @classmethod
    def reset(cls):
        """
        Resets the Glean singleton.
        """
        # TODO: 1594184 Send the metrics ping
        if cls._handle != 0:
            _ffi.lib.glean_destroy_glean(cls._handle)
        cls._handle = 0
        if cls._destroy_data_dir and cls._data_dir.exists():
            shutil.rmtree(cls._data_dir)

    @classmethod
    def is_initialized(cls) -> bool:
        """
        Returns True if the Glean SDK has been initialized.
        """
        return cls._handle != 0

    @classmethod
    def register_pings(cls):
        """
        Register the pings generated from pings.yaml with the Glean SDK.
        """
        # TODO: 1594184
        pass

    @classmethod
    def set_upload_enabled(cls, enabled: bool):
        """
        Enable or disable Glean collection and upload.

        Metric collection is enabled by default.

        When uploading is disabled, metrics aren't recorded at all and no data
        is uploaded.

        When disabling, all pending metrics, events and queued pings are cleared.

        When enabling, the core Glean metrics are recreated.

        Args:
            enabled (bool): When True, enable metric collection.
        """
        if cls.is_initialized():
            original_enabled = cls.get_upload_enabled()

            @Dispatcher.launch
            def set_upload_enabled():
                _ffi.lib.glean_set_upload_enabled(cls._handle, enabled)

                if original_enabled is False and cls.get_upload_enabled() is True:
                    cls._initialize_core_metrics()

        else:
            cls._upload_enabled = enabled

    @classmethod
    def get_upload_enabled(cls) -> bool:
        """
        Get whether or not Glean is allowed to record and upload data.
        """
        if cls.is_initialized():
            return bool(_ffi.lib.glean_is_upload_enabled(cls._handle))
        else:
            return cls._upload_enabled

    @classmethod
    def _initialize_core_metrics(cls):
        """
        Set a few metrics that will be sent as part of every ping.
        """
        # TODO: 1594184
        pass

    @classmethod
    def get_data_dir(cls) -> Path:
        """
        Get the data directory for Glean.
        """
        return cls._data_dir
