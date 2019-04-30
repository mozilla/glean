# String List

Record a list of Unicode string values.

## Limits

* Fixed maximum string length: 20 bytes. Specified in number of Unicode characters.
* Fixed maximum list length: 20 items. Additional strings are dropped.

## Examples

The names of the enabled search engines.

## Recorded errors

* `invalid_value`: if the string is too long
* `invalid_value`: if the list is too long
