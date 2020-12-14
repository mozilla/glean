# Debug Pings

For debugging and testing purposes Glean allows to tag pings, which are then available in the [Debug Ping Viewer][debug-ping-viewer][^1].

Pings are sent to the same endpoint as all pings, with the addition of one HTTP header:

```
X-Debug-ID: <tag>
```

`<tag>` is a alphanumeric string with a maximum length of 20 characters, used to identify pings in the Debug Ping Viewer.

See [Debugging products using the Glean SDK](../../../user/debugging/index.md) for detailed information how to use this mechanism in applications.

[debug-ping-viewer]: https://debug-ping-preview.firebaseapp.com/

---

[^1]: Requires a Mozilla login.
