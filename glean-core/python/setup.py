# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""The setup script."""

import os
import shutil
import sys

from setuptools import setup, Distribution, find_packages
from setuptools.command.install import install
import wheel.bdist_wheel


platform = sys.platform
if os.environ.get("GLEAN_PYTHON_MINGW_BUILD") or platform.startswith("win"):
    platform = "windows"


if sys.version_info < (3, 5):
    print("glean requires at least Python 3.5", file=sys.stderr)
    sys.exit(1)

from pathlib import Path  # noqa
import toml  # noqa

ROOT = Path(__file__).parent.absolute()

os.chdir(str(ROOT))

with (ROOT.parent.parent / "README.md").open() as readme_file:
    readme = readme_file.read()

with (ROOT.parent.parent / "CHANGELOG.md").open() as history_file:
    history = history_file.read()

with (ROOT.parent / "Cargo.toml").open() as cargo:
    parsed_toml = toml.load(cargo)
    version = parsed_toml["package"]["version"]

requirements = ["cffi==1.13.1", "glean_parser==1.17.3", "inflection==0.3.1"]

setup_requirements = []

shared_object_build_dir = "../../target/debug/"
if os.environ.get("GLEAN_PYTHON_MINGW_BUILD"):
    shared_object_build_dir = "../../target/x86_64-pc-windows-gnu/debug/"


if platform == "linux":
    shared_object = "libglean_ffi.so"
elif platform == "darwin":
    shared_object = "libglean_ffi.dylib"
elif platform == "windows":
    shared_object = "glean_ffi.dll"
else:
    raise ValueError("The platform {} is not supported.".format(sys.platform))


shutil.copyfile("../ffi/glean.h", "glean/glean.h")
shutil.copyfile("../metrics.yaml", "glean/metrics.yaml")
shutil.copyfile("../pings.yaml", "glean/pings.yaml")
shutil.copyfile(shared_object_build_dir + shared_object, "glean/" + shared_object)


class BinaryDistribution(Distribution):
    def is_pure(self):
        return False

    def has_ext_modules(self):
        return True


# The logic for specifying wheel tags in setuptools/wheel is very complex, hard
# to override, and is really meant for extensions that are compiled against
# libpython.so, not this case where we have a fairly portable Rust-compiled
# binary that should work across a number of Python versions. Therefore, we
# just skip all of its logic be overriding the `get_tag` method with something
# simple that only handles the cases we need.
class bdist_wheel(wheel.bdist_wheel.bdist_wheel):
    def get_tag(self):
        if platform == "linux":
            return ("cp35", "abi3", "linux_x86_64")
        elif platform == "darwin":
            return ("cp35", "abi3", "macosx_10_7_x86_64")
        elif platform == "windows":
            return ("py3", "none", "win_amd64")


class InstallPlatlib(install):
    def finalize_options(self):
        install.finalize_options(self)
        if self.distribution.has_ext_modules():
            self.install_lib = self.install_platlib


setup(
    author="The Glean Team",
    author_email="glean-team@mozilla.com",
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
    version=version,
    packages=find_packages(include=["glean", "glean.*"]),
    setup_requires=setup_requirements,
    url="https://github.com/mozilla/glean",
    zip_safe=False,
    package_data={"glean": ["glean.h", shared_object, "metrics.yaml", "pings.yaml"]},
    distclass=BinaryDistribution,
    cmdclass={"install": InstallPlatlib, "bdist_wheel": bdist_wheel},
)
