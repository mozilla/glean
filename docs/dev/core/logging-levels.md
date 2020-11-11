# Logging

On the Glean SDK's Rust crates we use the [log](https://github.com/rust-lang/log) crate
to display log messages.

## Logging Levels

### [trace](https://docs.rs/log/0.4.11/log/macro.trace.html)

- Used for logging very granular information about the flow of glean-core internals. This logging level is meant for helping with bug diagnostics and may be very noisy.
    - **Example**: `log::trace!("{} pings left in the queue (only deletion-request expected)", queue.len());`

### [debug](https://docs.rs/log/0.4.11/log/macro.debug.html)

- Used for logging messages that contain internal glean-core information. This logging level is meant for helping with bug diagnostics and may be very noisy.
    - **Example**: `log::debug!("Database path: {:?}", path.display());`

### [info](https://docs.rs/log/0.4.11/log/macro.info.html)

- Used for logging informational messages. These messages should be enough for glean-core developers and users to verify that Glean is working as expected during development.
    - **Example**: `log::info!("Ping {} successfully sent {}.", document_id, status);`

### [warn](https://docs.rs/log/0.4.11/log/macro.warn.html)

- Used for logging internal glean-core warnings. These could be internal errors or warnings, that do not require further action by a Glean user, but may be informative to diagnose bugs.
    - **Example**: `log::warn!("IO error writing event to store '{}': {}", store_name, err);`

### [error](https://docs.rs/log/0.4.11/log/macro.error.html)

- Used for logging messages that indicate wrongful usage of the Glean API.
    - **Example**: `log::error!("Unexpected 'uuid' value coming from platform code '{}'", value);`
- Used for logging messages that may be of interest to a user of the Glean SDK.
    - **Example**: `log::error!("Failed to submit deletion-request ping on optout: {}", err);`

> **Note** Errors should be used sparingly. They are often surfaced in runtime error monitoring software, such as Sentry, so really should only be used when an action by a Glean user must be taken.
