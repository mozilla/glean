/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/// This implements the developer facing API for recording memory distribution metrics.
///
/// Instances of this class type are automatically generated by the parsers at build time,
/// allowing developers to record values that were previously registered in the metrics.yaml file.
///
/// The memory distribution API only exposes the `MemoryDistributionMetricType.accumulate()` method.
public typealias MemoryDistributionMetricType = MemoryDistributionMetric

extension MemoryDistributionMetric {
    public func testHasValue(_ pingName: String? = nil) -> Bool {
        return self.testGetValue(pingName) != nil
    }
}