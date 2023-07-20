# This Source Code Form is subject to the terms of the Mozilla Public
# License, v. 2.0. If a copy of the MPL was not distributed with this
# file, You can obtain one at http://mozilla.org/MPL/2.0/.

import logging

from glean import Configuration, Glean, load_metrics, load_pings


config = Configuration()

Glean.initialize(
    application_id="glean-sample-app",
    application_version="0.1.0",
    upload_enabled=True,
    data_dir="./data",
    log_level=logging.DEBUG,
    configuration=config,
)

metrics = load_metrics("metrics.yaml")
pings = load_pings("pings.yaml")

metrics.test.metrics.sample_boolean.set(True)
pings.prototype.submit()

Glean.shutdown()
