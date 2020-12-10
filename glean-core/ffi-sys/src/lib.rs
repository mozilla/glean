// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(clippy::style)]
#![allow(clippy::too_many_arguments)]
#![allow(clippy::transmute_ptr_to_ptr)]

// Re-import/export some enums.
//
// `bindgen` butchers their defintions by having both the enum and a typedef,
// thus breaking the build.
// `glean-core` already defines them and marks them `repr(i32)`, therefore FFI-compatible.
pub use glean_core::{
    metrics::{MemoryUnit, TimeUnit},
    Lifetime,
};

/// From `glean-ffi`'s `upload.rs`.
///
/// Replicated here because `bindgen` does the wrong thing.
#[repr(u8)]
#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum FfiPingUploadTask_Tag {
    Upload = 0,
    Wait = 1,
    Done = 2,
}

include!("./bindings.rs");
