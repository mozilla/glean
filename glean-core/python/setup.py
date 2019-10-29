# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""The setup script."""

import os
import sys

from setuptools import setup, find_packages

if sys.version_info < (3, 7):
    print("glean requires at least Python 3.7", file=sys.stderr)
    sys.exit(1)

from pathlib import Path  # noqa
import toml  # noqa

ROOT = Path(__file__).parent.absolute()

os.chdir(ROOT)

with open(ROOT.parent.parent / "README.md") as readme_file:
    readme = readme_file.read()

with open(ROOT.parent.parent / "CHANGELOG.md") as history_file:
    history = history_file.read()

with open(ROOT.parent / "Cargo.toml") as cargo:
    parsed_toml = toml.load(cargo)
    version = parsed_toml["package"]["version"]

requirements = ["cffi==1.13.1"]

setup_requirements = []

if sys.platform == "linux":
    shared_object_extension = "so"
elif sys.platform == "darwin":
    shared_object_extension = "dylib"
elif sys.platform.startswith("win"):
    shared_object_extension = "dll"
else:
    raise ValueError(f"The platform {sys.platform} is not supported.")

setup(
    author="The Glean Team",
    author_email="telemetry-client-dev@mozilla.com",
    classifiers=[
        "Intended Audience :: Developers",
        "Natural Language :: English",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
    ],
    description="Mozilla's Glean Telemetry SDK: The Machine that Goes 'Ping!'",
    install_requires=requirements,
    long_description=readme + "\n\n" + history,
    long_description_content_type="text/markdown",
    include_package_data=True,
    keywords="glean",
    name="glean-sdk",
    # While the Python bindings are still in "pre-release", let's not
    # follow the main project version.  Afterward, uncomment the line
    # below to automatically get the Rust project's version.
    version="0.0.1",
    # version=version,
    packages=find_packages(include=["glean"]),
    setup_requires=setup_requirements,
    url="https://github.com/mozilla/glean",
    zip_safe=False,
    data_files=[
        (
            "glean",
            [
                "../ffi/glean.h",
                f"../../target/debug/libglean_ffi.{shared_object_extension}",
            ],
        )
    ],
)
