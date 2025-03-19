## GeckoView Content Processes and Glean

This is a brief overview of how GeckoView content processes record data in Glean, despite Glean being required to be called only from the main process.

### GeckoView Content Processes

These are processes in which web page's content is rendered and data is collected such as [use counters](https://firefox-source-docs.mozilla.org/dom/use-counters.html).

Content processes are [launched as a "child" of the main GeckoView UI process](https://firefox-source-docs.mozilla.org/dom/ipc/process_model.html).

### Recording Telemetry in Content Processes

Content processes communicate to the main GeckoView process through Inter-Thread and Inter-Process Message Passing ([IPDL](https://firefox-source-docs.mozilla.org/ipc/ipdl.html). Because Glean doesn't know about processes, it requires only a single process to interact with the API in order to work properly so content processes record data on the main process through IPDL, specifically through the Firefox on Glean (FOG) [IPC implementation](https://firefox-source-docs.mozilla.org/toolkit/components/glean/index.html).

As long as processes within gecko make use of these mechanisms, data recorded in them should make it into Glean whether it is running in Desktop or Android.

