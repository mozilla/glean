# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""The setup script."""

import os
import shutil
import sys

from setuptools import setup, Distribution, find_packages
from setuptools.command.install import install


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

requirements = ["cffi==1.13.1", "glean_parser==1.15.1", "inflection==0.3.1"]

setup_requirements = []

if sys.platform == "linux":
    shared_object = "libglean_ffi.so"
elif sys.platform == "darwin":
    shared_object = "libglean_ffi.dylib"
elif sys.platform.startswith("win"):
    shared_object = "glean_ffi.dll"
else:
    raise ValueError(f"The platform {sys.platform} is not supported.")


shutil.copyfile("../ffi/glean.h", "glean/glean.h")
shutil.copyfile("../metrics.yaml", "glean/metrics.yaml")
shutil.copyfile("../pings.yaml", "glean/pings.yaml")
shutil.copyfile(f"../../target/debug/{shared_object}", f"glean/{shared_object}")


class BinaryDistribution(Distribution):
    def is_pure(self):
        return False

    def has_ext_modules(self):
        return True


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
    cmdclass={"install": InstallPlatlib},
)
