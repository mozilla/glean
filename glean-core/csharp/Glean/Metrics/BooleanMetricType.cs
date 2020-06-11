// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

using Mozilla.Glean.FFI;
using System;

namespace Mozilla.Glean.Private
{
    public sealed class BooleanMetricType
    {
        private bool disabled;
        private string[] sendInPings;
        private LibGleanFFI.BooleanMetricTypeHandle handle;

        public BooleanMetricType(
            bool disabled,
            string category,
            Lifetime lifetime,
            string name,
            string[] sendInPings
            ) : this(new LibGleanFFI.BooleanMetricTypeHandle(), disabled, sendInPings)
        {
            handle = LibGleanFFI.glean_new_boolean_metric(
                        category: category,
                        name: name,
                        send_in_pings: sendInPings,
                        send_in_pings_len: sendInPings.Length,
                        lifetime: (int)lifetime,
                        disabled: disabled);
        }

        internal BooleanMetricType(
            LibGleanFFI.BooleanMetricTypeHandle handle,
            bool disabled,
            string[] sendInPings
            )
        {
            this.disabled = disabled;
            this.sendInPings = sendInPings;
            this.handle = handle;
        }

        /// <summary>
        /// Set a boolean value.
        /// </summary>
        /// <param name="value"> This is a user defined boolean value.
        public void Set(bool value)
        {
            if (disabled)
            {
                return;
            }

            Dispatchers.LaunchAPI(() => {
                SetSync(value);
            });
        }

        /// <summary>
        /// Internal only, synchronous API for setting a boolean value.
        /// </summary>
        /// <param name="value">This is a user defined boolean value.</param>
        internal void SetSync(bool value)
        {
            LibGleanFFI.glean_boolean_set(this.handle, Convert.ToByte(value));
        }


        /// <summary>
        /// Tests whether a value is stored for the metric for testing purposes only. This function will
        /// attempt to await the last task (if any) writing to the the metric's storage engine before
        /// returning a value.
        /// </summary>
        /// <param name="pingName">represents the name of the ping to retrieve the metric for Defaults
        /// to the first value in `sendInPings`</param>
        /// <returns>true if metric value exists, otherwise false</returns>
        public bool TestHasValue(string pingName = null)
        {
            Dispatchers.AssertInTestingMode();

            string ping = pingName ?? sendInPings[0];
            return LibGleanFFI.glean_boolean_test_has_value(this.handle, ping) != 0;
        }

        /// <summary>
        /// Returns the stored value for testing purposes only. This function will attempt to await the
        /// last task (if any) writing to the the metric's storage engine before returning a value.
        /// @throws [NullPointerException] if no value is stored
        /// </summary>
        /// <param name="pingName">represents the name of the ping to retrieve the metric for.
        /// Defaults to the first value in `sendInPings`</param>
        /// <returns>value of the stored metric</returns>
        /// <exception cref="System.NullReferenceException">Thrown when the metric contains no value</exception>
        public bool TestGetValue(string pingName = null)
        {
            Dispatchers.AssertInTestingMode();

            if (!TestHasValue(pingName))
            {
                throw new NullReferenceException();
            }

            string ping = pingName ?? sendInPings[0];
            return LibGleanFFI.glean_boolean_test_get_value(this.handle, ping) != 0;
        }
    }
}
