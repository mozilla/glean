# Labeled metrics

Some metrics can be used as *labeled* variants. This means that for a single
metric entry you define in `metrics.yaml`, you can record into multiple metrics
under the same name, each identified by a different string label.

This is useful when you need to break down metrics by a label known at build time or run time. For example:
- When you want to count a different set of subviews that users interact with, you could use `viewCount["view1"].add()` and `viewCount["view2"].add()`.
- When you want to count errors that might occur for a feature, you could use `errorCount[errorName].add()`.

**Note**: Be careful with using arbitrary strings as labels and make sure they can't accidentally contain identifying data (like directory paths or user input).

All metric types except events have labeled variants.  For example, for a labeled counter, use `type: labeled_counter`.

Say you're adding a new counter for errors that can occur when loading a resource from a REST API. First you need to add an entry for the counter to the `metrics.yaml` file:

```YAML
updater:
  load_error:
    type: labeled_counter
    labels: # This is optional, if provided it limits the set of labels you can use.
    - timeout
    - not_found
    description: >
      Counts the different types of load errors that can occur.
    ...
```

Now you can use the labeled counter from the applications code:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Updater

Updater.loadError["timeout"].add() // Adds 1 to the "timeout" counter.
Updater.loadError["not_found"].add(2)
```

There are test APIs available too:

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Updater
Glean.enableTestingMode()

// Was anything recorded?
assertTrue(Updater.loadError["timeout"].testHasValue())
assertTrue(Updater.loadError["not_found"].testHasValue())
// Does the counter have the expected value?
assertEquals(2, Updater.loadError["not_found"].testGetValue())
```
