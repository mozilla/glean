/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

import Foundation

/// This class represents the structure of a timing distribution according to the pipeline schema. It
/// is meant to help serialize and deserialize data to the correct format for transport and storage,
/// as well as including a helper function to calculate the bucket sizes.
///
/// @param values a map containing the bucket index mapped to the accumulated count
/// @param sum the accumulated sum of all the samples in the timing distribution
public class DistributionData {
    let values: [UInt64: UInt64]
    let sum: UInt64

    /// Parse the distribution data from the given JSON string.
    ///
    /// If the string is not valid JSON or data is missing `nil` is returned.
    public init?(fromJson json: String) {
        do {
            let data = json.data(using: .utf8)!
            guard let content = try JSONSerialization.jsonObject(with: data, options: []) as? [String: Any] else {
                return nil
            }

            if content.isEmpty {
                return nil
            }

            if let mapData = content["values"] as? [String: UInt64] {
                var values = [UInt64: UInt64]()
                for (key, value) in mapData {
                    values[UInt64(key)!] = value
                }
                self.values = values
            } else {
                return nil
            }

            if let sum = content["sum"] as? UInt64 {
                self.sum = sum
            } else {
                return nil
            }
        } catch {
            return nil
        }
    }

    /// The total count of accumulated values.
    ///
    /// This is calculated from all recorded values.
    var count: UInt64 {
        values.reduce(0) { (acc, tuple: (_key: UInt64, value: UInt64)) in
            acc + tuple.value
        }
    }
}
