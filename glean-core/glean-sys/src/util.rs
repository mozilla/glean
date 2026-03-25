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
uniffi::derive_ffi_traits!(local types::CowString);

pub trait CloneFfiArg<T> {
    fn clone_for_ffi(&self) -> T;
}

pub trait DestroyFfiArg {
    fn destroy(self);
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
        )*
    }
}

impl_clone_ffi_arg_primitive!(i8, i32, i64);

impl CloneFfiArg<RustBuffer> for RustBuffer {
    fn clone_for_ffi(&self) -> RustBuffer {
        // SAFETY: trust me, bro.
        unsafe {
            let bytes = uniffi::ForeignBytes::from_raw_parts(self.data_pointer(), self.len().try_into().unwrap());
            let mut call_status = uniffi::RustCallStatus::default();
            let buffer = (crate::GLEAN.ffi_glean_core_rustbuffer_from_bytes)(
                bytes,
                &mut call_status,
            );
            buffer
        }
    }
}

impl DestroyFfiArg for RustBuffer {
    fn destroy(self) {
        _ = self.destroy_into_vec();
    }
}
