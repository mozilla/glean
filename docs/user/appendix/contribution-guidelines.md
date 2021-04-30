# Contribution Guidelines

There are two important questions to answer before adding new content to this book:

- Where to include this content?
- In which format to present it?

This guide aims to provide context for answering both questions.

## Table of contents

<!-- toc -->

## Where to add new content?

This book is divided in five different sections. Each section contains pages that are of a specific
type. New content will fit in one of these section. Following is an explanation on what kind of content
fits in each section.

### Overview

_Is the content you want to add an essay or high level explanation on some aspect of Glean?_

The overview section is the place to provide important higher level context for
users of Glean. This section may include essays about Glean’s views, principles,
data practices, etc. It also contains primers on information such as what is the Glean
SDK and Glean.js.

### User Guides

_Is the content you want to add a general purpose guide on a Glean aspect or feature?_

This section is the place for Glean SDK user guides. Pages under this section
contain prose format guides on how to include Glean in a project, how to add metrics,
how to create pings, and so on.

Pages on this section may link to the API Reference pages, but should not
themselves be API References.

Guides can be quite long, thus we should favor having one page per language binding instead
of using tabs.

### API Reference

_Is the content you want to add a developer reference on a specific Glean API?_

This section of the book contains reference for all of Glean’s user APIs.
Its pages follow a strict structure.

Each API description contains:

- A title with the name of the API.
  - It’s important to use titles, because they automatically generate links to that API.
- A brief description of what the API does.
- Tabs with examples for using that API in each language binding.
  - Even if a certain language binding do not contain a given API, the tab will be included in
  the tabs bar in the disabled state.

The API Reference pages should not contain any guides in prose format, they should all be linked from the User’s Guide when convenient.

### Language Binding Information

_Is the content you want to add a language binding specific guide on a Glean feature?_

Language bindings may require some dedicated pages, these section contains these pages.
Each language binding has a top level section under this section, specific pages live
under these titles.

### Appendix

_Is the content you want to add support content for the rest of the content on book?_

The appendix contains support information related to the Glean SDK or the content of this book.

## In which format to present content?

### General guidelines

#### Link to other internal pages and sections whenever that is possible

Each page of the book should be written as if it were the first page a user is visiting ever.
There should be links to other pages of the book wherever there is missing context in the
current page. This is important, because documentations are first and foremost reference books,
manuals. They should not be expected to be read in order.

#### Prefer using headers whenever a new topic is introduced

[mdbook](https://rust-lang.github.io/mdBook/index.html) (the tool used to build this book) turns
all headers into links. Which is useful to refer to specific documentation sections.

#### Favor creating new pages, instead of adding unrelated content to an already existing page

This makes it easier to find content through the Summary.

### Custom elements

#### Tabs

Tabs are useful for providing small code examples of Glean's APIs for each language binding.
A tabs section starts with the `tab_header` [include](https://rust-lang.github.io/mdBook/format/mdbook.html#including-files) and ends with the `tab_footer` [include](https://rust-lang.github.io/mdBook/format/mdbook.html#including-files).
Each tab is declared in between these [include](https://rust-lang.github.io/mdBook/format/mdbook.html#including-files) statements.

Each tab content is placed inside an html `div` tag with the `data-lang` and `class="tab"` attributes.
The `data-lang` attribute contains the title of the tab. Titles must match for different tabs on the
same language binding. Whenever a user clicks in a tab with a specific title, all tabs with that same
title will be opened by default, until the user clicks in a tab with a different title.

Every tab section contains tabs for each Glean language binding, even that language binding does not
provide the API in question. In this case, the tab div is still there without any inner HTML. When
that is the case that tab will be rendered in a disabled state.

Each tab section contains tabs for the following language bindings:

- Kotlin
- Java
- Swift
- Python
- Rust
- Javascript
- Firefox Desktop

Finally, here is an example code for a tabs sections:

```html
\{{#include ../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab">
  Kotlin information...
</div>
<div data-lang="Java" class="tab">
  Java information...
</div>
<div data-lang="Swift" class="tab">
  Swift information...
</div>
<div data-lang="Python" class="tab">
  Python information...`
</div>
<div data-lang="Rust" class="tab">
  Rust information...
</div>
<!--
  In this example, Javascript and Firefox Desktop
  would show up as disabled in the final page.
-->
<div data-lang="Javascript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab"></div>
\{{#include ../../shared/tab_footer.md}}
```

And this is how those tabs will look like:

{{#include ../../shared/tab_header.md}}
<div data-lang="Kotlin" class="tab">
  Kotlin information...
</div>
<div data-lang="Java" class="tab">
  Java information...
</div>
<div data-lang="Swift" class="tab">
  Swift information...
</div>
<div data-lang="Python" class="tab">
  Python information...`
</div>
<div data-lang="Rust" class="tab">
  Rust information...
</div>
<div data-lang="Javascript" class="tab"></div>
<div data-lang="Firefox Desktop" class="tab"></div>

{{#include ../../shared/tab_footer.md}}

#### Custom block quotes

Sometimes it is necessary to bring attention to a special piece of information, or simply to provide
extra context related to the a given text.

In order to do that, there are three custom block quote formats available.

{{#include ../../shared/blockquote-info.html}}

##### Info quote

> An information block quote format, to provide useful extra context for a given text.

{{#include ../../shared/blockquote-warning.html}}

##### Warning quote

> A warning block quote format, to provide useful warning related to a given text.

{{#include ../../shared/blockquote-stop.html}}

##### Stop quote

> A stronger warning block quote format, to provide useful warning related to a given text in
> a more emphatic format. Use these sparingly.

To include such quotes, again you can use mdbook [include](https://rust-lang.github.io/mdBook/format/mdbook.html#including-files) statements.

For the above quotes, this is the corresponding code.

```markdown
\{{#include ../../shared/blockquote-info.html}}

##### Info quote

> An information blockquote format, to provide useful extra context for a given text.

\{{#include ../../shared/blockquote-warning.html}}

##### Warning quote

> A warning blockquote format, to provide useful warning related to a given text.

\{{#include ../../shared/blockquote-stop.html}}

##### Stop quote

> A stronger warning blockquote format, to provide useful warning related to a given text in a
> more emphatic format. Use these sparingly.
```

It is possible to use any level header with these special block quotes
and also no header at all.
