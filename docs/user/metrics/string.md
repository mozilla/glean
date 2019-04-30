# String

This allows recording a Unicode string value with arbitrary content.

---

_Note:_ This is not supporting recording JSON blobs - please get in contact with the Telemetry team if you're missing a type.

---

## Methods

* `set(value)` - Set to the specified `value`.

## Limits

* Fixed maximum string length: 20 bytes. Specified in number of Unicode characters. Longer strings are truncated.

## Examples

* Record the operating system name with a value of "android".
* Recording the device model with a value of "SAMSUNG-SGH-I997".

## Recorded errors

* `invalid_value`: if the string is too long
