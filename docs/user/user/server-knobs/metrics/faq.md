# Frequently Asked Questions

* How can I tell if a given client id has the metric X on?

Once we have established the functionality behind the data control plane, a dashboard for monitoring this will be provided. Details are to be determined.

* Why isn't some client id reporting the metric that should be enabled for all the clients for that channel? (e.g. Some fraction of population may get stuck on “default” config)

Nimbus must be able to both reach and supply a valid configuration to the audience. For some outliers this doesn't work and so may be "unreachable" at times.
