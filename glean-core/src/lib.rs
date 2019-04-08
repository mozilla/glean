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

    #[test]
    fn it_works() {
        metrics::flags::a11yEnabled.set(true);
        metrics::app::clientId.set("c0ffee".into());

        assert_eq!("", metrics::StorageManager.dump());
    }
}
