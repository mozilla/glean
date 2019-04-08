use std::sync::RwLock;

use lazy_static::lazy_static;

pub mod metrics;

lazy_static! {
    static ref GLEAN_SINGLETON: RwLock<Glean> = RwLock::new(Glean::new());
}

#[derive(Debug)]
pub struct Glean;

impl Glean {
    fn new() -> Self {
        Self
    }

    pub fn singleton() -> &'static RwLock<Glean> {
        &*GLEAN_SINGLETON
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use metrics::MetricRecorder;

    #[test]
    fn it_works() {
        assert_eq!(false, metrics::flags::a11yEnabled.get());
        metrics::flags::a11yEnabled.set(true);
        assert_eq!(true, metrics::flags::a11yEnabled.get());
    }
}
