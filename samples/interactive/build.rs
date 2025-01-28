#![cfg_attr(rustfmt, rustfmt_skip)]

use std::io::Write;
use std::{
    env,
    fs::{self, File},
    path::PathBuf,
};

use quote::quote;

use glean_build::Builder;

struct Metric {
    name: String,
    typ: String,
}

struct Ping {
    name: String,
}

fn main() {
    Builder::default()
        .file("metrics.yaml")
        .file("pings.yaml")
        .generate()
        .expect("Error generating Glean Rust bindings");

    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let glean_metrics = out_dir.join("glean_metrics.rs");
    let content = fs::read_to_string(&glean_metrics).unwrap();

    let info_file_path = out_dir.join("metric_info.rs");
    let info_file = File::create(info_file_path).unwrap();

    let mut metrics = vec![];
    let mut pings = vec![];
    for line in content.lines() {
        if !line.contains("__export::Lazy") {
            continue;
        }
        if !line.contains("pub static") {
            continue;
        }

        if line.contains("Metric") {
            let mut components = line.trim().split(" ");

            components.next().unwrap();
            components.next().unwrap();
            let name = components.next().unwrap().trim_matches(':').to_string();
            let typ = components
                .next()
                .unwrap()
                .strip_prefix("::glean::private::__export::Lazy<")
                .unwrap()
                .strip_suffix(">")
                .unwrap()
                .to_string();
            metrics.push(Metric { name, typ });
        }

        if line.contains("PingType") {
            let mut components = line.trim().split(" ");

            components.next().unwrap();
            components.next().unwrap();
            let name = components.next().unwrap().trim_matches(':').to_string();

            pings.push(Ping { name });
        }
    }

    let metric_names = metrics.iter().map(|metric| &metric.name);
    let tokens = quote! {
        pub static METRICS: &[&str] = &[
            #(#metric_names),*
        ];
    };
    writeln!(&info_file, "{}", tokens).unwrap();

    let matcher = metrics.iter().map(|metric| {
        let name = &metric.name;
        if metric.typ.contains("Event") {
            quote! {}
        } else {
            let varname = proc_macro2::Ident::new(name, proc_macro2::Span::call_site());
            quote! {
                #name => {
                    println!("Value: {:?}", super::glean_metrics::metrics::#varname.test_get_value(ping.into()));
                }
            }
        }
    });
    let tokens = quote! {
        pub fn metric_get(name: &str, ping: Option<String>) {
            match name {
                #(#matcher)*
                other => eprintln!("unknown metric: {}", other),
            }
        }
    };
    writeln!(&info_file, "{}", tokens).unwrap();

    // COUNTER
    let matcher = metrics.iter().map(|metric| {
        let name = &metric.name;
        if !metric.typ.contains("Counter") || metric.typ.contains("LabeledMetric") {
            quote! {}
        } else {
            let varname = proc_macro2::Ident::new(name, proc_macro2::Span::call_site());
            quote! {
                #name => {
                    super::glean_metrics::metrics::#varname.add(amount);
                    println!("Value: {:?}", super::glean_metrics::metrics::#varname.test_get_value(None));
                }
            }
        }
    });
    let tokens = quote! {
        pub fn counter_add(name: &str, amount: i32) {
            match name {
                #(#matcher)*
                other => eprintln!("unknown metric: {}", other),
            }
        }
    };
    writeln!(&info_file, "{}", tokens).unwrap();

    // BOOLEAN
    let matcher = metrics.iter().map(|metric| {
        let name = &metric.name;
        if !metric.typ.contains("Boolean") || metric.typ.contains("LabeledMetric") {
            quote! {}
        } else {
            let varname = proc_macro2::Ident::new(name, proc_macro2::Span::call_site());
            quote! {
                #name => {
                    super::glean_metrics::metrics::#varname.set(val);
                    println!("Value: {:?}", super::glean_metrics::metrics::#varname.test_get_value(None));
                }
            }
        }
    });
    let tokens = quote! {
        pub fn boolean_set(name: &str, val: bool) {
            match name {
                #(#matcher)*
                other => eprintln!("unknown metric: {}", other),
            }
        }
    };
    writeln!(&info_file, "{}", tokens).unwrap();

    // EVENTS
    let matcher = metrics.iter().map(|metric| {
        let name = &metric.name;
        if !metric.typ.contains("EventMetric") {
            quote! {}
        } else {
            let varname = proc_macro2::Ident::new(name, proc_macro2::Span::call_site());
            quote! {
                #name => {
                    super::glean_metrics::metrics::#varname.record(None);
                    println!("Value: {:?}", super::glean_metrics::metrics::#varname.test_get_value(None));
                }
            }
        }
    });
    let tokens = quote! {
        pub fn event_record(name: &str) {
            match name {
                #(#matcher)*
                other => eprintln!("unknown metric: {}", other),
            }
        }
    };
    writeln!(&info_file, "{}", tokens).unwrap();


    // PINGS
    let ping_names = pings.iter().map(|ping| &ping.name);
    let tokens = quote! {
        pub static PINGS: &[&str] = &[
            #(#ping_names),*
        ];
    };
    writeln!(&info_file, "{}", tokens).unwrap();

    let register = pings.iter().map(|ping| {
        let name = &ping.name;
        let varname = proc_macro2::Ident::new(name, proc_macro2::Span::call_site());
        quote! {
            _ = &*super::glean_metrics::#varname;
        }
    });
    let tokens = quote! {
        pub fn register_pings() {
            #(#register)*
        }
    };
    writeln!(&info_file, "{}", tokens).unwrap();

    let matcher = pings.iter().map(|ping| {
        let name = &ping.name;
        let varname = proc_macro2::Ident::new(name, proc_macro2::Span::call_site());
        quote! {
            #name => {
                super::glean_metrics::#varname.submit(None);
                println!("{} submitted.", #name);
            }
        }
    });
    let tokens = quote! {
        pub fn ping_submit(name: &str) {
            match name {
                #(#matcher)*
                other => eprintln!("unknown ping: {}", other),
            }
        }
    };
    writeln!(&info_file, "{}", tokens).unwrap();
}
