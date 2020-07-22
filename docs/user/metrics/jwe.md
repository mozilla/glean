# JWE

JWE metrics are supposed to be used as a transport for [JWE](https://tools.ietf.org/html/rfc7516) encrypted data.

The encryption should happen before setting the metric and the decryption happens on the pipeline.
This means that the Glean SDK is only in charge of validating that a given input is valid JWE and
encryption and decryption are not part of its scope.

## Configuration

You first need to add an entry for it to the `metrics.yaml` file:

```YAML
user:
  anon_id:
    type: jwe
    description: >
      The JWE encrypted value of an anonymized ID.
    lifetime: user
    decrypted_name: anon_id_decrypted
    ...
```

## API

Now that the JWE is defined in `metrics.yaml`, you can use the metric to record values in the application's code.

{{#include ../../tab_header.md}}

<div data-lang="Kotlin" class="tab">

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.User

// Build a JWE from its elements and set the metric to it
User.anonId.set(
    "eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ",
    "",
    "48V1_ALb6US04U3b",
    "5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A",
    "XFBoMYUZodetZdvTiFvSkQ"
)

// Set the metric from a JWE compact representation
User.anonId.setWithCompactRepresentation("eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ..48V1_ALb6US04U3b.5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A.XFBoMYUZodetZdvTiFvSkQ")
```

There are test APIs available too.

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.User

// Was anything recorded?
assertTrue(User.anonId.testHasValue())

// Was it the expected value?

// Get a snapshot of the data
let snapshot = User.anonId.testGetValue()
assertEquals(header, snapshot.header)
assertEquals(key, snapshot.key)
assertEquals(initVector, snapshot.initVector)
assertEquals(cipherText, snapshot.cipherText)
assertEquals(authTag, snapshot.authTag)

// Or check against the compact representation
assertEquals(jwe, User.anonId.testGetCompactRepresentation())
```

</div>

<div data-lang="Java" class="tab">

```Java
import org.mozilla.yourApplication.GleanMetrics.User;

// Build a JWE from its elements and set the metric to it
User.INSTANCE.anonId.set(
    "eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ",
    "",
    "48V1_ALb6US04U3b",
    "5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A",
    "XFBoMYUZodetZdvTiFvSkQ"
);  

// Set the metric from a JWE compact representation
User.INSTANCE.anonId.setWithCompactRepresentation("eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ..48V1_ALb6US04U3b.5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A.XFBoMYUZodetZdvTiFvSkQ")
```

There are test APIs available too:

```Java
import org.mozilla.yourApplication.GleanMetrics.User;

// Was anything recorded?
assertTrue(User.INSTANCE.anonId.testHasValue());

// Was it the expected value?

// Get a snapshot of the data
JweData snapshot = User.INSTANCE.anonId.testGetValue();
assertEquals(header, snapshot.header);
assertEquals(key, snapshot.key);
assertEquals(initVector, snapshot.initVector);
assertEquals(cipherText, snapshot.cipherText);
assertEquals(authTag, snapshot.authTag);

// Or check against the compact representation
assertEquals(anonId, User.INSTANCE.anonId.testGetCompactRepresentation());
```

</div>


<div data-lang="Swift" class="tab">

```Swift
// Build a JWE from its elements and set the metric to it
User.anonId.set(
    "eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ",
    "",
    "48V1_ALb6US04U3b",
    "5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A",
    "XFBoMYUZodetZdvTiFvSkQ"
)

// Set the metric from a JWE compact representation
User.anonId.setWithCompactRepresentation("eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ..48V1_ALb6US04U3b.5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A.XFBoMYUZodetZdvTiFvSkQ")
```

There are test APIs available too.

```Swift
@testable import Glean

// Was anything recorded?
XCTAssert(User.anonId.testHasValue())

// Was it the expected value?

// Get a snapshot of the data
let snapshot = try User.anonId.testGetValue()
XCTAssertEqual(header, snapshot.header)
XCTAssertEqual(key, snapshot.key)
XCTAssertEqual(initVector, snapshot.initVector)
XCTAssertEqual(cipherText, snapshot.cipherText)
XCTAssertEqual(authTag, snapshot.authTag)

// Or check against the compact representation
XCTAssertEqual(jwe, try User.anonId.testGetCompactRepresentation())
```

</div>

<div data-lang="Python" class="tab">

```Python
from glean import load_metrics
metrics = load_metrics("metrics.yaml")

# Build a JWE from its elements and set the metric to it
metrics.user.anon_id.set(
  "eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ",
  "",
  "48V1_ALb6US04U3b",
  "5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A",
  "XFBoMYUZodetZdvTiFvSkQ"
)

# Set the metric from a JWE compact representation
metrics.user.anon_id.set_with_compact_representation("eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ..48V1_ALb6US04U3b.5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A.XFBoMYUZodetZdvTiFvSkQ")
```

There are test APIs available too.

```Python
# Was anything recorded?
assert metrics.user.anon_id.test_has_value()

# Was it the expected value?

# Get a snapshot of the data
snapshot = metrics.user.anon_id.test_get_value()
assert header == snapshot.header
assert key == snapshot.key
assert init_vector == snapshot.init_vector
assert cipher_text == snapshot.cipher_text
assert auth_tag == snapshot.auth_tag

# Or check against the compact representation
assert anon_id == metrics.user.anon_id.test_get_compact_representation()
```

</div>

<div data-lang="C#" class="tab">

```C#
using static Mozilla.YourApplication.GleanMetrics.User;

// Build a JWE from its elements and set the metric to it
User.anonId.set(
    "eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ",
    "",
    "48V1_ALb6US04U3b",
    "5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A",
    "XFBoMYUZodetZdvTiFvSkQ"
)

// Set the metric from a JWE compact representation
User.anonId.setWithCompactRepresentation("eyJhbGciOiJSU0EtT0FFUCIsImVuYyI6IkEyNTZHQ00ifQ..48V1_ALb6US04U3b.5eym8TW_c8SuK0ltJ3rpYIzOeDQz7TALvtu6UG9oMo4vpzs9tX_EFShS8iB7j6jiSdiwkIr3ajwQzaBtQD_A.XFBoMYUZodetZdvTiFvSkQ")
```

There are test APIs available too:

```C#
using Mozilla.Glean;
using static Mozilla.YourApplication.GleanMetrics.User;

// Was anything recorded?
Assert.True(User.anonId.TestHasValue());

// Was it the expected value?

// Get a snapshot of the data
Mozilla.Glean.Private.JweData snapshot = User.anonId.testGetValue()
Assert.Equals(header, snapshot.Header)
Assert.Equals(key, snapshot.Key)
Assert.Equals(initVector, snapshot.InitVector)
Assert.Equals(cipherText, snapshot.CipherText)
Assert.Equals(authTag, snapshot.AuthTag)

// Or check against the compact representation
Assert.Equals(jwe, User.anonId.testGetCompactRepresentation())
```

</div>

{{#include ../../tab_footer.md}}

## Limits

* Variable sized elements of the JWE must not exceed 1024 characters. These elements are `header`, `key` and `cipher_text`.

## Examples

* An anonymized identifier.

## Recorded errors

* `invalid_value`:
  * if the [compact representation](https://tools.ietf.org/html/rfc7516#appendix-A.2.7) passed to `set_with_compact_representation` is not valid;
  * if any one of the elements of the JWE is not valid [BASE64URL](https://tools.ietf.org/html/rfc7515#section-2);
  * if `header` or `cipher_text` elements are empty strings.
* `invalid_overflow`:
  * if `header`, `key` or `cipher_text` elements exceed 1024 characters;
  * if `init_vector` element is not empty or exactly 96-bits;
  * if `auth_tag` element is not empty or exactly 128-bits.

## Reference

* [Kotlin API docs](../../../javadoc/glean/mozilla.telemetry.glean.private/-jwe-metric-type/index.html).
* [Swift API docs](../../../swift/Classes/JweMetricType.html)
* [Python API docs](../../../python/glean/metrics/jwe.html)
