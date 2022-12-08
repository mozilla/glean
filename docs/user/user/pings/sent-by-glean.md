# Pings sent by Glean

If data collection is enabled, the Glean SDKs provide a set of built-in pings that are assembled out of the box without any developer intervention.  The following is a list of these built-in pings:

- [`baseline` ping](baseline.md): A small ping sent every time the application goes to foreground and background. Going to foreground also includes when the application starts.
- [`deletion-request` ping](deletion-request.md): Sent when the user disables telemetry in order to request a deletion of their data.
- [`events` ping](events.md): The default ping for events. Sent every time the application goes to background or a certain number of events is reached.
  Is not sent when there are no events recorded, even if there are other metrics with values.
- [`metrics` ping](metrics.md): The default ping for metrics. Sent approximately daily.

Applications can also define and send their own [custom pings](custom.md) when the schedules of these pings is not suitable.

There is also a [high-level overview](ping-schedules-and-timings.html) of how the `metrics` and `baseline` pings relate and the timings they record.

### Available pings per platform

| SDK | [`baseline`](baseline.md) | [`deletion-request`](deletion-request.md) | [`events`](events.md) | [`metrics`](metrics.md) |
|-:|:-:|:-:|:-:|:-:|
| Kotlin | ✅ | ✅ | ✅ | ✅ |
| Swift | ✅ | ✅ | ✅ | ✅ |
| Python | ✅ [^1] | ✅ | ✅ [^2] | ❌ |
| Rust | ✅ | ✅ | ✅ | ✅ |
| JavaScript | ❌ | ✅ | ✅ | ❌ |
| Firefox Desktop | ✅ | ✅ | ✅ | ✅ |

[^1]: Not sent automatically. Use the [`handle_client_active`](../../../python/glean/#glean.Glean.handle_client_active) and [`handle_client_inactive`](../../../python/glean/#glean.Glean.handle_client_inactive) API.

[^2]: Sent on startup when pending events are stored. Additionally sent when [`handle_client_inactive`](../../../python/glean/#glean.Glean.handle_client_inactive) is called.

## Defining foreground and background state

These docs refer to application 'foreground' and 'background' state in several places.

{{#include ../../../shared/tab_header.md}}

<div data-lang="Kotlin" class="tab">

### Foreground

For Android, this specifically means the activity becomes visible to the user, it has entered the `Started` state, and the system invokes the [`onStart()`](https://developer.android.com/reference/android/app/Activity.html#onStart()) callback.

### Background

This specifically means when the activity is no longer visible to the user, it has entered the `Stopped` state, and the system invokes the [`onStop()`](https://developer.android.com/reference/android/app/Activity.html#onStop()) callback.

This may occur, if the user uses `Overview` button to change to another app, the user presses the `Back` button and
navigates to a previous application or the home screen, or if the user presses the `Home` button to return to the
home screen.  This can also occur if the user navigates away from the application through some notification or
other means.

The system may also call `onStop()` when the activity has finished running, and is about to be terminated.

</div>

<div data-lang="Swift" class="tab">

### Foreground

For iOS, the Glean Swift SDK attaches to the [`willEnterForegroundNotification`](https://developer.apple.com/documentation/uikit/uiapplication/1622944-willenterforegroundnotification).
This notification is posted by the OS shortly before an app leaves the background state on its way to becoming the active app.

### Background

For iOS, this specifically means when the app is no longer visible to the user, or when the `UIApplicationDelegate`
receives the [`applicationDidEnterBackground`](https://developer.apple.com/documentation/uikit/uiapplicationdelegate/1622997-applicationdidenterbackground) event.

This may occur if the user opens the task switcher to change to another app, or if the user presses the `Home` button
to show the home screen.  This can also occur if the user navigates away from the app through a notification or other
means.

> **Note:** Glean does not currently support [Scene based lifecycle events](https://developer.apple.com/documentation/uikit/app_and_environment/managing_your_app_s_life_cycle) that were introduced in iOS 13.

</div>

{{#include ../../../shared/tab_footer.md}}
