# Logging

Logs from `glean-core` are only passed through to the Android logging framework when `Glean.initialize` is called for the first time.
This means any logging that might happen before that, e.g. from early metric collection will not be collected.

If these logs are needed for debugging add the following initializer to `Glean.kt`:

```kotlin
init {
    gleanEnableLogging()
}
```
