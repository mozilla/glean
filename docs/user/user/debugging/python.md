# Debugging Python applications using the Glean SDK

Debugging features in Python can be enabled using environment variables.
For more information on the available features and how to enable them,
see the [Debugging API reference](../../reference/debug/index.md).

## Sending pings

Unlike other platforms, Python doesn't expose convenience methods to send pings on demand.

In case that is necessary, calling the `submit` function for a given ping,
such as `pings.custom_ping.submit()`, will send it.

## Logging pings

Glean offers two options for logging from Python:

- **Simple logging API:** A simple API that only allows for setting the logging level, but includes all Glean log messages, including those from its networking subprocess. This is also the only mode in which [`GLEAN_LOG_PINGS`](../../reference/debug/logPings.md) can be used to display ping contents in the log.
- **Flexible logging API:** Full use of the Python `logging` module, including its features for redirecting to files and custom handling of messages, but does not include messages from the networking subprocess about HTTP requests.

### Simple logging API

You can set the logging level for Glean log messages by passing `logging.DEBUG` to `Glean.initialize` as follows:

```python
import logging
from glean import Glean

Glean.initialize(..., log_level=logging.DEBUG)
```

If you want to see ping contents as well, set the `GLEAN_LOG_PINGS` environment variable to `true`.

### Flexible logging API

You can set the logging level for the Python logging to `DEBUG` as follows:

```python
import logging

logging.basicConfig(level=logging.DEBUG)
```

All log messages from the Glean Python SDK are on the `glean` logger, so if you need to control it independently, you can set a level for just the Glean Python SDK (but note that the global Python logging level also needs to be set as above):

```python
logging.getLogger("glean").setLevel(logging.DEBUG)
```

The flexible logging API is unable to display networking-related log messages or ping contents with `GLEAN_LOG_PINGS` set to true.

See the [Python logging documentation][python-logging] for more information.

[python-logging]: https://docs.python.org/3.8/library/logging.html

