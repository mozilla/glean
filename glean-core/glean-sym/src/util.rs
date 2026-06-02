// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use std::collections::HashMap;

use uniffi::FfiConverter;
use uniffi::RustBuffer;

use super::types;

macro_rules! forward_ffi_converter {
    ($ty:ty) => {
        unsafe impl FfiConverter<crate::UniFfiTag> for $ty {
            type FfiType = RustBuffer;

            fn lower(obj: Self) -> Self::FfiType {
                uniffi::Lower::<crate::UniFfiTag>::lower(obj)
            }

            fn try_lift(v: Self::FfiType) -> uniffi::Result<Self> {
                uniffi::Lift::<crate::UniFfiTag>::try_lift(v)
            }

            fn write(obj: Self, buf: &mut Vec<u8>) {
                uniffi::Lower::<crate::UniFfiTag>::write(obj, buf)
            }

            fn try_read(buf: &mut &[u8]) -> uniffi::Result<Self> {
                uniffi::Lift::<crate::UniFfiTag>::try_read(buf)
            }

            const TYPE_ID_META: uniffi::MetadataBuffer = uniffi::MetadataBuffer::from_code(0);
        }
    };
}

unsafe impl FfiConverter<crate::UniFfiTag> for () {
    type FfiType = ();

    fn lower(_obj: Self) -> Self::FfiType {}

    fn try_lift(_v: Self::FfiType) -> uniffi::Result<Self> {
        Ok(())
    }

    fn write(_obj: Self, _buf: &mut Vec<u8>) {}

    fn try_read(_buf: &mut &[u8]) -> uniffi::Result<Self> {
        Ok(())
    }

    const TYPE_ID_META: uniffi::MetadataBuffer = uniffi::MetadataBuffer::from_code(0);
}

unsafe impl FfiConverter<crate::UniFfiTag> for types::CowString {
    type FfiType = <String as ::uniffi::Lower<crate::UniFfiTag>>::FfiType;
    fn lower(s: types::CowString) -> Self::FfiType {
        <String as ::uniffi::Lower<crate::UniFfiTag>>::lower(s.into_owned())
    }
    fn try_lift(v: Self::FfiType) -> ::uniffi::Result<types::CowString> {
        let s = <String as ::uniffi::Lift<crate::UniFfiTag>>::try_lift(v)?;
        Ok(types::CowString::from(s))
    }

    fn write(s: types::CowString, buf: &mut Vec<u8>) {
        <String as ::uniffi::Lower<crate::UniFfiTag>>::write(s.into_owned(), buf);
    }

    fn try_read(buf: &mut &[u8]) -> ::uniffi::Result<types::CowString> {
        let s = <String as ::uniffi::Lift<crate::UniFfiTag>>::try_read(buf)?;
        Ok(types::CowString::from(s))
    }

    const TYPE_ID_META: uniffi::MetadataBuffer = uniffi::MetadataBuffer::from_code(0);
}

forward_ffi_converter!(Option<String>);
forward_ffi_converter!(Vec<String>);
forward_ffi_converter!(Vec<i64>);
forward_ffi_converter!(Vec<types::CommonMetricData>);
forward_ffi_converter!(Option<types::DistributionData>);
forward_ffi_converter!(Option<types::Datetime>);
forward_ffi_converter!(Option<Vec<String>>);
forward_ffi_converter!(Option<Vec<types::RecordedEvent>>);
forward_ffi_converter!(Option<types::Rate>);
forward_ffi_converter!(Option<Vec<types::CowString>>);
forward_ffi_converter!(HashMap<String, String>);
forward_ffi_converter!(Option<HashMap<String, HashMap<String, i32>>>);
forward_ffi_converter!(Option<i8>);
forward_ffi_converter!(Option<i32>);
forward_ffi_converter!(Option<i64>);
forward_ffi_converter!(Option<bool>);
uniffi::derive_ffi_traits!(local types::CowString);

pub trait CloneFfiArg<T> {
    fn clone_for_ffi(&self) -> T;
}

pub trait DestroyFfiArg {
    fn destroy(self);
}

/// Consume a remote type.
///
/// This might clone the data and drop self using the FFI.
trait ConsumeRemoteType {
    fn consume(self) -> Self;
}

