# Substituting `glean_parser`

By default the Glean Kotlin SDK requires an exact version of the [`glean_parser`].
It's automatically installed as part of the Glean Gradle Plugin.

For upgrading the required `glean_parser` see [Upgrading glean_parser](../upgrading-glean-parser.md).

For local development the `glean_parser` can be replaced with a development version.
To use `glean_parser` from a git repository, add this to the project's `build.gradle`:

```groovy
ext.gleanParserOverride = "git+ssh://git@github.com/mozilla/glean_parser@main#glean-parser"
```

Adjust the repository URL as needed. `main` can be any available branch.
Ensure the suffix `#glean_parser` exists, as it tells the Python package management about the name.



[`glean_parser`]: https://github.com/mozilla/glean_parser/
