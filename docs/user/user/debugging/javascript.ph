# Debugging JavaScript applications using Glean.js

Debugging features in JavaScript can be enabled through APIs exposed on the Glean object.
For more information on the available features and how to enable them,
see the [Debugging API reference](../../reference/debug/index.md).

## Debugging in the browser

Websites running Glean allow you to debug at runtime using the `window.Glean` object in the browser console. You can start debugging by simply:

1. [Opening the browser console](https://developer.mozilla.org/en-US/docs/Learn/Common_questions/Tools_and_setup/What_are_browser_developer_tools#the_javascript_console)
2. Calling one of the `window.Glean` APIs: `window.Glean.setLogPings`, `window.Glean.setDebugViewTag`, `window.Glean.setSourceTags`.

These debugging options will persist for the length of the current page session. Once the tab is closed, you will need to make those API calls again.

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
