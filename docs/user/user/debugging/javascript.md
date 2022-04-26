# Debugging JavaScript applications using Glean.js

Debugging features in JavaScript can be enabled through APIs exposed on the Glean object.
For more information on the available features and how to enable them,
see the [Debugging API reference](../../reference/debug/index.md).

## Sending pings

Unlike other platforms, JavaScript doesn't expose convenience methods to send pings on demand.

In case that is necessary, calling the `submit` function for a given ping,
such as `pings.customPing.submit()`, will send it.

Note that this method is only effective for custom pings.
Glean internal pings are not exposed to users.

## Logging pings

By calling `Glean.logPings(true)` all subsequent pings sent will be logged to the console.

To access the logs for web extensions on

### Firefox

1. Go to `about:debugging#/runtime/this-firefox`;
2. Find the extension you want to see the logs for;
3. Click on `Inspect`.

### Chromium-based browsers

1. Go to `chrome://extensions`;
2. Find the extension you want to see the logs for;
3. Click on `background page`.
