# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.


import atexit
import json
import logging
from pathlib import Path
import platform
import shutil
import tempfile
from typing import Dict, List, Optional, Set, TYPE_CHECKING


from .config import Configuration
from ._dispatcher import Dispatcher
from . import _ffi
from . import hardware
from .net import PingUploadWorker
from .net import DeletionPingUploadWorker
from . import util


# To avoid cyclical imports, but still make mypy type-checking work.
# See https://mypy.readthedocs.io/en/latest/common_issues.html#import-cycles
if TYPE_CHECKING:
    from .metrics import PingType, RecordedExperimentData


log = logging.getLogger(__name__)


class Glean:
    """
    The main Glean API.

    Before any data collection can take place, the Glean SDK **must** be
    initialized from the application.

    >>> Glean.initialize(application_id="my-app", application_version="0.0.0", upload_enabled=True)
    """

    # Whether Glean was initialized
    _initialized = False  # type: bool

    # The Configuration that was passed to `initialize`
    _configuration = None  # type: Configuration

    # The directory that Glean stores data in
    _data_dir = Path()  # type: Path

    # Whether Glean "owns" the data directory and should destroy it upon reset.
    _destroy_data_dir = False  # type: bool

    # Keep track of this setting before Glean is initialized
    _upload_enabled = True  # type: bool

    # The ping types, so they can be registered prior to Glean initialization,
    # and saved between test runs.
    _ping_type_queue = set()  # type: Set[PingType]

    # The application id to send in the ping.
    _application_id = None  # type: str

    # The version of the application sending Glean data.
    _application_version = None  # type: str

    @classmethod
    def initialize(
        cls,
        application_id: str,
        application_version: str,
        upload_enabled: bool,
        configuration: Optional[Configuration] = None,
        data_dir: Optional[Path] = None,
    ):
        """
        Initialize the Glean SDK.

        This should only be initialized once by the application, and not by
        libraries using the Glean SDK. A message is logged to error and no
        changes are made to the state if initialize is called a more than
        once.

        Args:
            application_id (str): The application id to use when sending pings.
            application_version (str): The version of the application sending
                Glean data.
            upload_enabled (bool): Controls whether telemetry is enabled. If
                disabled, all persisted metrics, events and queued pings
                (except first_run_date) are cleared.
            configuration (glean.config.Configuration): (optional) An object with
                global settings.
            data_dir (pathlib.Path): (optional) The path to the Glean data
                directory. If not provided, uses a temporary directory.
        """
        if cls.is_initialized():
            return

        if configuration is None:
            configuration = Configuration()

        if data_dir is None:
            data_dir = Path(tempfile.TemporaryDirectory().name)
            cls._destroy_data_dir = True
        else:
            cls._destroy_data_dir = False
        cls._data_dir = data_dir

        cls._configuration = configuration
        cls._application_id = application_id
        cls._application_version = application_version

        cls._upload_enabled = upload_enabled

        cfg = _ffi.make_config(
            cls._data_dir,
            application_id,
            cls._upload_enabled,
            configuration.max_events,
        )

        cls._initialized = _ffi.lib.glean_initialize(cfg) != 0

        # If initialization of Glean fails, we bail out and don't initialize
        # further
        if not cls._initialized:
            return

        for ping in cls._ping_type_queue:
            cls.register_ping_type(ping)

        # Initialize the core metrics
        cls._initialize_core_metrics()

        # Glean Android sets up the metrics ping scheduler here, but we don't
        # have one.

        # Deal with any pending events so we can start recording new ones
        @Dispatcher.launch_at_front
        def submit_pending_events():
            if _ffi.lib.glean_on_ready_to_submit_pings():
                PingUploadWorker.process()

        Dispatcher.flush_queued_initial_tasks()

        # Glean Android sets up the lifecycle observer here. We don't really
        # have a lifecycle.

        if cls._upload_enabled is False:

            @Dispatcher.launch
            def check_pending_deletion_request():
                DeletionPingUploadWorker.process()

    @classmethod
    def reset(cls):
        """
        Resets the Glean singleton.
        """
        # TODO: 1594184 Send the metrics ping
        Dispatcher.reset()
        if cls._initialized:
            _ffi.lib.glean_destroy_glean()
        cls._initialized = False
        if cls._destroy_data_dir and cls._data_dir.exists():
            shutil.rmtree(str(cls._data_dir))

    @classmethod
    def is_initialized(cls) -> bool:
        """
        Returns True if the Glean SDK has been initialized.
        """
        return cls._initialized

    @classmethod
    def register_ping_type(cls, ping: "PingType"):
        """
        Register the ping type in the registry.
        """
        if cls.is_initialized():
            _ffi.lib.glean_register_ping_type(ping._handle)

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
            _ffi.lib.glean_test_has_ping_type(_ffi.ffi_encode_string(ping_name))
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
                _ffi.lib.glean_set_upload_enabled(enabled)

                if original_enabled is False and cls.get_upload_enabled() is True:
                    cls._initialize_core_metrics()

                if original_enabled is True and cls.get_upload_enabled() is False:
                    # If uploading is disabled, we need to send the deletion-request ping
                    DeletionPingUploadWorker.process()

        else:
            cls._upload_enabled = enabled

    @classmethod
    def get_upload_enabled(cls) -> bool:
        """
        Get whether or not Glean is allowed to record and upload data.
        """
        if cls.is_initialized():
            return bool(_ffi.lib.glean_is_upload_enabled())
        else:
            return cls._upload_enabled

    @classmethod
    def set_experiment_active(
        cls, experiment_id: str, branch: str, extra: Optional[Dict[str, str]] = None
    ):
        """
        Indicate that an experiment is running. Glean will then add an
        experiment annotation to the environment which is sent with pings. This
        information is not persisted between runs.

        Args:
            experiment_id (str): The id of the active experiment (maximum 100
                bytes)
            branch (str): The experiment branch (maximum 100 bytes)
            extra (dict of str -> str): Optional metadata to output with the
                ping
        """
        if extra is None:
            keys = []  # type: List[str]
            values = []  # type: List[str]
        else:
            keys, values = zip(*extra.items())  # type: ignore

        @Dispatcher.launch
        def set_experiment_active():
            _ffi.lib.glean_set_experiment_active(
                _ffi.ffi_encode_string(experiment_id),
                _ffi.ffi_encode_string(branch),
                _ffi.ffi_encode_vec_string(keys),
                _ffi.ffi_encode_vec_string(values),
                len(keys),
            )

    @classmethod
    def set_experiment_inactive(cls, experiment_id: str):
        """
        Indicate that the experiment is no longer running.

        Args:
            experiment_id (str): The id of the experiment to deactivate.
        """

        @Dispatcher.launch
        def set_experiment_inactive():
            _ffi.lib.glean_set_experiment_inactive(
                _ffi.ffi_encode_string(experiment_id)
            )

    @classmethod
    def test_is_experiment_active(cls, experiment_id: str) -> bool:
        """
        Tests whether an experiment is active, for testing purposes only.

        Args:
            experiment_id (str): The id of the experiment to look for.

        Returns:
            is_active (bool): If the experiement is active and reported in
                pings.
        """
        return bool(
            _ffi.lib.glean_experiment_test_is_active(
                _ffi.ffi_encode_string(experiment_id)
            )
        )

    @classmethod
    def test_get_experiment_data(cls, experiment_id: str) -> "RecordedExperimentData":
        """
        Returns the stored data for the requested active experiment, for testing purposes only.

        Args:
            experiment_id (str): The id of the experiment to look for.

        Returns:
            experiment_data (RecordedExperimentData): The data associated with
                the experiment.
        """
        from .metrics import RecordedExperimentData

        json_string = _ffi.ffi_decode_string(
            _ffi.lib.glean_experiment_test_get_data(
                _ffi.ffi_encode_string(experiment_id)
            )
        )

        json_tree = json.loads(json_string)

        return RecordedExperimentData(**json_tree)  # type: ignore

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
        metrics.glean.internal.metrics.locale.set(util.get_locale_tag())

        sysinfo = hardware.get_system_information()
        metrics.glean.internal.metrics.device_manufacturer.set(sysinfo.manufacturer)
        metrics.glean.internal.metrics.device_model.set(sysinfo.model)

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
    def test_collect(cls, ping: "PingType", reason: Optional[str] = None) -> str:
        """
        Collect a ping and return as a string.

        Args:
            ping: The PingType to submit
            reason (str, optional): The reason code to record in the ping.
        """
        return _ffi.ffi_decode_string(
            _ffi.lib.glean_ping_collect(ping._handle, _ffi.ffi_encode_string(reason))
        )

    @classmethod
    def _submit_ping(cls, ping: "PingType", reason: Optional[str] = None):
        """
        Collect and submit a ping for eventual uploading.

        If the ping currently contains no content, it will not be assembled and
        queued for sending.

        Args:
            ping (PingType): Ping to submit.
            reason (str, optional): The reason the ping was submitted.
        """
        cls._submit_ping_by_name(ping.name, reason)

    @classmethod
    @Dispatcher.task
    def _submit_ping_by_name(cls, ping_name: str, reason: Optional[str] = None):
        """
        Collect and submit a ping by name for eventual uploading.

        The ping will be looked up in the known instances of
        `glean.metrics.PingType`. If the ping isn't known, an error is logged
        and the ping isn't queued for uploading.

        If the ping currently contains no content, it will not be assembled and
        queued for sending.

        Args:
            ping_name (str): Ping name to submit.
            reason (str, optional): The reason code to include in the ping.
        """
        if not cls.is_initialized():
            log.error("Glean must be initialized before submitting pings.")
            return

        if not cls.get_upload_enabled():
            log.error("Glean must be enabled before submitting pings.")
            return

        sent_ping = _ffi.lib.glean_submit_ping_by_name(
            _ffi.ffi_encode_string(ping_name), _ffi.ffi_encode_string(reason),
        )

        if sent_ping:
            PingUploadWorker.process()


__all__ = ["Glean"]


atexit.register(Glean.reset)
