# Registering custom pings

After defining [custom pings](../../user/pings/custom.md) `glean_parser` is able to generate code from
`pings.yaml` files in a `Pings` object, which must be instantiated so Glean can send pings by name.

## API

### `registerPings`

Loads custom ping metadata into your application or library.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

In Kotlin, this object must be registered from your startup code before calling `Glean.initialize`
(such as in your application's `onCreate` method or a function called from that method).

```Kotlin
import org.mozilla.yourApplication.GleanMetrics.Pings

override fun onCreate() {
    Glean.registerPings(Pings)

    Glean.initialize(applicationContext, uploadEnabled = true)
}
```

</div>

<div data-lang="Java" class="tab"></div>

<div data-lang="Swift" class="tab">

In Swift, this object must be registered from your startup code before calling `Glean.shared.initialize`
(such as in your application's `UIApplicationDelegate` [`application(_:didFinishLaunchingWithOptions:)`](https://developer.apple.com/documentation/uikit/uiapplicationdelegate/1622921-application) method or a function called from that method).

```swift
import Glean

@UIApplicationMain
class AppDelegate: UIResponder, UIApplicationDelegate {
    func application(_: UIApplication, didFinishLaunchingWithOptions _: [UIApplication.LaunchOptionsKey: Any]?) -> Bool {
        Glean.shared.registerPings(GleanMetrics.Pings)

        Glean.shared.initialize(uploadEnabled = true)
    }
}
```

</div>

<div data-lang="Python" class="tab">

For Python, the `pings.yaml` file must be available and loaded at runtime.

While the Python SDK does provide a `Glean.register_ping_type` function, if your project is a script (i.e. just Python files in a directory), you can load the `pings.yaml` before calling `Glean.initialize` using:

```python
from glean import load_pings

pings = load_pings("pings.yaml")

Glean.initialize(
    application_id="my-app-id",
    application_version="0.1.0",
    upload_enabled=True,
)
```

If your project is a distributable Python package, you need to include the `pings.yaml` file using [one of the myriad ways to include data in a Python package](https://setuptools.readthedocs.io/en/latest/setuptools.html#including-data-files) and then use [`pkg_resources.resource_filename()`](https://setuptools.readthedocs.io/en/latest/pkg_resources.html#resource-extraction) to get the filename at runtime.

```Python
from glean import load_pings
from pkg_resources import resource_filename

pings = load_pings(resource_filename(__name__, "pings.yaml"))
```

</div>

<div data-lang="Rust" class="tab">

In Rust custom pings need to be registered individually.
This should be done before calling `glean::initialize`.

```Rust
use your_glean_metrics::pings;

glean::register_ping_type(&pings::custom_ping);
glean::register_ping_type(&pings::search);
glean::initialize(cfg, client_info);
```

</div>
<div data-lang="JavaScript" class="tab" data-info="On JavaScript environments, it is not necessary to register pings."></div>
<div data-lang="Firefox Desktop" class="tab" data-info="On Firefox Desktop all custom pings are registered automatically."></div>

</div>

{{#include ../../../shared/tab_footer.md}}
