# Enabling data to be ingested by the data platform

This page provides a step-by-step guide on how to enable data from your product to be ingested by the data platform.

This is just one of the required steps for integrating Glean successfully into a product.
Check the full [Glean integration checklist](./index.md) for a comprehensive list of all the steps involved in doing so.

## Requirements

* GitHub Workflows

## Add your product to probe scraper

At least one week before releasing your product, [file a data engineering bug][dataeng-bug] to enable your product's application id.
This will result in your product being added to [probe scraper](https://github.com/mozilla/probe-scraper)'s
[`repositories.yaml`](https://github.com/mozilla/probe-scraper/blob/main/repositories.yaml).

### Validate and publish metrics

After your product has been enabled, you must submit commits to probe scraper to validate and publish metrics.
Metrics will only be published from branches defined in probe scraper's `repositories.yaml`, or the Git default branch if not explicitly configured.
This should happen on every CI run to the specified branches.
Nightly jobs will then automatically add published metrics to the [Glean Dictionary](https://dictionary.telemetry.mozilla.org/) and other data platform tools.

Enable the GitHub Workflow by creating a new file `.github/workflows/glean-probe-scraper.yml` with the following content:

```yaml
---
name: Glean probe-scraper
on: [push, pull_request]
jobs:
  glean-probe-scraper:
    uses: mozilla/probe-scraper/.github/workflows/glean.yaml@main
```

## Add your library to probe scraper

At least one week before releasing your product, [file a data engineering bug][dataeng-bug-libraries] to add your library to probe scraper
and be scraped for metrics as a dependency of another product.
This will result in your library being added to [probe scraper](https://github.com/mozilla/probe-scraper)'s
[`repositories.yaml`](https://github.com/mozilla/probe-scraper/blob/main/repositories.yaml).

[dataeng-bug]: https://bugzilla.mozilla.org/enter_bug.cgi?assigned_to=nobody%40mozilla.org&bug_ignored=0&bug_severity=--&bug_status=NEW&bug_type=task&cf_accessibility_severity=---&cf_fx_iteration=---&cf_fx_points=---&cf_status_firefox128=---&cf_status_firefox129=---&cf_status_firefox130=---&cf_tracking_firefox128=---&cf_tracking_firefox129=---&cf_tracking_firefox130=---&cf_tracking_firefox_relnote=---&comment=%2523%20To%20be%20filled%20by%20the%20requester%0D%0A%0D%0A%2A%2AApplication%20ID%5C%2A%2A%2A%3A%20my.app_id%0D%0A%2A%2AApplication%20Canonical%20Name%2A%2A%3A%20My%20Application%0D%0A%2A%2ADescription%2A%2A%3A%20Brief%20description%20of%20your%20application%0D%0A%2A%2AData-review%20response%20link%2A%2A%3A%20The%20link%20to%20the%20data%20response%20to%20the%20data%20collection%20request%20for%20adding%20Glean%20to%20your%20project.%0D%0A%2A%2ARepository%20URL%2A%2A%3A%20https%3A%2F%2Fgithub.com%2Fmozilla%2Fmy_app_name%0D%0A%2A%2ALocations%20of%20%60metrics.yaml%60%20files%20%28can%20be%20many%29%3A%2A%2A%0D%0A%20%20-%20src%2Fmetrics.yaml%0D%0A%0D%0A%2A%2ALocations%20of%20%60pings.yaml%60%20files%20%28can%20be%20many%29%3A%2A%2A%0D%0A%20-%20src%2Fpings.yaml%0D%0A%0D%0A%2A%2ADependencies%5C%2A%5C%2A%2A%2A%3A%0D%0A%20-%20glean-core%0D%0A%0D%0A%2A%2ARetention%20Days%5C%2A%5C%2A%5C%2A%2A%2A%3A%20N%0D%0A%0D%0A%2A%2AData%20access%20restrictions%5C%2A%5C%2A%5C%2A%5C%2A%2A%2A%3A%20No%2FYes%0D%0A%0D%0A%2523%2523%20Notes%20and%20guidelines%0D%0A%0D%0A%5C%2A%20This%20is%20the%20identifier%20used%20to%20initialize%20Glean%20%28or%20the%20id%20used%20on%20the%20store%20on%20Android%20and%20Apple%20devices%29.%0D%0A%0D%0A%5C%2A%5C%2A%20Dependencies%20can%20be%20found%20%5Bin%20the%20Glean%20repositories%5D%28https%3A%2F%2Fprobeinfo.telemetry.mozilla.org%2Fv2%2Fglean%2Flibrary-variants%29.%20Each%20dependency%20must%20be%20listed%20explicitly.%20For%20example%2C%20the%20default%20Glean%20probes%20will%20only%20be%20included%20if%20glean%20itself%20is%20a%20dependency.%0D%0A%0D%0A%5C%2A%5C%2A%5C%2A%20Number%20of%20days%20that%20raw%20data%20will%20be%20retained.%20A%20good%20default%20is%20400.%20We%20can%20change%20this%20later%20to%20accommodate%20longer%20retention%20periods%2C%20though%20we%20cannot%20recover%20data%20that%20is%20past%20the%20retention%20period%20%28for%20example%2C%20we%20cannot%20recover%20data%20that%20is%20200%20days%20old%20if%20your%20retention%20period%20is%20180%20days%29.%0D%0A%0D%0A%5C%2A%5C%2A%5C%2A%5C%2A%20Depending%20on%20classification%2C%20access%20to%20data%20might%20need%20to%20be%20restricted%20in%20BigQuery.%20If%20this%20is%20the%20case%20and%20you%20know%20that%20should%20be%20used%20-%20note%20it.%0D%0A%0D%0A%2523%2523%20Need%20additional%20help%253F%0D%0AIf%20you%20need%20new%20dependencies%2C%20please%20file%20new%20bugs%20for%20them%2C%20separately%20from%20this%20one.%20For%20any%20questions%2C%20ask%20in%20the%20%2523glean%20channel.%0D%0A%0D%0A%2523%20To%20be%20filled%20by%20the%20Glean%20team%0D%0A%5B%2A%2AApplication%20friendly%20name%2A%2A%5D%28https%3A%2F%2Fmozilla.github.io%2Fprobe-scraper%2F%2523tag%2Fapplication%29%3A%20my_app_name&component=Glean%20Platform&contenttypemethod=list&contenttypeselection=text%2Fplain&defined_groups=1&filed_via=standard_form&flag_type-4=X&flag_type-607=X&flag_type-803=X&flag_type-864=X&flag_type-936=X&needinfo_role=other&needinfo_type=needinfo_from&op_sys=Unspecified&priority=--&product=Data%20Platform%20and%20Tools&rep_platform=Unspecified&short_desc=Enable%20new%20Glean%20App%20%60my.app_id%60&target_milestone=---&version=unspecified

[dataeng-bug-libraries]: https://bugzilla.mozilla.org/enter_bug.cgi?assigned_to=nobody%40mozilla.org&bug_ignored=0&bug_severity=--&bug_status=NEW&bug_type=task&cf_accessibility_severity=---&cf_fx_iteration=---&cf_fx_points=---&cf_status_firefox119=---&cf_status_firefox120=---&cf_status_firefox121=---&cf_tracking_firefox119=---&cf_tracking_firefox120=---&cf_tracking_firefox121=---&cf_tracking_firefox_relnote=---&comment=%23%20To%20be%20filled%20by%20the%20requester%0D%0A%0D%0A%2A%2ALibrary%20Canonical%20Name%2A%2A%3A%20My%20library%0D%0A%2A%2ADescription%2A%2A%3A%20Brief%20description%20of%20your%20library%0D%0A%2A%2AData-review%20response%20link%2A%2A%3A%20The%20link%20to%20the%20data%20response%20to%20the%20data%20collection%20request%20for%20adding%20Glean%20to%20your%20project%5C%2A.%0D%0A%2A%2ARepository%20URL%2A%2A%3A%20https%3A%2F%2Fgithub.com%2Fmozilla%2Fmy_app_name%0D%0A%2A%2ALocations%20of%20%60metrics.yaml%60%20files%20%28can%20be%20many%29%3A%2A%2A%0D%0A%20%20-%20src%2Fmetrics.yaml%0D%0A%0D%0A%2A%2ALocations%20of%20%60pings.yaml%60%20files%20%28can%20be%20many%29%3A%2A%2A%0D%0A%20-%20src%2Fpings.yaml%0D%0A%0D%0A%2A%2AData%20owners%3A%2A%2A%0D%0A-%20you%3F%0D%0A-%20anyone%20else%3F%0D%0A%0D%0A%2A%2ADependency%20of%2A%2A%3A%0D%0A%20-%20Fenix%0D%0A%0D%0A%23%23%20Notes%0D%0A%0D%0A%5C%2A%20These%20data-review%20requests%20need%20to%20be%20for%20the%20final%20product%20embedding%20the%20library%20and%20sending%20the%20data.%0D%0A%0D%0A%23%23%20Need%20additional%20help%3F%0D%0A%0D%0AFor%20any%20questions%2C%20ask%20in%20the%20%23glean%20channel.&component=Glean%20Platform&contenttypemethod=list&contenttypeselection=text%2Fplain&defined_groups=1&filed_via=standard_form&flag_type-4=X&flag_type-607=X&flag_type-800=X&flag_type-803=X&flag_type-864=X&flag_type-936=X&form_name=enter_bug&maketemplate=Remember%20values%20as%20bookmarkable%20template&op_sys=Unspecified&priority=--&product=Data%20Platform%20and%20Tools&rep_platform=Unspecified&short_desc=Enable%20new%20Glean%20library%20%60library-name%60&target_milestone=---&version=unspecified
