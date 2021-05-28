# Adding Glean to your project

<!-- toc -->

## Glean integration checklist

The Glean integration checklist can help to ensure your Glean SDK-using product is meeting all of the recommended guidelines.

Products (applications or libraries) using the Glean SDK to collect telemetry **must**:

1. [Integrate the Glean SDK into the build system](#integrating-with-your-project). Since the Glean SDK does some code generation for your metrics at build time, this requires a few more steps than just adding a library.

2. Go through [data review process](https://wiki.mozilla.org/Firefox/Data_Collection) for all newly collected data.

3. Ensure that telemetry coming from automated testing or continuous integration is either not sent to the telemetry server or [tagged with the `automation` tag using the `sourceTag` feature](debugging/index.md#available-debugging-features).

4. At least one week before releasing your product, [file a data engineering bug][dataeng-bug] to enable your product's application id and have your metrics be indexed by the [Glean Dictionary].

Additionally, applications (but not libraries) **must**:

5. [Initialize Glean](../reference/general/index.md#initializing-the-glean-sdk) as early as possible at application startup.

6. Provide a way for users to turn data collection off (e.g. providing settings to control `Glean.setUploadEnabled()`). The exact method used is application-specific.

## Usage

### Integrating with your project

{{#include ../../shared/tab_header.md}}

<div data-lang="C#" class="tab">

TODO. To be implemented in [bug 1643568](https://bugzilla.mozilla.org/show_bug.cgi?id=1643568).

</div>

{{#include ../../shared/tab_footer.md}}

### Adding new metrics

All metrics that your project collects must be defined in a `metrics.yaml` file.

To learn more, see [adding new metrics](adding-new-metrics.md).
See the [metric parameters](metric-parameters.md) documentation which provides reference information about the contents of that file.

> **Important**: as stated [before](adding-glean-to-your-project.md#glean-integration-checklist), any new data collection requires documentation and data-review.
> This is also required for any new metric automatically collected by the Glean SDK.

{{#include ../../shared/tab_header.md}}

<div data-lang="C#" class="tab">

A new build target needs to be added to the project `csproj` file in order to generate the metrics and pings APIs from the registry files (e.g. `metrics.yaml`, `pings.yaml`).

```xml
<Project>
  <!-- ... other directives ... -->

  <Target Name="GleanIntegration" BeforeTargets="CoreCompile">
    <ItemGroup>
      <!--
        Note that the two files are not required: Glean will work just fine
        with just the 'metrics.yaml'. A 'pings.yaml' is only required if custom
        pings are defined.
        Please also note that more than one metrics file can be added.
      -->
      <GleanRegistryFiles Include="metrics.yaml" />
      <GleanRegistryFiles Include="pings.yaml" />
    </ItemGroup>
    <!-- This is what actually runs the parser. -->
    <GleanParser RegistryFiles="@(GleanRegistryFiles)" OutputPath="$(IntermediateOutputPath)Glean" Namespace="csharp.GleanMetrics" />

    <!--
      And this adds the generated files to the project, so that they can be found by
      the compiler and Intellisense.
    -->
    <ItemGroup>
      <Compile Include="$(IntermediateOutputPath)Glean\**\*.cs" />
    </ItemGroup>
  </Target>
</Project>
```

This is using the Python 3 interpreter found in `PATH` under the hood. The `GLEAN_PYTHON` environment variable can be used to provide the location of the Python 3 interpreter.

</div>

{{#include ../../shared/tab_footer.md}}

### Adding custom pings

Please refer to the [custom pings documentation](pings/custom.md).

> **Important**: as stated [before](adding-glean-to-your-project.md#glean-integration-checklist), any new data collection requires documentation and data-review.
> This is also required for any new metric automatically collected by the Glean SDK.

### Parallelism

All of the Glean SDK's target languages use a separate worker thread to do most of its work, including any I/O. This thread is fully managed by the Glean SDK as an implementation detail. Therefore, users should feel free to use the Glean SDK wherever it is most convenient, without worrying about the performance impact of updating metrics and sending pings.

{{#include ../../shared/tab_header.md}}

<div data-lang="Python" class="tab">
Since the Glean SDK performs disk and networking I/O, it tries to do as much of its work as possible on separate threads and processes.
Since there are complex trade-offs and corner cases to support Python parallelism, it is hard to design a one-size-fits-all approach.

#### Default behavior

When using the Python bindings, most of the Glean SDK's work is done on a separate thread, managed by the Glean SDK itself.
The Glean SDK releases the Global Interpreter Lock (GIL) for most of its operations, therefore your application's threads should not be in contention with the Glean SDK's worker thread.

The Glean SDK installs an [`atexit` handler](https://docs.python.org/3/library/atexit.html) so that its worker thread can cleanly finish when your application exits.
This handler will wait up to 30 seconds for any pending work to complete.

By default, ping uploading is performed in a separate child process. This process will continue to upload any pending pings even after the main process shuts down. This is important for commandline tools where you want to return control to the shell as soon as possible and not be delayed by network connectivity.

#### Cases where subprocesses aren't possible

The default approach may not work with applications built using [`PyInstaller`](https://www.pyinstaller.org/) or similar tools which bundle an application together with a Python interpreter making it impossible to spawn new subprocesses of that interpreter. For these cases, there is an option to ensure that ping uploading occurs in the main process. To do this, set the `allow_multiprocessing` parameter on the `glean.Configuration` object to `False`.

#### Using the `multiprocessing` module

Additionally, the default approach does not work if your application uses the `multiprocessing` module for parallelism.
The Glean SDK can not wait to finish its work in a `multiprocessing` subprocess, since `atexit` handlers are not supported in that context.  
Therefore, if the Glean SDK detects that it is running in a `multiprocessing` subprocess, all of its work that would normally run on a worker thread will run on the main thread.
In practice, this should not be a performance issue: since the work is already in a subprocess, it will not block the main process of your application.
</div>

{{#include ../../shared/tab_footer.md}}

### Testing metrics

In order to make testing metrics easier 'out of the box', all metrics include a set of test API functions in order to facilitate unit testing.  These include functions to test whether a value has been stored, and functions to retrieve the stored value for validation.  For more information, please refer to [Unit testing Glean metrics](testing-metrics.md).

[dataeng-bug]: https://bugzilla.mozilla.org/enter_bug.cgi?assigned_to=nobody@mozilla.org&bug_ignored=0&bug_severity=--&bug_status=NEW&bug_type=task&cf_fx_iteration=---&cf_fx_points=---&comment=%23%20To%20be%20filled%20by%20the%20requester%0A%0A%2A%2AApplication%20ID%5C%2A%2A%2A%3A%20my.app_id%0A%2A%2AApplication%20Canonical%20Name%2A%2A%3A%20My%20Application%0A%2A%2ADescription%2A%2A%3A%20Brief%20description%20of%20your%20application%0A%2A%2AData-review%20response%20link%2A%2A%3A%20The%20link%20to%20the%20data%20response%20to%20the%20data%20collection%20request%20for%20adding%20Glean%20to%20your%20project.%0A%2A%2ARepository%20URL%2A%2A%3A%20https%3A%2F%2Fgithub.com%2Fmozilla%2Fmy_app_name%0A%2A%2ALocations%20of%20%60metrics.yaml%60%20files%20%28can%20be%20many%29%3A%2A%2A%0A%20%20-%20src%2Fmetrics.yaml%0A%0A%2A%2ALocations%20of%20%60pings.yaml%60%20files%20%28can%20be%20many%29%3A%2A%2A%0A%20-%20src%2Fpings.yaml%0A%0A%2A%2ADependencies%5C%2A%5C%2A%2A%2A%3A%0A%20-%20glean-core%0A%0A%2A%2ARetention%20Days%5C%2A%5C%2A%5C%2A%2A%2A%3A%20N%0A%0A%23%23%20_%28Optional%29_%20To%20be%20filled%20by%20the%20requester%0A%2A%2ADoes%20the%20product%20require%20end-to-end%20encryption%20in%20the%20pipeline%3F%2A%2A%20Yes%20%7C%20No%0A%2A%2AIf%20answered%20yes%20to%20the%20above%2C%20who%20should%20be%20granted%20access%20to%20the%20data%3F%2A%2A%0A%0A-%20LDAP%20account%201%0A-%20LDAP%20account%202%0A%0A%23%23%20Notes%20and%20guidelines%0A%0A%5C%2A%20This%20is%20the%20identifier%20used%20to%20initialize%20Glean%20%28or%20the%20id%20used%20on%20the%20store%20on%20Android%20and%20Apple%20devices%29.%0A%0A%5C%2A%5C%2A%20Dependencies%20can%20be%20found%20%5Bin%20the%20Glean%20repositories%5D%28https%3A%2F%2Fprobeinfo.telemetry.mozilla.org%2Fv2%2Fglean%2Flibrary-variants%29.%20Each%20dependency%20must%20be%20listed%20explicitly.%20For%20example%2C%20the%20default%20Glean%20probes%20will%20only%20be%20included%20if%20glean%20itself%20is%20a%20dependency.%0A%0A%5C%2A%5C%2A%5C%2A%20Number%20of%20days%20that%20raw%20data%20will%20be%20retained.%20A%20good%20default%20is%20180.%20We%20can%20change%20this%20later%20to%20accommodate%20longer%20retention%20periods%2C%20though%20we%20cannot%20recover%20data%20that%20is%20past%20the%20retention%20period%20%28for%20example%2C%20we%20cannot%20recover%20data%20that%20is%20200%20days%20old%20if%20your%20retention%20period%20is%20180%20days%29.%0A%0A%23%23%20Need%20additional%20help%3F%0AIf%20you%20need%20new%20dependencies%2C%20please%20file%20new%20bugs%20for%20them%2C%20separately%20from%20this%20one.%20For%20any%20questions%2C%20ask%20in%20the%20%23glean%20channel.%0A%0A%23%20To%20be%20filled%20by%20the%20Glean%20team%0A%5B%2A%2AApplication%20friendly%20name%2A%2A%5D%28https%3A%2F%2Fmozilla.github.io%2Fprobe-scraper%2F%23tag%2Fapplication%29%3A%20my_app_name%0A%0A%23%23%20The%20following%20are%20only%20required%20for%20products%20requiring%20encryption%3A%0A%2A%2ADocument%20namespace%2A%2A%3A%20my-app-name%0A%0A%2A%2APlease%20flag%20Operations%20on%20this%20bug%20to%20request%20the%20creation%20of%20encryption%20keys.%2A%2A&component=Glean%20Platform&contenttypemethod=list&contenttypeselection=text%2Fplain&defined_groups=1&filed_via=standard_form&flag_type-4=X&flag_type-607=X&flag_type-800=X&flag_type-803=X&flag_type-936=X&form_name=enter_bug&maketemplate=Remember%20values%20as%20bookmarkable%20template&op_sys=Unspecified&priority=--&product=Data%20Platform%20and%20Tools&rep_platform=Unspecified&short_desc=Enable%20new%20Glean%20App%20%60my.app_id%60&target_milestone=---&version=unspecified
[Glean Dictionary]: https://dictionary.telemetry.mozilla.org
