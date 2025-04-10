#### `labels`

Labeled metrics may have an optional `labels` parameter, containing a list of known labels.
The labels in this list must match the following requirements:

* Conform to the [label format](index.md#label-format).
* Each label must have a maximum of 111 characters.
* Each label must only contain printable ASCII characters.
* This list itself is limited to 4096 labels.

{{#include ../../shared/blockquote-warning.html}}

##### Important

> If the labels are specified in the `metrics.yaml`, using any label not listed in that file
> will be replaced with the special value `__other__`.
>
> If the labels are **not** specified in the `metrics.yaml`, only 16 different dynamic labels
> may be used, after which the special value `__other__` will be used.

Removing or changing labels, including their order in the registry file, is permitted. Avoid reusing labels
that were removed in the past. It is best practice to add documentation about removed labels to the
description field so that analysts will know of their existence and meaning in historical data.
Special care must be taken when changing GeckoView metrics sent through the Glean SDK, as the
index of the labels is used to report Gecko data through the Glean SDK.
