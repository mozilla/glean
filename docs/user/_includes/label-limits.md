* Labels must conform to the [label format](index.md#label-format).
* Each label must have a maximum of 111 characters.
* Each label must only contain printable ASCII characters.
* The list of labels is limited to:
  * 16 different dynamic labels if no static labels are defined.
    Additional labels will all record to the special label `__other__`.
  * 100 labels if specified as static labels in `metrics.yaml`, see [Labels](#labels).
    Unknown labels will be recorded under the special label `__other__`.
