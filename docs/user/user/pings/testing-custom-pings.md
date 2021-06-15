# Testing custom pings

Applications defining [custom pings](custom.md) can use use the [ping testing API](../../reference/pings/index.md#testing-api) to test these pings in unit tests.

## General testing strategy

The schedule of custom pings depends on the specific application implementation, since it is up to the SDK user to define the ping semantics. This makes the testing strategy a bit more complex, but usually boiling down to:

1. Triggering the code path that accumulates/records the data.
2. Defining a callback validation function using the [ping testing API](../../reference/pings/index.md#testbeforenextsubmit).
3. Finally triggering the code path that submits the custom ping or submitting the ping using the [`submit` API](../../reference/pings/index.md#submit).
