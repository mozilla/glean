# Debugging Python applications using the Glean SDK

Debugging features in Python can be enabled using environment variables.
For more information on the available features and how to enable them,
see [Enabling debugging features through environment variables](./index.md).

## Sending pings

Unlike other platforms, Python doesn't expose convenience methods to send pings on demand.

In case that is necessary, calling the `submit` function for a given ping,
such as `pings.custom_ping.submit()`, will send it.

## Logging pings

If the `GLEAN_LOG_PINGS` environment variable is set to `true`, pings are
logged to the console on `DEBUG` level whenever they are submitted.

Make sure that when you configure logging in your application, you set the
level for the `glean` logger to `DEBUG` or higher. Otherwise pings won't be
logged even if `GLEAN_LOG_PINGS` is set to `true`.

You can set the logging level for Glean to `DEBUG` as follows:

```python
import logging

logging.getLogger("glean").setLevel(logging.DEBUG)
```

See the [Python logging documentation][python-logging] for more information.

[python-logging]: https://docs.python.org/3.8/library/logging.html

