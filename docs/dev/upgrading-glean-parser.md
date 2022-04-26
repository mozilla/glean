# Upgrading glean_parser

To upgrade the version of `glean_parser` used by the Glean SDK, run the `bin/update-glean-parser-version.sh` script, providing the version as a command line parameter:

```sh
bin/update-glean-parser-version.sh 1.28.3
```

This will update the version in all of the required places. Commit those changes to `git` and submit a pull request.

No further steps are required to use the new version of `glean_parser` for code generation: all of the build integrations automatically update `glean_parser` to the correct version.

For testing the Glean Python bindings, the virtual environment needs to be deleted to force an upgrade of `glean_parser`:

```sh
rm -rf glean-core/python/.venv*
```
