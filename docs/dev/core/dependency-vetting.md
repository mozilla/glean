# Dependency vetting

[cargo-vet]: https://mozilla.github.io/cargo-vet/index.html

The Glean SDK uses `cargo-vet` to ensure that third-party Rust dependencies have been audited by a trusted entity.
For a full overview over `cargo-vet`'s capabilities and usage see the [`cargo-vet` documentation][cargo-vet].

New or updated dependencies need to be audited to allow their usage.
Dependency audits are shared with downstream Mozilla projects.

## 3-step guide

* `cargo vet`
* `cargo vet diff $crate $old-version $new-version`
* `cargo vet certify`

## Longer guide

### Prerequisites

Install `cargo-vet`:

```
cargo install cargo-vet
```

### Auditing steps

After adding or updating a dependency start the audit process:

```
cargo vet
```

This will scan the dependencies for any missing audits and show instructions how to proceed.
For dependency updates you should start by looking at the diff.
For new dependencies you will look at the full code.

This will be something like the following command for any dependency:

```
cargo vet diff $crate $old-version $new-version
```

Please read the printed criteria and consider them when performing the audit.
If unsure please ask other Glean engineers for help.

It will then provide you with a Sourcegraph link to inspect the code.
Alternatively you can run with `--mode=local` to get a diff view in your terminal.

Once you have reviewed run:

```
cargo vet certify
```

and follow the instructions.

Finally you will notice the audit being added to `supply-chain/audits.toml`.
Add this file to your commit and create a pull request.
