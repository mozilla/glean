# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""
The main entry point for work performed on a worker process by
_dispatcher_subprocess.

This needs to be here and not at the top-level of the package to avoid
ambiguity between the `glean` and `glean.glean` import paths.
"""


if __name__ == "__main__":
    import base64
    import pickle
    import sys

    __builtins__.IN_GLEAN_SUBPROCESS = True  # type: ignore

    func, args = pickle.loads(base64.b64decode(sys.argv[1]))

    success = func(*args)

    if success:
        sys.exit(0)
    else:
        sys.exit(1)
