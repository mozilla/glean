# Object

Record structured data.

{{#include ../../../shared/blockquote-warning.html}}

## Recording API

### `set`

Sets an object metric to a specific value.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab"></div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab"></div>

<div data-lang="Python" class="tab"></div>

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
Glean.testOnly.balloons.set(balloons);
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

<div data-lang="Kotlin" class="tab"></div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab"></div>

<div data-lang="Python" class="tab"></div>

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
  Glean.testOnly.balloons.testGetValue()
);
```

</div>

{{#include ../../../shared/tab_footer.md}}

### `testGetNumRecordedErrors`

Gets the number of errors recorded for a given text metric.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab"></div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab"></div>

<div data-lang="Python" class="tab"></div>

<div data-lang="Rust" class="tab"></div>

<div data-lang="JavaScript" class="tab"></div>

<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../../shared/tab_footer.md}}

## Metric parameters

Example text metric definition:

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

