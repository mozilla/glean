# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import atexit
import logging
from pathlib import Path
import platform
import shutil
import tempfile
from typing import List, Optional, Set, TYPE_CHECKING


from .config import Configuration
from ._dispatcher import Dispatcher
from . import _ffi
from .net import PingUploadWorker
from . import util


# To avoid cyclical imports, but still make mypy type-checking work.
# See https://mypy.readthedocs.io/en/latest/common_issues.html#import-cycles
if TYPE_CHECKING:
    from .metrics import PingType


log = logging.getLogger(__name__)


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
    _configuration: Configuration

    # The directory that Glean stores data in
    _data_dir: Path = Path()

    # Whether Glean "owns" the data directory and should destroy it upon reset.
    _destroy_data_dir: bool = False

    # Keep track of this setting before Glean is initialized
    _upload_enabled: bool = True

    # The ping types, so they can be registered prior to Glean initialization,
    # and saved between test runs.
    _ping_type_queue: Set["PingType"] = set()

    # The application id to send in the ping.
    _application_id: str = "glean-python"

    # The version of the application sending Glean data.
    _application_version: str = "unknown"

    @classmethod
    def initialize(
        cls,
        configuration: Optional[Configuration] = None,
        application_id: Optional[str] = None,
        application_version: Optional[str] = None,
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
            application_version (str): (optional) The version of the application
                sending Glean data.
            data_dir (pathlib.Path): (optional) The path to the Glean data
                directory. If not provided, uses a temporary directory.
        """
        if cls.is_initialized():
            return

        if configuration is None:
            configuration = Configuration()

        if application_id is None:
            application_id = "glean-python"

        if application_version is None:
            application_version = "unknown"

        if data_dir is None:
            data_dir = Path(tempfile.TemporaryDirectory().name)
            cls._destroy_data_dir = True
        else:
            cls._destroy_data_dir = False
        cls._data_dir = data_dir

        cls._configuration = configuration
        cls._application_id = application_id
        cls._application_version = application_version

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

        for ping in cls._ping_type_queue:
            cls.register_ping_type(ping)

        # Initialize the core metrics
        cls._initialize_core_metrics()

        # Glean Android sets up the metrics ping scheduler here, but we don't
        # have one.

        # Deal with any pending events so we can start recording new ones
        @Dispatcher.launch_at_front
        def send_pending_events():
            if _ffi.lib.glean_on_ready_to_send_pings(cls._handle):
                PingUploadWorker.process()

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
    def register_ping_type(cls, ping: "PingType"):
        """
        Register the ping type in the registry.
        """
        if cls.is_initialized():
            _ffi.lib.glean_register_ping_type(cls._handle, ping._handle)

        # We need to keep track of pings, so they get re-registered after a
        # reset. This state is kept across Glean resets, which should only ever
        # happen in test mode. It's a set and keeping them around forever
        # should not have much of an impact.
        cls._ping_type_queue.add(ping)

    @classmethod
    def test_has_ping_type(cls, ping_name: str):
        """
        Returns True if a ping by this name is in the ping registry.
        """
        return bool(
            _ffi.lib.glean_test_has_ping_type(
                cls._handle, _ffi.ffi_encode_string(ping_name)
            )
        )

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
        from ._builtins import metrics

        metrics.glean.baseline.locale.set(util.get_locale_tag())
        metrics.glean.internal.metrics.os.set(platform.system())
        metrics.glean.internal.metrics.os_version.set(platform.release())
        metrics.glean.internal.metrics.architecture.set(platform.machine())

        # device_model and device_manufacturer exist on desktop platforms,
        # but aren't easily obtainable. See bug 1595751
        metrics.glean.internal.metrics.device_manufacturer.set("unknown")
        metrics.glean.internal.metrics.device_model.set("unknown")

        if cls._configuration.channel is not None:
            metrics.glean.internal.metrics.app_channel.set(cls._configuration.channel)

        metrics.glean.internal.metrics.app_build.set(cls._application_id)

        if cls._application_version is not None:
            metrics.glean.internal.metrics.app_display_version.set(
                cls._application_version
            )

    @classmethod
    def get_data_dir(cls) -> Path:
        """
        Get the data directory for Glean.
        """
        return cls._data_dir

    @classmethod
    def test_collect(cls, ping: "PingType") -> str:
        """
        Collect a ping and return as a string.
        """
        return _ffi.ffi_decode_string(
            _ffi.lib.glean_ping_collect(cls._handle, ping._handle)
        )

    @classmethod
    def _send_pings(cls, pings: List["PingType"]):
        """
        Send a list of pings.

        If the ping currently contains no content, it will not be assembled and
        queued for sending.

        Args:
            pings (list of PingType): List of pings to send.
        """
        ping_names = [ping.name for ping in pings]

        cls._send_pings_by_name(ping_names)

    @classmethod
    @Dispatcher.task
    def _send_pings_by_name(cls, ping_names: List[str]):
        """
        Send a list of pings by name.

        Each ping will be looked up in the known instances of
        `glean.metrics.PingType`. If the ping isn't known, an error is logged
        and the ping isn't queued for uploading.

        If the ping currently contains no content, it will not be assembled and
        queued for sending.

        Args:
            ping_names (list of str): List of pings to send.
        """
        if not cls.is_initialized():
            log.error("Glean must be initialized before sending pings.")
            return

        if not cls.get_upload_enabled():
            log.error("Glean must be enabled before sending pings.")
            return

        sent_ping = _ffi.lib.glean_send_pings_by_name(
            cls._handle, _ffi.ffi_encode_vec_string(ping_names), len(ping_names)
        )

        if sent_ping:
            PingUploadWorker.process()


__all__ = ["Glean"]


atexit.register(Glean.reset)
