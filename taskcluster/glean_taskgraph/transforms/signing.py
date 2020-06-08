# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from __future__ import absolute_import, print_function, unicode_literals

from taskgraph.transforms.base import TransformSequence

from . import publications_to_artifact_paths
from ..build_config import get_version

transforms = TransformSequence()


@transforms.add
def build_upstream_artifacts(config, tasks):
    for task in tasks:
        dep = task.pop("primary-dependency")
        module_info = task["attributes"]["buildconfig"]
        name = module_info["name"]
        version = get_version()

        worker_definition = {"upstream-artifacts": [{
            "taskId": {"task-reference": "<{}>".format(dep.kind)},
            "taskType": "build",
            "paths": publications_to_artifact_paths(name, version, module_info["publications"]),
            "formats": ["autograph_gpg"],
        }]}

        task.setdefault("worker", {})
        task["worker"].update(worker_definition)
        yield task


@transforms.add
def signing_task(config, tasks):
    for task in tasks:
        task["worker"]["signing-type"] = 'dep-signing' if config.params["level"] != u'3' else 'release-signing'
        yield task


@transforms.add
def remove_dependent_tasks(config, tasks):
    for task in tasks:
        del task["dependent-tasks"]
        yield task
