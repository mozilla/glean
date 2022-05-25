# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

"""The setup script."""

# setuptools must be imported first
from setuptools import setup, Distribution, find_packages
from setuptools.command.install import install

from distutils.command.build import build as _build
import os
import re
import shutil
import subprocess
import sys

import wheel.bdist_wheel


# Workaround for the system-installed setuptools on macOS. That version wants
# to write bytecode files to locations that violate the sandbox, with this
# message:
#
#   The package setup script has attempted to modify files on your system
#   that are not within the EasyInstall build area, and has been aborted.
#
#   This package cannot be safely installed by EasyInstall, and may not
#   support alternate installation locations even if you run its setup
#   script by hand.  Please inform the package's author and the EasyInstall
#   maintainers to find out if a fix or workaround is available.
#
# See https://bugzilla.mozilla.org/1679370
sys.dont_write_bytecode = True


if sys.version_info < (3, 6):
    print("glean requires at least Python 3.6", file=sys.stderr)
    sys.exit(1)

from pathlib import Path  # noqa

# Path to the directory containing this file
PYTHON_ROOT = Path(__file__).parent.absolute()

# Relative path to this directory from cwd.
FROM_TOP = PYTHON_ROOT.relative_to(Path.cwd())

# Path to the root of the git checkout
SRC_ROOT = PYTHON_ROOT.parents[1]

with (SRC_ROOT / "README.md").open() as readme_file:
    readme = readme_file.read()

with (SRC_ROOT / "CHANGELOG.md").open() as history_file:
    history = history_file.read()

# glean version. Automatically updated by the bin/prepare_release.sh script
version = "50.0.1"

requirements = [
    "cffi>=1.13.0",
    "glean_parser==6.0.1",
]

setup_requirements = ["cffi>=1.13.0"]

# The environment variable `GLEAN_BUILD_VARIANT` can be set to `debug` or `release`
buildvariant = os.environ.get("GLEAN_BUILD_VARIANT", "debug")


class BinaryDistribution(Distribution):
    def is_pure(self):
        return False

    def has_ext_modules(self):
        return True


def macos_compat(target):
    if target.startswith("aarch64-"):
        return "11.0"
    return "10.7"


# The logic for specifying wheel tags in setuptools/wheel is very complex, hard
# to override, and is really meant for extensions that are compiled against
# libpython.so, not this case where we have a fairly portable Rust-compiled
# binary that should work across a number of Python versions. Therefore, we
# just skip all of its logic be overriding the `get_tag` method with something
# simple that only handles the cases we need.
class bdist_wheel(wheel.bdist_wheel.bdist_wheel):
    def get_tag(self):
        cpu, _, __ = target.partition("-")
        impl, abi_tag = "cp36", "abi3"
        if "-linux" in target:
            plat_name = f"linux_{cpu}"
        elif "-darwin" in target:
            compat = macos_compat(target).replace(".", "_")
            if cpu == "aarch64":
                cpu = "arm64"
            plat_name = f"macosx_{compat}_{cpu}"
        elif "-windows" in target:
            impl, abi_tag = "py3", "none"
            if cpu == "i686":
                plat_name = "win32"
            elif cpu == "x86_64":
                plat_name = "win_amd64"
            else:
                raise ValueError("Unsupported Windows platform")
        else:
            # Keep local wheel build on BSD/etc. working
            _, __, plat_name = super().get_tag()

        return (impl, abi_tag, plat_name)


class InstallPlatlib(install):
    def finalize_options(self):
        install.finalize_options(self)
        if self.distribution.has_ext_modules():
            self.install_lib = self.install_platlib


def get_rustc_info():
    """
    Get the rustc info from `rustc --version --verbose`, parsed into a
    dictionary.
    """
    regex = re.compile(r"(?P<key>[^:]+)(: *(?P<value>\S+))")

    output = subprocess.check_output(["rustc", "--version", "--verbose"])

    data = {}
    for line in output.decode("utf-8").splitlines():
        match = regex.match(line)
        if match:
            d = match.groupdict()
            data[d["key"]] = d["value"]

    return data


