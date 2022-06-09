# Adding a new metric type

Data in the Glean SDK is stored in so-called metrics.
You can find the full list of implemented metric types [in the user overview](../../book/user/metrics/index.md).

Adding a new metric type involves defining the metric type's API, its persisted and in-memory storage as well as its serialization into the ping payload.

## `glean_parser`

In order for your metric to be usable, you must add it to
[`glean_parser`](https://github.com/mozilla/glean_parser)
so that instances of your new metric can be instantiated and available to our users.

The documentation for how to do this should live in the `glean_parser` repository,
but in short:
* Your metric type must be added to the metrics schema.
* Your metric type must be added as a type in the object model
* Any new parameters outside of the common metric data must also be added to the schema,
  and be stored in the object model.
* You must add tests.

## The metric type's API

A metric type implementation is defined in its own file under `glean-core/src/metrics/`, e.g. `glean-core/src/metrics/counter.rs` for a [Counter](../../book/user/metrics/counter.md).

Start by defining a structure to hold the metric's metadata:

```rust,noplaypen
#[derive(Clone, Debug)]
pub struct CounterMetric {
    meta: CommonMetricData
}
```

Implement the `MetricType` trait to create a metric from the meta data as well as expose the meta data.
This also gives you a `should_record` method on the metric type.

```rust,noplaypen
impl MetricType for CounterMetric {
    fn meta(&self) -> &CommonMetricData {
        &self.meta
    }

    fn meta_mut(&mut self) -> &mut CommonMetricData {
        &mut self.meta
    }
}
```

Its implementation should have a way to create a new metric from the common metric data. It should be the same for all metric types.

```rust,noplaypen
impl CounterMetric {
    pub fn new(meta: CommonMetricData) -> Self {
        Self { meta }
    }
}
```

Implement each method for the type. The first argument to accept should always be `glean: &Glean`, that is: a reference to the `Glean` object, used to access the storage:

```rust,noplaypen
impl CounterMetric { // same block as above
    pub fn add(&self, glean: &Glean, amount: i32) {
        // Always include this check!
        if !self.should_record() {
            return;
        }

        // Do error handling here

        glean
            .storage()
            .record_with(&self.meta, |old_value| match old_value {
                Some(Metric::Counter(old_value)) => Metric::Counter(old_value + amount),
                _ => Metric::Counter(amount),
            })
    }
}
```

Use `glean.storage().record()` to record a fixed value or `glean.storage.record_with()` to construct a new value from the currently stored one.

The storage operation makes use of the metric's variant of the `Metric` enumeration.

## The `Metric` enumeration

Persistence and in-memory serialization as well as ping payload serialization are handled through the `Metric` enumeration.
This is defined in `glean-core/src/metrics/mod.rs`.
Variants of this enumeration are used in the storage implementation of the metric type.

To add a new metric type, include the metric module and declare its use, then add a new variant to the `Metric` enum:

```rust,noplaypen

mod counter;

// ...

pub use self::counter::CounterMetric;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Metric {
    // ...
    Counter(i32),
}
```

Then modify the below implementation and define the right ping section name for the new type. This will be used in the ping payload:

```rust,noplaypen
impl Metric {
    pub fn ping_section(&self) -> &'static str {
        match self {
            // ...
            Metric::Counter(_) => "counter",
        }
    }
}
```

Finally, define the ping payload serialization (as JSON).
In the simple cases where the in-memory representation maps to its JSON representation it is enough to call the `json!` macro.

```rust,noplaypen
impl Metric { // same block as above
    pub fn as_json(&self) -> JsonValue {
        match self {
            // ...
            Metric::Counter(c) => json!(c),
        }
    }
}
```

For more complex serialization consider implementing serialization logic as a function returning a [`serde_json::Value`](https://docs.rs/serde_json/*/serde_json/enum.Value.html)
or another object that can be serialized.

For example, the `DateTime` serializer has the following entry, where `get_iso_time_string` is a function to convert from the `DateTime` metric representation to a string:

```rust,noplaypen
Metric::Datetime(d, time_unit) => json!(get_iso_time_string(*d, *time_unit)),
```

## Documentation

Documentation for the new metric type must be added to the
[user book](https://mozilla.github.io/glean/book/index.html).

* Add a new file for your new metric in `docs/user/user/metrics/`.
  Its contents should follow the form and content of the other examples in that folder.
* Reference that file in `docs/user/SUMMARY.md` so it will be included in the build.
* Follow the [Documentation Contribution Guide](../docs.html).

You must also update the
[payload documentation](internal/payload.md)
with how the metric looks in the payload.

## Tests

Tests are written in the Language Bindings and tend to just cover basic functionality:
* The metric returns the correct value when it has no value
* The metric correctly reports errors
* The metric returns the correct value when it has value

---

In the next step we will create the FFI wrapper and platform-specific wrappers.
