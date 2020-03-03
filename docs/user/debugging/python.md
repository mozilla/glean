# Debugging Python applications using the Glean SDK

Glean provides a couple of configuration flags to assist with debugging Python applications.

## Tagging pings

The `Glean.configuration.ping_tag` property can be used to add a special flag to the HTTP header so that the ping will end up in the [Glean Debug View](https://docs.telemetry.mozilla.org/concepts/glean/debug_ping_view.html).

You can set it after `Glean.initialize` is called:

```py
from Glean import Glean, Configuration
Glean.initialize(
    application_id="my-app-id",
    application_version="0.1.0",
    upload_enabled=True,
)

# ...

Glean.configuration.ping_tag = "my-ping-tag"
```

After doing so, something like `pings.custom_ping.submit()` will send the custom ping to the Glean Debug View.

## Logging pings

If the `Glean.configuration.log_pings` property is set to `True`, pings are
logged to the console on `DEBUG` level whenever they are submitted. You can set
this property in a similar way as the `ping_tag` property above.

Make sure that when you configure logging in your application, you set the
level for the `glean` logger to `DEBUG` or higher. Otherwise pings won't be
logged even if `log_pings` is set to `True`.

See the [Python logging documentation][python-logging] for more information.

[python-logging]: https://docs.python.org/3.8/library/logging.html

