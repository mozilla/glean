use glean_build::Builder;

fn main() {
    Builder::default()
        .file("metrics.yaml")
        .file("pings.yaml")
        .generate()
        .expect("Error generating Glean Rust bindings");
}
