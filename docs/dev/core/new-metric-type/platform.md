# Adding a new metric type - data platform

The data platform technically exists outside of the Glean SDK. However, the Glean-specific steps for adding a new Glean metric type to the data platform are documented here for convenience.

## Adding a new metric type to `mozilla-pipeline-schemas`

The [`mozilla-pipeline-schemas`](https://github.com/mozilla-services/mozilla-pipeline-schemas) contains JSON schemas that are used to validate the Glean ping payload when reaching the ingestion server.
These schemas are written using a simple custom templating system to give more structure and organization to the schemas.

Each individual metric type has its own file in `templates/include/glean`. For example, here is the schema for the Counter metric type in `counter.1.schema.json`:

```json
{
  "type": "integer"
}
```

Add a new file for your new metric type in that directory, containing the JSON schema snippet to validate it. A good resource for learning about JSON schema and the validation keywords that are available is [Understanding JSON Schema](https://json-schema.org/understanding-json-schema/).

A reference to this template needs to be added to the main Glean schema in [templates/include/glean/glean.1.schema.json](https://github.com/mozilla-services/mozilla-pipeline-schemas/blob/04043f16b319c2a38b1cfd773ccbcf8ec4d73ac3/templates/include/glean/glean.1.schema.json#L133). For example, the snippet to include the template for the counter metric type is:

```json
        "counter": {
          @GLEAN_BASE_OBJECT_1_JSON@,
          "additionalProperties": @GLEAN_COUNTER_1_JSON@
        },
```

If adding a labeled metric type as well, the same template from the "core" metric type can be reused:

```json
        "labeled_counter": {
          @GLEAN_BASE_OBJECT_1_JSON@,
          "additionalProperties": {
            @GLEAN_LABELED_GROUP_1_JSON@,
            "additionalProperties": @GLEAN_COUNTER_1_JSON@
          }
        },
```

After updating the templates, you need to regenerate the fully-qualified schema using [these instructions](https://github.com/mozilla-services/mozilla-pipeline-schemas#build).

The fully-qualified Glean schema is also used by the Glean SDK's unit test suite to make sure that ping payloads validate against the schema. Therefore, whenever the Glean JSON schema is updated, it should also be copied and checked in to the [Glean SDK repository](https://github.com/mozilla/glean). Specifically, copy the generated schema in `mozilla-pipeline-schemas/schemas/glean/glean.1.schema.json` to the root of the Glean SDK repository.

## Adding a new metric type to `mozilla-schema-generator`

Each new metric type also needs an entry in the Glean configuration in [`mozilla-schema-generator`](https://github.com/mozilla/mozilla-schema-generator). The config file for Glean is in [`glean.yaml`](https://github.com/mozilla/mozilla-schema-generator/blob/7276cfb3b14440f8cb93e57d9f167d7588092dae/mozilla_schema_generator/configs/glean.yaml#L1). Each entry in that file just needs some boilerplate for each metric type. For example, the snippet for the Counter metric type is:

```yaml
  counter:
    match:
      send_in_pings:
        not:
          - glean_ping_info
      type: counter
```

## Adding a new metric type to `lookml-generator`

Each new metric type needs to be enabled in the [`lookml-generator`](https://github.com/mozilla/lookml-generator).
The list of allowed Glean metric types is in [`generator/views/glean_ping_view.py.yaml`](https://github.com/mozilla/lookml-generator/blob/3e4a8fdca43bd96635be477c7418bbf996801eaf/generator/views/glean_ping_view.py#L21).
Add the new metric type to `ALLOWED_TYPES`:

```python
ALLOWED_TYPES = DISTRIBUTION_TYPES | {
    # ...
    "counter",
}
```
