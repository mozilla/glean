// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.

namespace Mozilla.Glean.Testing
{
    /// <summary>
    /// Different types of errors that can be reported through Glean's error reporting metrics.
    /// </summary>
    public enum ErrorType
    {
        /// <summary>
        /// For when the value to be recorded does not match the metric-specific restrictions
        /// </summary>
        InvalidValue,

        /// <summary>
        /// For when the label of a labeled metric does not match the restrictions
        /// </summary>
        InvalidLabel,

        /// <summary>
        /// For when timings are recorded incorrectly
        /// </summary>
        InvalidState,

        /// <summary>
        /// For when the value to be recorded overflows the metric-specific upper range
        /// </summary>
        InvalidOverflow
    }
}
