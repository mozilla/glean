// via https://github.com/rust-lang/rust/commit/07633821ed63360d4d7464998c29f4f588930a03
// Shim for bcryptprimitives.dll. The Wine version shipped with Ubuntu 22.04
// doesn't support it yet. Authored by @ChrisDenton

#![crate_type = "cdylib"]
#![allow(nonstandard_style)]

#[no_mangle]
pub unsafe extern "system" fn ProcessPrng(mut pbData: *mut u8, mut cbData: usize) -> i32 {
    while cbData > 0 {
        let size = core::cmp::min(cbData, u32::MAX as usize);
        RtlGenRandom(pbData, size as u32);
        cbData -= size;
        pbData = pbData.add(size);
    }
    1
}

#[link(name = "advapi32")]
extern "system" {
    #[link_name = "SystemFunction036"]
    pub fn RtlGenRandom(RandomBuffer: *mut u8, RandomBufferLength: u32) -> u8;
}
