# Common code used by the automation python scripts.

# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

import os
import subprocess
from pathlib import Path


def step_msg(msg):
    print(f"> \033[34m{msg}\033[0m")

def fatal_err(msg):
    print(f"\033[31mError: {msg}\033[0m")
    exit(1)

def run_cmd_checked(*args, **kwargs):
    """Run a command, throwing an exception if it exits with non-zero status."""
    kwargs["check"] = True
    return subprocess.run(*args, **kwargs)

def ensure_working_tree_clean():
    """Error out if there are un-committed or staged files in the working tree."""
    if run_cmd_checked(["git", "status", "--porcelain"], capture_output=True).stdout:
        fatal_err("The working tree has un-commited or staged files.")

def find_glean_root():
    """Find the absolute path of the Glean repository root."""
    cur_dir = Path(__file__).parent
    while not Path(cur_dir, "LICENSE").exists():
        cur_dir = cur_dir.parent
    return cur_dir.absolute()

def set_gradle_substitution_path(project_dir, name, value):
    """Set a substitution path property in a gradle `local.properties` file.

    Given the path to a gradle project directory, this helper will set the named
    property to the given path value in that directory's `local.properties` file.
    If the named property already exists with the correct value then it will
    silently succeed; if the named property already exists with a different value
    then it will noisily fail.
    """
    project_dir = Path(project_dir).resolve()
    properties_file = project_dir / "local.properties"
    step_msg(f"Configuring local publication in project at {properties_file}")
    name_eq = name + "="
    abs_value = Path(value).resolve()
    # Check if the named property already exists.
    if properties_file.exists():
        with properties_file.open() as f:
            for ln in f:
                # Not exactly a thorough parser, but should be good enough...
                if ln.startswith(name_eq):
                    cur_value = ln[len(name_eq):].strip()
                    if Path(project_dir, cur_value).resolve() != abs_value:
                        fatal_error(f"Conflicting property {name}={cur_value} (not {abs_value})")
                    return
    # The file does not contain the required property, append it.
    # Note that the project probably expects a path relative to the project root.
    ancestor = Path(os.path.commonpath([project_dir, abs_value]))
    relpath = Path(".")
    for _ in project_dir.parts[len(ancestor.parts):]:
        relpath /= ".."
    for nm in abs_value.parts[len(ancestor.parts):]:
        relpath /= nm
    step_msg(f"Setting relative path from {project_dir} to {abs_value} as {relpath}")
    with properties_file.open("a") as f:
        f.write(f"{name}={relpath}\n")
