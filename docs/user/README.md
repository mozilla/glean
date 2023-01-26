# Introduction

The Glean SDKs are modern cross-platform telemetry client libraries
and are a part of the [Glean project](https://docs.telemetry.mozilla.org/concepts/glean/glean.html).

![Glean logo](glean.jpeg)

The Glean SDKs are available for several programming languages and development environments.
Each SDK aims to contain the same group of features with similar, but idiomatic APIs.

To learn more about each SDK, refer to the [SDKs overview](./language-bindings/index.md) page.

To get started adding Glean to your project, choose one of the following guides:

- **[Kotlin](./user/adding-glean-to-your-project/kotlin.md)**
  - Get started adding Glean to an Android application or library.
- **[Swift](./user/adding-glean-to-your-project/swift.md)**
  - Get started adding Glean to an iOS application or library.
- **[Python](./user/adding-glean-to-your-project/python.md)**
  - Get started adding Glean to any Python project.
- **[Rust](./user/adding-glean-to-your-project/rust.md)**
  - Get started adding Glean to any Rust project or library.
- **[JavaScript](./user/adding-glean-to-your-project/javascript.md)**
  - Get started adding Glean to a website, web extension or Node.js project.
- **[QML](./user/adding-glean-to-your-project/qt.md)**
  - Get started adding Glean to a Qt/QML application or library.
<!-- - **[Firefox Desktop](TODO)**
  - Get started adding Glean to a Firefox Desktop component. -->

For development documentation on the `Glean SDK`, refer to [the Glean SDK development book](../dev/index.html).

## Sections

### [User Guides](./user/adding-glean-to-your-project/index.html)

This section of the book contains step-by-step guides and essays detailing how to
achieve specific tasks with each Glean SDK.

It contains guides on the first steps of integrating Glean into your project,
choosing the right metric type for you, debugging products that use Glean and
Glean's built-in error reporting mechanism.

If you want to start using Glean to report data, this is the section you should read.

### [API Reference](./reference/yaml/index.html)

This section of the book contains reference pages for Glean’s user facing APIs.

If you are looking for information a specific Glean API, this is the section you should check out.

### [SDK Specific Information](./language-bindings/android/index.html)

This section contains guides and essays regarding specific usage information
and possibilities in each Glean SDK.

Check out this section for more information on the SDK you are using.

### Appendix

#### [Glossary](./appendix/glossary.html)

In this book we use a lot of Glean specific terminology. In the glossary, we go through
many of the terms used throughout this book and describe exactly what we mean when we use them.

#### [Changelog](./appendix/changelog/index.html)

This section contains detailed notes about changes in Glean, per release.

#### [This Week in Glean](./appendix/twig.html)

“This Week in Glean” is a series of blog posts that the Glean Team at Mozilla is using to try
to communicate better about our work. They could be release notes, documentation, hopes, dreams,
or whatever: so long as it is inspired by Glean.

#### [Contribution Guidelines](./appendix/contribution-guidelines.html)

This section contains detailed information on where and how to include new content to this book.

## Contact

To contact the Glean team you can:

- Find us in the [#glean channel on chat.mozilla.org](https://chat.mozilla.org/#/room/#glean:mozilla.org).
- To report issues or request changes, file a bug in [Bugzilla in Data Platform & Tools :: Glean: SDK](https://bugzilla.mozilla.org/enter_bug.cgi?assigned_to=nobody%40mozilla.org&bug_ignored=0&bug_severity=normal&bug_status=NEW&bug_type=defect&cf_fx_iteration=---&cf_fx_points=---&cf_status_firefox100=---&cf_status_firefox101=---&cf_status_firefox99=---&cf_status_firefox_esr91=---&cf_tracking_firefox100=---&cf_tracking_firefox101=---&cf_tracking_firefox99=---&cf_tracking_firefox_esr91=---&component=Glean%3A%20SDK&contenttypemethod=list&contenttypeselection=text%2Fplain&defined_groups=1&filed_via=standard_form&flag_type-4=X&flag_type-607=X&flag_type-721=X&flag_type-737=X&flag_type-799=X&flag_type-800=X&flag_type-803=X&flag_type-936=X&flag_type-947=X&form_name=enter_bug&maketemplate=Remember%20values%20as%20bookmarkable%20template&op_sys=Unspecified&priority=P3&product=Data%20Platform%20and%20Tools&rep_platform=Unspecified&status_whiteboard=%5Bglean-sdk%3Am%3F%5D&target_milestone=---&version=unspecified).
- Send an email to *glean-team@mozilla.com*.
- The Glean SDKs team is: *:janerik*, *:dexter*, *:travis*, *:chutten*, *:perrymcmanis*.

## License

The Glean SDKs Source Code is subject to the terms of the Mozilla Public License v2.0.
You can obtain a copy of the MPL at <https://mozilla.org/MPL/2.0/>.
