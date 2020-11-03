# Logging

On the Glean SDK's Rust crates we use the [log](https://github.com/rust-lang/log) crate
to display log messages.

## Logging Levels

### [trace](https://docs.rs/log/0.4.11/log/macro.info.html)

- Used for logging very granular information about the flow of glean-core internals. This logging level is meant for helping with bug diagnostics and may be very noisy.
    - **Example**: `log::trace!("{} pings left in the queue (only deletion-request expected)", queue.len());`

### [debug](https://docs.rs/log/0.4.11/log/macro.debug.html)

- Used for logging messages that contain internal glean-core information. This logging level is meant for helping with bug diagnostics and may be very noisy.
    - **Example**: `log::debug!("Database path: {:?}", path.display());`

### [info](https://docs.rs/log/0.4.11/log/macro.info.html)

- Used for logging informational messages. These messages should be enough for glean-core developers and users to verify that Glean is working as expected during development.
    - **Example**: `log::info!("Ping {} successfully sent {}.", document_id, status);`

### [warn](https://docs.rs/log/0.4.11/log/macro.warn.html)

- Used for logging internal glean-core errors. These errors do not require further action of a Glean consumer, but may be informative to diagnose bugs.
    - **Example**: `log::warn!("IO error writing event to store '{}': {}", store_name, err);`

> **Note** We prefer to put such errors on the `warn` level also because otherwise they are sent to error monitoring tools, such as Sentry, and may pollute our end users dashboards.

### [error](https://docs.rs/log/0.4.11/log/macro.error.html)

- Used for logging messages that indicate wrongful usage of the Glean API.
    - **Example**: `log::error!("Unexpected 'uuid' value coming from platform code '{}'", value);`
- Used for logging messages that may be of interest to a consumer of the Glean SDK.
    - **Example**: `log::error!("Failed to submit deletion-request ping on optout: {}", err);`