/// Wrapper around `FfiConverter::try_lift` to add `ConsumeRemoteType` functionality.
pub trait LocalTryLift: Sized {
    type FfiType;

    fn try_lift(v: Self::FfiType) -> Result<Self, uniffi::deps::anyhow::Error>;
}

macro_rules! impl_clone_ffi_arg_primitive {
    ($($ty:ty),+) => {
        $(
        impl CloneFfiArg<$ty> for $ty {
            fn clone_for_ffi(&self) -> $ty {
                *self
            }
        }

        impl DestroyFfiArg for $ty {
            fn destroy(self) {
                /* left empty */
            }
        }

        impl ConsumeRemoteType for $ty {
            fn consume(self) -> Self {
                self
            }
        }
        )*
    }
}

impl_clone_ffi_arg_primitive!(i8, i32, i64);

impl CloneFfiArg<RustBuffer> for RustBuffer {
    fn clone_for_ffi(&self) -> RustBuffer {
        // SAFETY:
        // * `ForeignBytes` is a UniFFI type and `repr(C)` of a `len: i32` and `data: *const u8`.
        // * `RustBuffer` is a UniFFI type, `repr(C)` and has a `capacity`, `len` and `data`.
        // * `data_pointer` gives us a valid pointer to the underlying allocated data (backed by a `Vec<u8>`)
        // * We assert that the length fits in `i32`.
        // * The data pointer of the `RustBuffer` is valid while we're in this call. We can pass it over.
        // * `RustBuffer`'s `from_bytes` copies the bytes from the passed data pointer into a new allocation
        // * and wraps that allocation into a new `RustBuffer`.
        debug_assert!(self.len() <= i32::MAX as usize);
        unsafe {
            let bytes = uniffi::ForeignBytes::from_raw_parts(
                self.data_pointer(),
                self.len().try_into().unwrap(),
            );
            let mut call_status = uniffi::RustCallStatus::default();
            (crate::GLEAN.ffi_glean_core_rustbuffer_from_bytes)(bytes, &mut call_status)
        }
    }
}

impl DestroyFfiArg for RustBuffer {
    fn destroy(self) {
        _ = self.destroy_into_vec();
    }
}

impl ConsumeRemoteType for () {
    fn consume(self) -> Self {
        /* intentionally left empty */
    }
}

/// A remote `RustBuffer` is consumed by
///
/// 1) Cloning the data into a local `RustBuffer`
/// 2) Freeing the remote `RustBuffer` using `ffi_glean_core_rustbuffer_free`.
impl ConsumeRemoteType for RustBuffer {
    fn consume(self) -> Self {
        let src = self.data_pointer();
        let reslen = self.len();

        let mut newvec = Vec::with_capacity(reslen);

        // SAFETY:
        // * `src` is a newly created `Vec` with `reslen` capacity.
        // * `dst` is a `Vec` pointer with size `reslen`
        // * Both `src` (newly created `Vec`) and `dst` (a pointer obtained from a `Vec`)
        //   are correctly aligned.
        // * `src` and `dst` are definitely separate `Vec`s, thus not overlapping.
        unsafe {
            let dst = newvec.as_mut_ptr();
            std::ptr::copy_nonoverlapping(src, dst, reslen);
            newvec.set_len(reslen);
        }

        let mut call_status = uniffi::RustCallStatus::default();

        // SAFETY:
        // * `call_status` is an empty status.
        // * `ffi_glean_core_rustbuffer_free` is generated by `UniFFI`
        //   and accepts the `repr(C)` `RustBuffer` that we previously got via FFI.
        unsafe {
            (crate::GLEAN.ffi_glean_core_rustbuffer_free)(self, &mut call_status);
        }

        uniffi::RustBuffer::from_vec(newvec)
    }
}

/// Forwards `try_lift` to `FfiConverter::try_lift`, but consumes any remote type for local usage.
impl<T> LocalTryLift for T
where
    T: FfiConverter<crate::UniFfiTag>,
    <T as FfiConverter<crate::UniFfiTag>>::FfiType: ConsumeRemoteType,
{
    type FfiType = <T as FfiConverter<crate::UniFfiTag>>::FfiType;

    fn try_lift(v: Self::FfiType) -> Result<Self, uniffi::deps::anyhow::Error> {
        let v = v.consume();
        uniffi::FfiConverter::<crate::UniFfiTag>::try_lift(v)
    }
}
