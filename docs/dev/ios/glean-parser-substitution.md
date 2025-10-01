# Substituting `glean_parser`

By default the Glean Kotlin SDK requires an exact version of the [`glean_parser`].
It's automatically installed into a local Python virtual environment as part of the [`sdk_generator.sh`](https://github.com/mozilla/glean/blob/main/glean-core/ios/sdk_generator.sh) script.

For upgrading the required `glean_parser` see [Upgrading glean_parser](../upgrading-glean-parser.md).

For local development `glean_parser` can be replaced with a development version.

Inside your project's root directory should be a `.venv` directory.
If not force a rebuild of your project.

Then install a development version of `glean_parser` in this virtual environment.
With a local `glean_parser` checkout you can use:

```
.venv/bin/pip install -e path/to/your/checkout
```

Any changes in your `glean_parser` code should be reflected at build time.
Note that the build system will only be invoked if your `metrics.yaml` changed since the last build.
Force that using `touch path/to/your/metrics.yaml`.

To use `glean_parser` from a git repository install it like this:

```
.venv/bin/pip install "git+ssh://git@github.com/mozilla/glean_parser@main#glean-parser"
```

Adjust the repository URL as needed. `main` can be any available branch.
Ensure the suffix `#glean_parser` exists, as it tells the Python package management about the name.

[`glean_parser`]: https://github.com/mozilla/glean_parser/
