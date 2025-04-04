# Clearing metrics when disabling/enabling Glean

When disabling upload (`Glean.setCollectionEnabled(false)`), metrics are also cleared to prevent their storage on the local device, and lessen the likelihood
of accidentally sending them.
There is an exceptions to this:

- `first_run_date` is retained so it isn't reset if metrics are later re-enabled.

When re-enabling metrics:

- `first_run_date` is left as-is. (It should remain a correct time of first run of the application, unaffected by disabling/enabling the Glean SDK).

- The `client_id` is set to a newly-generated random UUID. It has no connection to the `client_id` used prior to disabling the Glean SDK.

- Application lifetime metrics owned by Glean are regenerated from scratch so that they will appear in subsequent pings. This is the same process that happens during every startup of the application when the Glean SDK is enabled. The application is also responsible for setting any application lifetime metrics that it manages at this time.

- Ping lifetime metrics do not need special handling.  They will begin recording again after metric uploading is re-enabled.