target = os.environ.get("GLEAN_BUILD_TARGET")
if not target:
    target = get_rustc_info()["host"]


if "-darwin" in target:
    shared_object = "libglean_ffi.dylib"
elif "-windows" in target:
    shared_object = "glean_ffi.dll"
else:
    # Anything else must be an ELF platform - Linux, *BSD, Solaris/illumos
    shared_object = "libglean_ffi.so"


class build(_build):
    def run(self):
        try:
            # Use `check_output` to suppress output
            subprocess.check_output(["cargo"])
        except subprocess.CalledProcessError:
            print("Install Rust and Cargo through Rustup: https://rustup.rs/.")
            print(
                "Need help installing the glean_sdk? https://github.com/mozilla/glean/#contact"
            )
            sys.exit(1)

        env = os.environ.copy()

        # For `musl`-based targets (e.g. Alpine Linux), we need to set a flag
        # to produce a shared object Python extension.
        if "-musl" in target:
            env["RUSTFLAGS"] = (
                env.get("RUSTFLAGS", "") + " -C target-feature=-crt-static"
            )
        if target == "i686-pc-windows-gnu":
            env["RUSTFLAGS"] = env.get("RUSTFLAGS", "") + " -C panic=abort"

        command = [
            "cargo",
            "build",
            "--package",
            "glean-bundle",
            "--target",
            target,
            "--features",
            "rkv-safe-mode",
        ]
        if buildvariant != "debug":
            command.append(f"--{buildvariant}")

        if "-darwin" in target:
            env["MACOSX_DEPLOYMENT_TARGET"] = macos_compat(target)

        subprocess.check_call(command, cwd=SRC_ROOT / "glean-core", env=env)
        shutil.copyfile(
            SRC_ROOT / "target" / target / buildvariant / shared_object,
            PYTHON_ROOT / "glean" / shared_object,
        )

        command = [
            "cargo",
            "uniffi-bindgen",
            "generate",
            "glean-core/src/glean.udl",
            "--language",
            "python",
            "--out-dir",
            SRC_ROOT / "target",
        ]
        subprocess.check_call(command, cwd=SRC_ROOT, env=env)
        shutil.copyfile(
            SRC_ROOT / "target" / "glean.py", PYTHON_ROOT / "glean" / "_uniffi.py"
        )

        shutil.copyfile(
            PYTHON_ROOT.parent / "metrics.yaml", PYTHON_ROOT / "glean" / "metrics.yaml"
        )
        shutil.copyfile(
            PYTHON_ROOT.parent / "pings.yaml", PYTHON_ROOT / "glean" / "pings.yaml"
        )

        return _build.run(self)


setup(
    author="The Glean Team",
    author_email="glean-team@mozilla.com",
    classifiers=[
        "Intended Audience :: Developers",
        "Natural Language :: English",
        "Programming Language :: Python :: 3",
        "Programming Language :: Python :: 3.6",
        "Programming Language :: Python :: 3.7",
        "Programming Language :: Python :: 3.8",
        "Programming Language :: Python :: 3.9",
    ],
    description="Mozilla's Glean Telemetry SDK: The Machine that Goes 'Ping!'",
    install_requires=requirements,
    long_description=readme + "\n\n" + history,
    long_description_content_type="text/markdown",
    include_package_data=True,
    keywords="glean",
    name="glean-sdk",
    version=version,
    packages=[
        "glean",
        "glean._subprocess",
        "glean.metrics",
        "glean.net",
        "glean.testing",
    ],
    package_dir={
        "glean": FROM_TOP / "glean",
        "glean._subprocess": FROM_TOP / "glean" / "_subprocess",
        "glean.metrics": FROM_TOP / "glean" / "metrics",
        "glean.net": FROM_TOP / "glean" / "net",
        "glean.testing": FROM_TOP / "glean" / "testing",
    },
    setup_requires=[],
    url="https://github.com/mozilla/glean",
    zip_safe=False,
    package_data={"glean": [shared_object, "metrics.yaml", "pings.yaml"]},
    distclass=BinaryDistribution,
    cmdclass={"install": InstallPlatlib, "bdist_wheel": bdist_wheel, "build": build},
)
