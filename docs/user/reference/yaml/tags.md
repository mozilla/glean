# Tags YAML Registry Format

Any number of custom "tags" can be added to any metric or ping.
This can be useful in data discovery tools like the [Glean Dictionary](https://dictionary.telemetry.mozilla.org).
The tags for an application are defined in YAML files which follow
the [`tags.yaml` JSON schema](https://mozilla.github.io/glean_parser/tags-yaml.html).

These files must be parsed by [`glean_parser`](https://pypi.org/project/glean-parser/) at build time in order to generate the metadata.

For more information on how to introduce the `glean_parser` build step for a specific language /
environment, refer to the ["Adding Glean to your project"](../../user/adding-glean-to-your-project/index.md)
section of this book.

{{#include ../../../shared/blockquote-info.html}}

## Note on the naming of these files

> Although we refer to tag definitions YAML files as `tags.yaml` throughout Glean documentation
> this files may be named whatever makes the most sense for each project and may even be broken down
> into multiple files, if necessary.

## File structure

```yaml
---
# Schema
$schema: moz://mozilla.org/schemas/glean/tags/1-0-0

Search:
  description: Metrics or pings in the "search" domain
```

## Schema

Declaring the schema at the top of a tags definitions file is required, as it is what indicates that the current file is a tag definitions file.

## Name

Tag names are the top-level keys on tag definitions files.
One single definition file may contain multiple tag declarations.

There is no restriction on the name of a tag, aside from the fact that they have a maximum of 80 characters.

## Tag parameters

### Required parameters

#### `description`

A textual description of the tag.
It may contain [markdown syntax](https://www.markdownguide.org/basic-syntax/).
