# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""
The main entry point for work performed on a worker process by
_dispatcher_subprocess.

This needs to be here and not at the top-level of the package to avoid
ambiguity between the `glean` and `glean.glean` import paths.
"""


if __name__ == "__main__":  # pragma: no cover
    import base64
    import logging
    import os
    import pickle
    import sys

    # Run coverage in the subprocess if necessary
    if "GLEAN_COVERAGE" in os.environ and "COVERAGE_PROCESS_START" in os.environ:
        import coverage  # type: ignore

        config_path = os.environ.get("COVERAGE_PROCESS_START")

        cov = coverage.Coverage(data_suffix=True, config_file=config_path)
        cov.start()
        cov._warn_no_data = False
        cov._warn_unimported_source = False
        cov._auto_save = True

    __builtins__.IN_GLEAN_SUBPROCESS = True  # type: ignore

    simple_log_level, func, args = pickle.loads(base64.b64decode(sys.argv[1]))

    if simple_log_level is not None:
        logging.basicConfig(level=simple_log_level)

    success = func(*args)

    if success:
        sys.exit(0)
    else:
        sys.exit(1)
