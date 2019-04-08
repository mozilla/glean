#![allow(non_upper_case_globals)]

use std::sync::RwLock;

pub struct BooleanMetric {
    value: bool,
}

pub trait MetricRecorder {
    fn set(&self, value: bool);
    fn get(&self) -> bool;
}

impl MetricRecorder for RwLock<BooleanMetric> {
    fn set(&self, value: bool) {
        let mut lock = self.write().unwrap();
        lock.value = value;
    }

    fn get(&self) -> bool {
        self.read().unwrap().value
    }
}

impl BooleanMetric {
    pub fn new() -> Self {
        Self { value: false }
    }

    pub fn set(&mut self, value: bool) {
        self.value = value;
    }
}

pub mod flags {
    use super::BooleanMetric;
    use std::sync::RwLock;
    use lazy_static::lazy_static;

    lazy_static! {
        pub static ref a11yEnabled : RwLock<BooleanMetric> = RwLock::new(BooleanMetric::new());
    }
}
