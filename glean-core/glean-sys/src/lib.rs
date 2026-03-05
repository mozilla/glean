use std::sync::LazyLock;

macro_rules! library_binding {
    ( $localname:ident members[$($members:tt)*] load[$($load:tt)*] fn $name:ident $args:tt $( -> $ret:ty )? ; $($rest:tt)* ) => {
        library_binding! {
            $localname
            members[
                $($members)*
                $name: libloading::Symbol<'static, unsafe extern "C" fn $args $(->$ret)?>,
            ]
            load[
                $($load)*
                $name: unsafe {
                    let symbol = $localname.get::<unsafe extern "C" fn $args $(->$ret)?>(stringify!($name).as_bytes())
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::NotFound, e))?;
                    // All symbols refer to library, so `'static` lifetimes are safe (`library`
                    // will outlive them).
                    std::mem::transmute(symbol)
                },
            ]
            $($rest)*
        }
    };
    ( $localname:ident members[$($members:tt)*] load[$($load:tt)*] ) => {
        pub struct GleanSys {
            $($members)*
            _library: libloading::Library
        }

        impl GleanSys {
            pub fn load() -> std::io::Result<Self> {
                // Try each of the libraries, debug-logging load failures.
                let library = crate::GLEAN_LIB_NAMES.iter().find_map(|&name| {
                    log::debug!("attempting to load {name}");
                    match unsafe { libloading::Library::new(name) } {
                        Ok(lib) => {
                            log::info!("loaded {name}");
                            Some(lib)
                        }
                        Err(e) => {
                            log::debug!("error when loading {name}: {e}");
                            None
                        }
                    }
                });

                let $localname = library.ok_or_else(|| {
                    std::io::Error::new(std::io::ErrorKind::NotFound, "failed to find glean library")
                })?;

                Ok(GleanSys { $($load)* _library: $localname })
            }
        }
    };
    ( $($rest:tt)* ) => {
        library_binding! {
            library members[] load[] $($rest)*
        }
    }
}

mod types;
mod util;
mod metrics;
use metrics::*;
use types::*;

struct UniFfiTag;

const GLEAN_LIB_NAMES: &[&str] = if cfg!(target_os = "linux") {
    &["libglean_ffi.so"]
} else if cfg!(target_os = "macos") {
    &["libglean_ffi.dylib"]
} else if cfg!(target_os = "windows") {
    &["libglean_ffi.dll"]
} else {
    &[]
};

static GLEAN: LazyLock<GleanSys> = LazyLock::new(|| {
    metrics::GleanSys::load().unwrap()
});

#[unsafe(no_mangle)]
extern "C" fn record_cat_name() {
    env_logger::init();
    _ = &*GLEAN;

    let cmd = CommonMetricData {
        category: "cat".to_string(),
        name: "name".to_string(),
        send_in_pings: vec!["metrics".to_string()],
        lifetime: Lifetime::Ping,
        disabled: false,
        dynamic_label: None,
    };
    let metric = CounterMetric::new(cmd);
    metric.add(31);
    let value = metric.test_get_value(None);
    dbg!(value);

    println!("Hello, world!");
}
