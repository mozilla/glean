# Glean

The `Glean SDK` is a modern approach for a Telemetry library and is part of the [Glean project](https://docs.telemetry.mozilla.org/concepts/glean/glean.html).

## `glean`

This library provides a Rust language bindings on top of `glean-core`, targeted to Rust consumers.

## Documentation

All documentation is available online:

* [The Glean SDK Book][book]
* [API documentation][apidocs]

[book]: https://mozilla.github.io/glean/
[apidocs]: https://mozilla.github.io/glean/docs/glean/index.html

## Example

```rust,no_run
use glean::{ConfigurationBuilder, Error, metrics::*};

let cfg = ConfigurationBuilder::new(true, "/tmp/data", "org.mozilla.glean_core.example").build();
glean::initialize(cfg)?;

let prototype_ping = PingType::new("prototype", true, true, vec![]);

glean::register_ping_type(&prototype_ping);

prototype_ping.submit(None);
```

## License

    This Source Code Form is subject to the terms of the Mozilla Public
    License, v. 2.0. If a copy of the MPL was not distributed with this
    file, You can obtain one at http://mozilla.org/MPL/2.0/
