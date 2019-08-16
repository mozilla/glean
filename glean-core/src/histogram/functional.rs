// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use super::{Bucketing, Histogram};

// Maximum time of 10 minutes in nanoseconds. This maximum means we
// retain a maximum of 313 buckets.
const MAX_SAMPLE_TIME: u64 = 1000 * 1000 * 1000 * 60 * 10;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Functional {
    exponent: f64,
}

impl Functional {
    fn new(log_base: f64, buckets_per_magnitutde: f64) -> Functional {
        let exponent = log_base.powf(1.0 / buckets_per_magnitutde);

        Functional { exponent }
    }

    fn sample_to_bucket_index(&self, sample: u64) -> u64 {
        ((sample + 1) as f64).log(self.exponent) as u64
    }

    fn bucket_index_to_bucket_minimum(&self, index: u64) -> u64 {
        self.exponent.powf(index as f64) as u64
    }
}

impl Bucketing for Functional {
    fn sample_to_bucket_minimum(&self, sample: u64) -> u64 {
        if sample == 0 {
            return 0;
        }

        let index = self.sample_to_bucket_index(sample);
        self.bucket_index_to_bucket_minimum(index)
    }
}

impl Histogram<Functional> {
    /// Create a histogram with functional buckets.
    pub fn functional(log_base: f64, buckets_per_magnitutde: f64) -> Histogram<Functional> {
        Histogram {
            values: HashMap::new(),
            count: 0,
            sum: 0,
            bucketing: Functional::new(log_base, buckets_per_magnitutde),
        }
    }

    /// Get a snapshot of _all_ values.
    pub fn snapshot_values(&self) -> HashMap<u64, u64> {
        if self.values.is_empty() {
            return HashMap::new();
        }

        let mut min_key = None;
        let mut max_key = None;

        for key in self.values.keys() {
            let key = *key;

            if min_key.is_none() || key < min_key.unwrap() {
                min_key = Some(key);
            }

            if max_key.is_none() || key > max_key.unwrap() {
                max_key = Some(key);
            }
        }

        // Non-empty values, therefore minimum/maximum exists.
        let min_bucket = self.bucketing.sample_to_bucket_index(min_key.unwrap());
        let max_bucket = self.bucketing.sample_to_bucket_index(max_key.unwrap()) + 1;

        let mut values = self.values.clone();

        for idx in min_bucket..=max_bucket {
            // Fill in missing entries.
            let min_bucket = self.bucketing.bucket_index_to_bucket_minimum(idx);
            let _ = values.entry(min_bucket).or_insert(0);
        }

        values
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_count() {
        let mut hist = Histogram::functional(2.0, 8.0);
        assert!(hist.is_empty());

        for i in 1..=10 {
            hist.accumulate(i);
        }

        assert_eq!(10, hist.count());
        assert_eq!(55, hist.sum());
    }

    #[test]
    fn sample_to_bucket_minimum_correctly_rounds_down() {
        let hist = Histogram::functional(2.0, 8.0);

        // Check each of the first 100 integers, where numerical accuracy of the round-tripping
        // is most potentially problematic
        for value in 0..100 {
            let bucket_minimum = hist.bucketing.sample_to_bucket_minimum(value);
            assert!(bucket_minimum <= value);

            assert_eq!(
                bucket_minimum,
                hist.bucketing.sample_to_bucket_minimum(bucket_minimum)
            );
        }

        // Do an exponential sampling of higher numbers
        for i in 11..500 {
            let value = 1.5f64.powi(i);
            let value = value as u64;
            let bucket_minimum = hist.bucketing.sample_to_bucket_minimum(value);
            assert!(bucket_minimum <= value);
            assert_eq!(
                bucket_minimum,
                hist.bucketing.sample_to_bucket_minimum(bucket_minimum)
            );
        }
    }
}
