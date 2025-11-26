/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

// Reexport of internal enums and classes for use in the public API.

package mozilla.telemetry.glean.private

/**
 * The common set of data shared across all different metric types.
 */
typealias CommonMetricData = mozilla.telemetry.glean.internal.CommonMetricData

/**
 * Representation of a date, time and timezone.
 */
typealias Datetime = mozilla.telemetry.glean.internal.Datetime

/**
 * Enumeration of the different kinds of histograms supported by metrics based on histograms.
 */
typealias HistogramType = mozilla.telemetry.glean.internal.HistogramType

/**
 * Enumeration of different metric lifetimes.
 */
typealias Lifetime = mozilla.telemetry.glean.internal.Lifetime

/**
 * Enumeration of different resolutions supported by the MemoryDistribution metric type.
 *
 * These use the power-of-2 values of these units, that is, Kilobyte is pedantically a Kibibyte.
 */
typealias MemoryUnit = mozilla.telemetry.glean.internal.MemoryUnit

/**
 * Enumeration of different resolutions supported by
 * the Timespan and DateTime metric types.
 */
typealias TimeUnit = mozilla.telemetry.glean.internal.TimeUnit

/*
 * Represents the recorded data for a single event.
 */
typealias RecordedEvent = mozilla.telemetry.glean.internal.RecordedEvent

/**
 * Deserialized experiment data.
 */
typealias RecordedExperiment = mozilla.telemetry.glean.internal.RecordedExperiment

/*
 * A rate value as given by its numerator and denominator.
 */
typealias Rate = mozilla.telemetry.glean.internal.Rate

/**
 * The set of data needed to construct labeled metric types.
 */
typealias LabeledMetricData = mozilla.telemetry.glean.internal.LabeledMetricData

/**
 * The set of data specifically needed to construct simple labeled metric types.
 */
typealias CommonLabeledMetricData = mozilla.telemetry.glean.internal.LabeledMetricData.Common
