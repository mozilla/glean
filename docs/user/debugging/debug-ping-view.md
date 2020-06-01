# Glean Debug Ping View

<!-- toc -->

## What is this good for?

The [Glean Debug Ping View](debug_view) enables you to easily see in real-time what data your application is sending.

This data is what actually arrives in our data pipeline, shown in a web
interface that is automatically updated when new data arrives. Any data sent from a Glean SDK-instrumented application usually shows up within 10 seconds,
updating the pages automatically. Pings are retained for 3 weeks.

[debug_view]: https://debug-ping-preview.firebaseapp.com/

## What setup is needed for applications?

You need to tag each ping with a debugging tag. See the documentation on
[debugging](./index.md) for information on how to do this for each platform and/or language.

## Troubleshooting

If nothing is showing up on the dashboard, try checking the following:

- If you see _”Glean must be enabled before sending pings.”_ in the logs,
  then the application has disabled Glean. Check with the application author
  on how to re-enable it.
- If no error is reported when triggering tagged pings, but the data won't
  show up on the dashboard, check if the used `<application-id>` is the same
  expected by the Glean pipeline (i.e. the one used to publish the
  application on the Play Store).
