#[no_mangle]
pub extern "C" fn increment() -> u64 {
    glean_core::increment()
}
