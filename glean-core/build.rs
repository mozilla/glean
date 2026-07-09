fn main() {
    uniffi::generate_scaffolding("./src/glean.udl").unwrap();
    #[cfg(feature = "mozbuild-rustlib")]
    if std::env::var_os("MOZ_TOPOBJDIR").is_some() {
        mozbuild::link_sqlite();
    }
}
