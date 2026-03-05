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
    }
}

unsafe impl FfiConverter<crate::UniFfiTag> for () {
    type FfiType = ();

    fn lower(_obj: Self) -> Self::FfiType {
        ()
    }

    fn try_lift(_v: Self::FfiType) -> uniffi::Result<Self> {
        Ok(())
    }

    fn write(_obj: Self, _buf: &mut Vec<u8>) {
        ()
    }

    fn try_read(_buf: &mut &[u8]) -> uniffi::Result<Self> {
        Ok(())
    }

    const TYPE_ID_META: uniffi::MetadataBuffer = uniffi::MetadataBuffer::from_code(0);
}

forward_ffi_converter!(Option<String>);
forward_ffi_converter!(Vec<String>);
forward_ffi_converter!(Vec<i64>);
forward_ffi_converter!(Option<types::DistributionData>);
forward_ffi_converter!(Option<types::Datetime>);
forward_ffi_converter!(Option<Vec<String>>);
forward_ffi_converter!(Option<Vec<types::RecordedEvent>>);
forward_ffi_converter!(Option<types::Rate>);
forward_ffi_converter!(HashMap<String, String>);
forward_ffi_converter!(Option<HashMap<String, HashMap<String, i32>>>);
forward_ffi_converter!(Option<i8>);
forward_ffi_converter!(Option<i32>);
forward_ffi_converter!(Option<i64>);

pub trait CloneFfiArg<T> {
    fn clone_for_ffi(&self) -> T;
}

macro_rules! impl_clone_ffi_arg_primitive {
    ($($ty:ty),+) => {
        $(
        impl CloneFfiArg<$ty> for $ty {
            fn clone_for_ffi(&self) -> $ty {
                *self
            }
        }
        )*
    }
}

impl_clone_ffi_arg_primitive!(i8, i32, i64);

#[repr(C)]
#[derive(Debug)]
struct LocalRustBuffer {
    /// The allocated capacity of the underlying `Vec<u8>`.
    /// In Rust this is a `usize`, but we use an `u64` to keep the foreign binding code simple.
    pub(crate) capacity: u64,
    /// The occupied length of the underlying `Vec<u8>`.
    /// In Rust this is a `usize`, but we use an `u64` to keep the foreign binding code simple.
    pub(crate) len: u64,
    /// The pointer to the allocated buffer of the `Vec<u8>`.
    pub(crate) data: *mut u8,
}

impl CloneFfiArg<RustBuffer> for RustBuffer {
    fn clone_for_ffi(&self) -> RustBuffer {
        // SAFETY: Same layout.
        unsafe {
            std::mem::transmute(LocalRustBuffer {
                capacity: self.capacity() as _,
                len: self.len() as _,
                data: self.data_pointer() as _,
            })
        }
    }
}
