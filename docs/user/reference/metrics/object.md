# Object

Record structured data.

## Recording API

### `set`

Sets an object metric to a specific value.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Party

var balloons = Party.BalloonsObject()
balloons.add(Party.BalloonsObjectItem(colour = "red", diameter = 5))
balloons.add(Party.BalloonsObjectItem(colour = "green"))
Party.balloons.set(balloons)
```

</div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab">

```Swift
var balloons: Party.BalloonsObject = []
balloons.append(Party.BalloonsObjectItem(colour: "red", diameter: 5))
balloons.append(Party.BalloonsObjectItem(colour: "green"))
Party.balloons.set(balloons)
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

balloons = metrics.BalloonsObject()
balloons.append(BalloonsObjectItem(colour="red", diameter=5))
balloons.append(BalloonsObjectItem(colour="green"))
metrics.party.balloons.set(balloons)
```

</div>

<div data-lang="Rust" class="tab"></div>

<div data-lang="JavaScript" class="tab"></div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

Not yet implemented.

**JavaScript**

```js
let balloons = [
  { colour: "red", diameter: 5 },
  { colour: "blue", diameter: 7 },
];
Glean.party.balloons.set(balloons);
```

</div>

{{#include ../../../shared/tab_footer.md}}

#### Limits

* Only objects matching the specified structure will be recorded

#### Recorded errors

* [`invalid_value`](../../user/metrics/error-reporting.md): if the passed value doesn't match the predefined structure

## Testing API

### `testGetValue`

Gets the recorded value for a given object metric.  
Returns the data as a JSON object if data is stored.  
Returns `null` if no data is stored.
Has an optional argument to specify the name of the ping you wish to retrieve data from, except
in Rust where it's required. `None` or no argument will default to the first value found for `send_in_pings`.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Party

val snapshot = metric.testGetValue()!!
assertEquals(1, snapshot.jsonArray.size)
```

</div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab">

```Swift
let snapshot = (try! Party.balloons.testGetValue()) as! [Any]
XCTAssertEqual(1, snapshot.size)
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

snapshot = metrics.party.balloons.test_get_value()
assert 2 == len(snapshot)
```

</div>

<div data-lang="Rust" class="tab"></div>

<div data-lang="JavaScript" class="tab"></div>

<div data-lang="Firefox Desktop" class="tab">

**C++**

Not yet implemented.

**JavaScript**

```js
// testGetValue will throw a data error on invalid value.
Assert.equal(
  [
    { colour: "red", diameter: 5 },
    { colour: "blue", diameter: 7 },
  ],
  Glean.party.balloons.testGetValue()
);
```

</div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets the number of errors recorded for a given text metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Party

assertEquals(
    0,
    Party.balloons.testGetNumRecordedErrors(ErrorType.INVALID_VALUE)
)
```

</div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab">

```Swift
XCTAssertEqual(0, Party.balloons.testGetNumRecordedErrors(.invalidValue))
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
from glean.testing import ErrorType
metrics = load_metrics("metrics.yaml")

assert 0 == metrics.party.balloons.test_get_num_recorded_errors(
    ErrorType.INVALID_VALUE
)
```

</div>

<div data-lang="Rust" class="tab"></div>

<div data-lang="JavaScript" class="tab"></div>

<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

## Metric parameters

The definition for an object metric type accepts a `structure` parameter.
This defines the accepted structure of the object using a subset of [JSON schema](https://json-schema.org/draft/2020-12/json-schema-core).

The allowed types are:

* `string`
* `number`
* `boolean`
* `array`
* `object`

The `array` type takes an `items` parameter, that does define the element types it can hold.  
The `object` type takes a `properties` parameter, that defines the nested object structure.

`array` and `object` metrics can be nested.  
No other schema parameters are allowed.  
All fields are optional.

Data is validated against this schema at recording time.  
Missing values will not be serialized into the payload.

### Example object metric definition:

```yaml
party:
  balloons:
    type: object
    description: A collection of balloons
    bugs:
      - https://bugzilla.mozilla.org/TODO
    data_reviews:
      - http://example.com/reviews
    notification_emails:
      - CHANGE-ME@example.com
    expires: never
    structure:
      type: array
      items:
        type: object
        properties:
          colour:
            type: string
          diameter:
            type: number
```

For a full reference on metrics parameters common to all metric types,
refer to the [metrics YAML registry format](../yaml/metrics.md) reference page.

## Data questions

* What is the crash stack after a Firefox main process crash?

## Reference

* [Python API docs](../../../python/glean/metrics/index.html#glean.metrics.ObjectMetricType)
* [Rust API docs](../../../docs/glean/private/struct.ObjectMetric.html)
* [Swift API docs](../../../swift/Classes/ObjectMetric.html)
