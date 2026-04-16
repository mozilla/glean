use glean_build::Builder;

fn main() {
    Builder::default()
        .file("metrics.yaml")
        .format("rust_sym")
        .generate()
        .expect("Error generating Glean Rust bindings");
}
