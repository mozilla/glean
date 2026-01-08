# Glean Devhub

Collect different metrics from the Glean SDK build and publish them.
Results will be displayed on <https://mozilla.github.io/glean/devhub>.

* This script is run by the CI infrastructure on every merge to `main`.
* It collects a set of "metrics", where a "metric" can be anything that can be measured with a single numeric value.
* The results of all measurements are serialized as a single JSON object.
* The key part: this JSON is then stored in a "distributed database" for our visualization
  front-end to pick up. This "database" is just a newline-delimited JSON file in a Git repository.

To generate a `DEVHUBDB_TOKEN` (used on CI to publish to the database repository):

1. Go to <https://github.com/settings/personal-access-tokens/new>
2. Fill out token name (e.g. "Glean CI devhubdb token").
3. Resource owner: "mozilla"
4. Expiry: "366 days" (maximum available)
5. Repository access: "Only select repositories"
6. Select repositories: "mozilla/glean-devhubdb"
7. Add permissions: "Metadata"
8. Add permissions: "Contents"; Access: "Read and write".
9. "Generate token".
10. (Copy token.)

To update token in Glean CI:

1. <https://github.com/mozilla/glean/settings/environments>
2. Click "production".
3. Environment Secrets > Edit `DEVHUBDB_TOKEN`
4. Paste token; "Update secret"

## References

This code is based on ideas from:

* [TigerBeetle's devhub](https://github.com/tigerbeetle/tigerbeetle/blob/b6d541562290f23c10760ea20b559cf21b9010b0/src/scripts/devhub.zig)
* [PerfHerder](https://treeherder.mozilla.org/perfherder/),
  and specifically [`PerfStats`](https://firefox-source-docs.mozilla.org/performance/perfstats.html) and [`MozGTestBench`](https://firefox-source-docs.mozilla.org/gtest/index.html#mozgtestbench)
