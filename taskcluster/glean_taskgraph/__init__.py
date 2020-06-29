# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

from __future__ import absolute_import, print_function, unicode_literals

from importlib import import_module
import os

from six import text_type
from voluptuous import Required, Any

from .build_config import get_version

def register(graph_config):
    """
    Import all modules that are siblings of this one, triggering decorators in
    the process.
    """
    _import_modules([
        "job",
        "target_tasks",
        "worker_types"
    ])


def _import_modules(modules):
    for module in modules:
        import_module(".{}".format(module), package=__name__)


def get_decision_parameters(graph_config, parameters):
    parameters["head_tag"] = ''
    if parameters["tasks_for"] == "github-release":
        head_tag = os.environ.get("GLEAN_HEAD_TAG")
        if head_tag is None:
            raise ValueError("Cannot run github-release if the environment variable "
                             "'GLEAN_HEAD_TAG' is not defined")
        parameters["head_tag"] = head_tag.decode("utf-8")
        version = get_version()
        # XXX: tags are in the format of `v<semver>`
        if head_tag[1:] != version:
            raise ValueError(
                "Cannot run github-release if tag {} is different than in-tree "
                "{version} from buildconfig.yml".format(head_tag[1:], version)
            )
    elif parameters["tasks_for"] == "github-pull-request":
        pr_title = os.environ.get("GLEAN_PULL_REQUEST_TITLE", "").decode("UTF-8")
        if "[ci full]" in pr_title:
            parameters["target_tasks_method"] = "pr-full"
        elif "[ci skip]" in pr_title:
            parameters["target_tasks_method"] = "pr-skip"
        else:
            parameters["target_tasks_method"] = "pr-normal"
