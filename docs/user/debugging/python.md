# Debugging Python applications using the Glean SDK

Glean provides a couple of configuration flags to assist with debugging Python applications.

## Tagging pings

The `Glean.configuration.ping_tag` property can be used to add a special flag to the HTTP header so that the ping will end up in the [Glean Debug View](https://docs.telemetry.mozilla.org/concepts/glean/debug_ping_view.html).

For example:

```
Glean.configuration.ping_tag = "my-ping-tag"

pings.custom_ping.submit()
```

will send the custom ping to the Glean Debug View.

## Logging pings

If the `Glean.configuration.log_pings` property is set to `True`, pings are logged to the console whenever they are submitted.
