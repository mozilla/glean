##### `ping` _(default)_

The metric is cleared each time it is submitted in the ping. This is the most common case,
and should be used for metrics that are highly dynamic, such as things computed
in response to the user's interaction with the application.

##### `application`

The metric is related to an application run, and is cleared after the application restarts
and any Glean-owned ping, due at startup, is submitted. This should be used for things
that are constant during the run of an application, such as the operating system version.
In practice, these metrics are generally set during application startup.  A common mistake---
using the ping lifetime for these type of metrics---means that they will only be included
in the first ping sent during a particular run of the application.

##### `user`

**Reach out to the Glean team before using this.**

The metric is part of the user's profile and will live as long as the profile lives.
This is often not the best choice unless the metric records a value that _really_ needs
to be persisted for the full lifetime of the user profile, e.g. an identifier like the `client_id`,
the day the product was first executed. It is rare to use this lifetime outside of some metrics
that are built in to the Glean SDK.
