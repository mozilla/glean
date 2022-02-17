# Adding Glean to your JavaScript project

This page provides a step-by-step guide on how to integrate
the Glean JavaScript SDK into a JavaScript project.

Nevertheless this is just one of the required steps for integrating Glean successfully into a project. Check you the full [Glean integration checklist](./index.md) for a comprehensive list of all the steps involved in doing so.

The Glean JavaScript SDK allows integration with three distinct JavaScript environments:
**websites, web extension and Node.js.**

## Requirements

* Node.js >= 12.20.0
* npm >= 7.0.0
* Webpack >= 5.34.0
* Python >= 3.6
  * The `glean` command requires Python to download [`glean_parser`](https://mozilla.github.io/glean_parser/) which is a Python library

### Browser extension specific requirements

* [webextension-polyfill](https://github.com/mozilla/webextension-polyfill) >= 0.8.0
  * Glean.js assumes a Promise-based `browser` API: Firefox provides such an API by default.
  Other browsers may require using a polyfill library such us `webextension-polyfill`
  when using Glean in browser extensions
* [Host permissions](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions#host_permissions) to the telemetry server
  * Only necessary if the defined server endpoint denies
  [cross-origin](https://developer.mozilla.org/en-US/docs/Web/HTTP/CORS) requests
  * Not necessary if using the default `https://incoming.telemetry.mozilla.org`.
* ["storage"](https://developer.mozilla.org/en-US/docs/Mozilla/Add-ons/WebExtensions/manifest.json/permissions#api_permissions) API permissions

{{#include ../../../shared/blockquote-info.html}}

#### Browser extension example configuration

> The [`manifest.json`](https://github.com/mozilla/glean.js/blob/main/samples/web-extension/javascript/manifest.json)
> file of the sample browser extension available on the `mozilla/glean.js` repository provides
> an example on how to define the above permissions as well as how and where to load
> the `webextension-polyfill` script.

## Setting up the dependency

The Glean JavaScript SDK is distributed as an npm package
[`@mozilla/glean`](https://www.npmjs.com/package/@mozilla/glean).

This package has different entry points to access the different SDKs.

- `@mozilla/glean/web` gives access to the websites SDK
- `@mozilla/glean/webext` gives access to the web extension SDK
- `@mozilla/glean/node` gives access to the Node.js SDK[^1]

[^1]: The Node.js SDK does not have persistent storage yet. This means, Glean does not persist
state throughout application runs. For updates on the implementation of this feature in Node.js,
follow [Bug 1728807](https://bugzilla.mozilla.org/show_bug.cgi?id=1728807).

Install Glean in your JavaScript project, by running:

```bash
npm install @mozilla/glean
```

Then import Glean into your project:

```js
// Importing the Glean JavaScript SDK for use in **web extensions**
//
// esm
import Glean from "@mozilla/glean/webext";
// cjs
const { default: Glean } = require("@mozilla/glean/webext");

// Importing the Glean JavaScript SDK for use in **websites**
//
// esm
import Glean from "@mozilla/glean/web";
// cjs
const { default: Glean } = require("@mozilla/glean/web");

// Importing the Glean JavaScript SDK for use in **Node.js**
//
// esm
import Glean from "@mozilla/glean/node";
// cjs
const { default: Glean } = require("@mozilla/glean/node");
```

{{#include ../../../shared/blockquote-warning.html}}

### Browser extension security considerations

> In case of privilege-escalation attack into the context of the web extension using Glean,
> the malicious scripts would be able to call Glean APIs or use the `browser.storage.local` APIs directly.
> That would be a risk to Glean data, but not caused by Glean. Glean-using extensions should be careful
> not to relax the default Content-Security-Policy that generally prevents these attacks.

### Common import errors

#### "Cannot find module '@mozilla/glean'"

Glean.js does not have a [`main`](https://nodejs.org/api/packages.html#packages_main_entry_point_export) package entry point.
Instead it relies on a series of entry points depending on the platform you are targeting.
 
In order to import Glean use:

```js
import Glean from '@mozilla/glean/{your-platform}'
```
 
#### "Module not found: Error: Can't resolve '@mozilla/glean/webext' in '...'"
 
Glean.js relies on Node.js' [subpath exports](https://nodejs.org/api/packages.html#packages_subpath_exports)
feature to define multiple package entry points.
 
Please make sure that you are using a supported Node.js runtime and also make sure the tools you are using support this Node.js feature.

## Setting up metrics and pings code generation

In JavaScript, the metrics and pings definitions must be parsed at build time.
The `@mozilla/glean` package exposes [glean_parser](https://github.com/mozilla/glean_parser) through the `glean` script.

To parse your YAML registry files using this script, define a new script in your `package.json` file:

```json
{
  // ...
  "scripts": {
    // ...
    "build:glean": "glean translate path/to/metrics.yaml path/to/pings.yaml -f javascript -o path/to/generated",
    // Or, if you are building for a Typescript project
    "build:glean": "glean translate path/to/metrics.yaml path/to/pings.yaml -f typescript -o path/to/generated"
  }
}
```

Then run this script by calling:

```bash
npm run build:glean
```

## Automation steps

### Documentation

{{#include ../../../shared/blockquote-warning.html}}

#### Prefer using the Glean Dictionary

> While it is still possible to generate Markdown documentation,
> if working on a public Mozilla project rely on the [Glean Dictionary] for documentation.
> Your product will be automatically indexed by the Glean Dictionary after it gets enabled in the pipeline.

One of the commands provided by `glean_parser` allows users to generate Markdown documentation based on the contents of their YAML registry files.
To perform that translation, use the `translate` command with a different output format, as shown below.

In your `package.json`, define the following script:

```json
{
  // ...
  "scripts": {
    // ...
    "docs:glean": "glean translate path/to/metrics.yaml path/to/pings.yaml -f markdown -o path/to/docs",
  }
}
```

Then run this script by calling:

```bash
npm run docs:glean
```

### YAML registry files linting

Glean includes a "linter" for the YAML registry files called the `glinter` that catches a number of common mistakes in these files.
To run the linter use the `glinter` command.

In your `package.json`, define the following script:

```json
{
  // ...
  "scripts": {
    // ...
    "lint:glean": "glean glinter path/to/metrics.yaml path/to/pings.yaml",
  }
}
```

Then run this script by calling:

```bash
npm run lint:glean
```

[Glean Dictionary]: https://dictionary.telemetry.mozilla.org
