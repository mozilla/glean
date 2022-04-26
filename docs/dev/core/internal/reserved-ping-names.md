# Reserved ping names

The Glean SDK reserves all ping names in `send_in_pings` starting with `glean_`.

This currently includes, but is not limited to:

* `glean_client_info`: metrics sent with this ping are added to every ping in its `client_info` section;
* `glean_internal_info`: metrics sent with this ping are added to every ping in its `ping_info` section.

Additionally, only Glean may specify `all-pings`.  This special value has no effect in the client, but indicates to the backend infrastructure that a metric may appear in any ping.
