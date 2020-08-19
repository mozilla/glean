# Developing documentation

The documentation in this repository pertains to the Glean SDK.  That is, the client-side code for Glean telemetry.
Documentation for Glean in general, and the Glean-specific parts of the data pipeline and analysis is [documented elsewhere](https://docs.telemetry.mozilla.org/concepts/glean/glean.html) in the [`firefox-data-docs` repository](https://github.com/mozilla/firefox-data-docs).

The main narrative documentation is written in Markdown and converted to static HTML using [mdbook](https://rust-lang.github.io/mdBook/).

API docs are also generated from docstrings for Rust, Kotlin, Swift and Python.

## Building documentation

### Building the narrative (book) documentation

The `mdbook` crate is required in order to build the narrative documentation:

```sh
cargo install mdbook mdbook-mermaid mdbook-open-on-gh
```

Then both the narrative and Rust API docs can be built with:

```sh
make docs
# ...or...
bin/build-rust-docs.sh
# ...or on Windows...
bin/build-rust-docs.bat
```

The built narrative documentation is saved in `build/docs/book`, and the Rust API documentation is saved in `build/docs/docs`.

### Building API documentation

{{#include ../tab_header.md}}

<div data-lang="Kotlin" class="tab">

Kotlin API documentation is generated using [dokka](https://github.com/Kotlin/dokka).
It is automatically installed by Gradle.

To build the Kotlin API documentation:

```sh
make kotlin-docs
```

The generated documentation is saved in `build/docs/javadoc`.

</div>

<div data-lang="Swift" class="tab">

Swift API documentation is generated using [jazzy](https://github.com/realm/jazzy).
It can be installed using:

1. Install the latest Ruby: `brew install ruby`
2. Make the installed Ruby available: `export PATH=/usr/local/opt/ruby/bin:$PATH` (and add that line to your `.bashrc`)
3. Install the documentation tool: `gem install jazzy`

To build the Swift API documentation:

```sh
make swift-docs
```

The generated documentation is saved in `build/docs/swift`.

</div>

<div data-lang="Python" class="tab">

The Python API docs are generated using [pdoc3](https://pdoc3.github.io/pdoc/).
It is installed as part of [creating the virtual environment for Python development](python/setting-up-python-build-environment.html#create-a-virtual-environment).

To build the Python API documentation:

```
make python-docs
```

The generated documentation is saved in `build/docs/python`.

</div>

<div data-lang="C#" class="tab">

TODO. To be implemented in [bug 1648410](https://bugzilla.mozilla.org/show_bug.cgi?id=1648410).

</div>

{{#include ../tab_footer.md}}

### Checking links

Internal links within the documentation can be checked using the [`link-checker`](https://www.npmjs.com/package/link-checker) tool.
External links are currently not checked, since this takes considerable time and frequently fails in CI due to networking restrictions or issues.

Link checking requires building the narrative documentation as well as all of the API documentation for all languages.
It is rare to build all of these locally (and in particular, the Swift API documentation can only be built on macOS), therefore it is reasonable to let CI catch broken link errors for you.

If you do want to run the `link-checker` locally, it can be installed using `npm` or your system's package manager.
Then, run `link-checker` with:

```sh
make linkcheck
```

### Spell checking

The narrative documentation (but not the API documentation) is spell checked using [aspell](http://aspell.net/).

On Unix-like platforms, it can be installed using your system's package manager:

```sh
sudo apt install aspell-en
# ...or...
sudo dnf install aspell-en
# ...or...
brew install aspell
```

Note that aspell 0.60.8 or later is required, as that is the first release with markdown support.

You can the spell check the narrative documentation using the following:

```sh
make spellcheck
# ...or...
bin/spellcheck.sh
```

This will bring up an interactive spell-checking environment in the console.
Pressing `a` to add a word will add it to the project's local `.dictionary` file, and the changes should be committed to `git`.

