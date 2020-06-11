# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""
The main Glean general API.
"""


import atexit
import json
import logging
from pathlib import Path
import platform
import shutil
import tempfile
import threading
from typing import Dict, List, Optional, Set, TYPE_CHECKING


from .config import Configuration
from ._dispatcher import Dispatcher
from . import _ffi
from .net import PingUploadWorker
from ._process_dispatcher import ProcessDispatcher
from . import _util


# To avoid cyclical imports, but still make mypy type-checking work.
# See https://mypy.readthedocs.io/en/latest/common_issues.html#import-cycles
if TYPE_CHECKING:
    from .metrics import PingType, RecordedExperimentData


log = logging.getLogger(__name__)


def _rmtree(path) -> bool:
    """
    A small wrapper around shutil.rmtree to make it runnable on the
    ProcessDispatcher.
    """
    shutil.rmtree(path)
    return True


class Glean:
    """
    The main Glean API.

    Before any data collection can take place, the Glean SDK **must** be
    initialized from the application.

    >>> Glean.initialize(
    ...     application_id="my-app",
    ...     application_version="0.0.0",
    ...     upload_enabled=True
    ... )
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

    # A thread lock for Glean operations that need to be synchronized
    _thread_lock = threading.RLock()

    @classmethod
    def initialize(
        cls,
        application_id: str,
        application_version: str,
        upload_enabled: bool,
        configuration: Optional[Configuration] = None,
        data_dir: Optional[Path] = None,
    ) -> None:
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
        with cls._thread_lock:
            if cls.is_initialized():
                return

            atexit.register(Glean._reset)

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

            cls.set_upload_enabled(upload_enabled)

        # Use `Glean._execute_task` rather than `Glean.launch` here, since we
        # never want to put this work on the `Dispatcher._preinit_queue`.
        @Dispatcher._execute_task
        def initialize():
            # Other platforms register the built-in pings here. That is not
            # necessary on Python since it doesn't have the problem with static
            # initializers that Kotlin and Swift have.

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

            # Kotlin bindings have a "synchronized" here, but that is
            # unnecessary given that Python has a GIL.
            with cls._thread_lock:
                for ping in cls._ping_type_queue:
                    cls.register_ping_type(ping)

            # If this is the first time ever the Glean SDK runs, make sure to set
            # some initial core metrics in case we need to generate early pings.
            # The next times we start, we would have them around already.
            is_first_run = _ffi.lib.glean_is_first_run() != 0
            if is_first_run:
                cls._initialize_core_metrics()

            # Deal with any pending events so we can start recording new ones
            if (
                _ffi.lib.glean_on_ready_to_submit_pings()
                or cls._upload_enabled is False
            ):
                PingUploadWorker.process()

            # Glean Android sets up the metrics ping scheduler here, but we don't
            # have one.

            # Other platforms check for the "dirty bit" and send the `baseline` ping
            # with reason `dirty_startup`.

            # From the second time we run, after all startup pings are generated,
            # make sure to clear `lifetime: application` metrics and set them again.
            # Any new value will be sent in newly generated pings after startup.
            if not is_first_run:
                _ffi.lib.glean_clear_application_lifetime_metrics()
                cls._initialize_core_metrics()

            Dispatcher.flush_queued_initial_tasks()

            # Glean Android sets up the lifecycle observer here. We don't really
            # have a lifecycle.

    @_util.classproperty
    def configuration(cls) -> Configuration:
        """
        Access the configuration object to change dynamic parameters.
        """
        return cls._configuration

    @classmethod
    def _reset(cls) -> None:
        """
        Resets the Glean singleton.
        """
        # TODO: 1594184 Send the metrics ping

        # WARNING: Do not run any tasks on the Dispatcher from here since this
        # is called atexit.

        # Wait for the dispatcher thread to complete.
        Dispatcher._task_worker._shutdown_thread()

        Dispatcher.reset()

        # Wait for the subprocess to complete.  We only need to do this if
        # we know we are going to be deleting the data directory.
        if cls._destroy_data_dir and cls._data_dir.exists():
            ProcessDispatcher._wait_for_last_process()

        # Destroy the Glean object.
        # Importantly on Windows, this closes the handle to the database so
        # that the data directory can be deleted without a multiple access
        # violation.
        if cls._initialized:
            _ffi.lib.glean_destroy_glean()
        cls._initialized = False

        # Remove the atexit handler or it will get called multiple times at
        # exit.
        atexit.unregister(cls._reset)

        if cls._destroy_data_dir and cls._data_dir.exists():
            # This needs to be run in the same one-at-a-time process as the
            # PingUploadWorker to avoid a race condition. This will block the
            # main thread waiting for all pending uploads to complete, but this
            # only happens during testing when the data directory is a
            # temporary directory, so there is no concern about delaying
            # application shutdown here.
            p = ProcessDispatcher.dispatch(_rmtree, (str(cls._data_dir),))
            p.wait()

    @classmethod
    def is_initialized(cls) -> bool:
        """
        Returns True if the Glean SDK has been initialized.
        """
        return cls._initialized

    @classmethod
    def register_ping_type(cls, ping: "PingType") -> None:
        """
        Register the ping type in the registry.
        """
        with cls._thread_lock:
            if cls.is_initialized():
                _ffi.lib.glean_register_ping_type(ping._handle)

            # We need to keep track of pings, so they get re-registered after a
            # reset. This state is kept across Glean resets, which should only
            # ever happen in test mode. It's a set and keeping them around
            # forever should not have much of an impact.
            cls._ping_type_queue.add(ping)

    @classmethod
    def test_has_ping_type(cls, ping_name: str) -> bool:
        """
        Returns True if a ping by this name is in the ping registry.
        """
        return bool(
            _ffi.lib.glean_test_has_ping_type(_ffi.ffi_encode_string(ping_name))
        )

    @classmethod
    def set_upload_enabled(cls, enabled: bool) -> None:
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
                    PingUploadWorker.process()

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
    ) -> None:
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
    def set_experiment_inactive(cls, experiment_id: str) -> None:
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
    def _initialize_core_metrics(cls) -> None:
        """
        Set a few metrics that will be sent as part of every ping.
        """
        from ._builtins import metrics

        metrics.glean.baseline.locale.set(_util.get_locale_tag())
        metrics.glean.internal.metrics.os_version.set(platform.release())
        metrics.glean.internal.metrics.architecture.set(platform.machine())
        metrics.glean.internal.metrics.locale.set(_util.get_locale_tag())

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
    def _submit_ping(cls, ping: "PingType", reason: Optional[str] = None) -> None:
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
    def _submit_ping_by_name(cls, ping_name: str, reason: Optional[str] = None) -> None:
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
            log.error("Glean disabled: not submitting any pings.")
            return

        sent_ping = _ffi.lib.glean_submit_ping_by_name(
            _ffi.ffi_encode_string(ping_name), _ffi.ffi_encode_string(reason),
        )

        if sent_ping:
            PingUploadWorker.process()


__all__ = ["Glean"]
