// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

use proc_macro2::TokenStream;
use quote::{format_ident, quote};
use weedle::types::ReturnType;

fn resolve_builtin_type(name: &str) -> Option<TokenStream> {
    match name {
        "string" => Some(quote! { String }),
        "bytes" => Some(quote! { Vec<u8> }),
        "u8" => Some(quote! { u8 }),
        "i8" => Some(quote! { i8 }),
        "u16" => Some(quote! { u16 }),
        "i16" => Some(quote! { i16 }),
        "u32" => Some(quote! { u32 }),
        "i32" => Some(quote! { i32 }),
        "u64" => Some(quote! { u64 }),
        "i64" => Some(quote! { i64 }),
        "f32" => Some(quote! { f32 }),
        "f64" => Some(quote! { f64 }),
        "void" => Some(quote! { () }),
        _ => None,
    }
}

fn resolve_builtin_type_ffi(name: &str) -> Option<TokenStream> {
    match name {
        "string" => Some(quote! { uniffi::RustBuffer }),
        "bytes" => Some(quote! { uniffi::RustBuffer }),
        "u8" => Some(quote! { u8 }),
        "i8" => Some(quote! { i8 }),
        "u16" => Some(quote! { u16 }),
        "i16" => Some(quote! { i16 }),
        "u32" => Some(quote! { u32 }),
        "i32" => Some(quote! { i32 }),
        "u64" => Some(quote! { u64 }),
        "i64" => Some(quote! { i64 }),
        "f32" => Some(quote! { f32 }),
        "f64" => Some(quote! { f64 }),
        "void" => Some(quote! { () }),
        _ => None,
    }
}

pub trait TypeResolver {
    fn resolve(&self) -> TokenStream;

    fn resolve_ffi(&self) -> TokenStream;
}

impl TypeResolver for &weedle::types::Type<'_> {
    fn resolve(&self) -> TokenStream {
        (*self).resolve()
    }

    fn resolve_ffi(&self) -> TokenStream {
        (*self).resolve_ffi()
    }
}

impl TypeResolver for weedle::types::Type<'_> {
    fn resolve(&self) -> TokenStream {
        match self {
            weedle::types::Type::Single(t) => match t {
                weedle::types::SingleType::Any(_) => panic!("no support for `any` types"),
                weedle::types::SingleType::NonAny(t) => t.resolve(),
            },
            weedle::types::Type::Union(_) => panic!("no support for union types yet"),
        }
    }

    fn resolve_ffi(&self) -> TokenStream {
        match self {
            weedle::types::Type::Single(t) => match t {
                weedle::types::SingleType::Any(_) => panic!("no support for `any` types"),
                weedle::types::SingleType::NonAny(t) => t.resolve_ffi(),
            },
            weedle::types::Type::Union(_) => panic!("no support for union types yet"),
        }
    }
}

impl TypeResolver for weedle::types::NonAnyType<'_> {
    fn resolve(&self) -> TokenStream {
        match self {
            weedle::types::NonAnyType::Boolean(t) => t.resolve(),
            weedle::types::NonAnyType::Identifier(t) => t.resolve(),
            weedle::types::NonAnyType::Integer(t) => t.resolve(),
            weedle::types::NonAnyType::FloatingPoint(t) => t.resolve(),
            weedle::types::NonAnyType::Sequence(t) => t.resolve(),
            weedle::types::NonAnyType::RecordType(t) => t.resolve(),
            _ => panic!("no support for type {:?}", self),
        }
    }

    fn resolve_ffi(&self) -> TokenStream {
        match self {
            weedle::types::NonAnyType::Boolean(t) => t.resolve_ffi(),
            weedle::types::NonAnyType::Identifier(t) => t.resolve_ffi(),
            weedle::types::NonAnyType::Integer(t) => t.resolve_ffi(),
            weedle::types::NonAnyType::FloatingPoint(t) => t.resolve_ffi(),
            weedle::types::NonAnyType::Sequence(t) => t.resolve_ffi(),
            weedle::types::NonAnyType::RecordType(t) => t.resolve_ffi(),
            _ => panic!("no support for type {:?}", self),
        }
    }
}

impl<T: TypeResolver> TypeResolver for weedle::types::MayBeNull<T> {
    fn resolve(&self) -> TokenStream {
        let type_ = self.type_.resolve();
        match self.q_mark {
            None => type_,
            Some(_) => {
                quote! { Option<#type_> }
            }
        }
    }

    fn resolve_ffi(&self) -> TokenStream {
        let type_ = self.type_.resolve_ffi();
        match self.q_mark {
            None => type_,
            Some(_) => {
                quote! { uniffi::RustBuffer }
            }
        }
    }
}

impl TypeResolver for weedle::types::IntegerType {
    fn resolve(&self) -> TokenStream {
        panic!(
            "WebIDL integer types not implemented ({:?}); consider using u8, u16, u32 or u64",
            self
        )
    }
    fn resolve_ffi(&self) -> TokenStream {
        self.resolve()
    }
}

impl TypeResolver for weedle::types::FloatingPointType {
    fn resolve(&self) -> TokenStream {
        match self {
            weedle::types::FloatingPointType::Float(t) => t.resolve(),
            weedle::types::FloatingPointType::Double(t) => t.resolve(),
        }
    }

    fn resolve_ffi(&self) -> TokenStream {
        match self {
            weedle::types::FloatingPointType::Float(t) => t.resolve_ffi(),
            weedle::types::FloatingPointType::Double(t) => t.resolve_ffi(),
        }
    }
}

impl TypeResolver for weedle::types::SequenceType<'_> {
    fn resolve(&self) -> TokenStream {
        let t = self.generics.body.as_ref().resolve();
        quote! { Vec<#t> }
    }

    fn resolve_ffi(&self) -> TokenStream {
        quote! { uniffi::RustBuffer }
    }
}

impl TypeResolver for weedle::types::RecordKeyType<'_> {
    fn resolve(&self) -> TokenStream {
        use weedle::types::RecordKeyType::*;
        match self {
            Byte(_) | USV(_) => panic!(
                "WebIDL Byte or USV string type not implemented ({self:?}); \
                 consider using a string",
            ),
            DOM(_) => quote! { String },
            NonAny(t) => t.resolve(),
        }
    }

    fn resolve_ffi(&self) -> TokenStream {
        use weedle::types::RecordKeyType::*;
        match self {
            Byte(_) | USV(_) => panic!(
                "WebIDL Byte or USV string type not implemented ({self:?}); \
                 consider using a string",
            ),
            DOM(_) => quote! { uniffi::RustBuffer },
            NonAny(t) => t.resolve_ffi(),
        }
    }
}

impl TypeResolver for weedle::types::RecordType<'_> {
    fn resolve(&self) -> TokenStream {
        let key_type = self.generics.body.0.resolve();
        let value_type = self.generics.body.2.resolve();

        quote! { ::std::collections::HashMap<#key_type, #value_type> }
    }

    fn resolve_ffi(&self) -> TokenStream {
        quote! { uniffi::RustBuffer }
    }
}

impl TypeResolver for weedle::common::Identifier<'_> {
    fn resolve(&self) -> TokenStream {
        match resolve_builtin_type(self.0) {
            Some(type_) => type_,
            None => {
                let ident = format_ident!("{}", self.0);
                quote! { #ident }
            }
        }
    }

    fn resolve_ffi(&self) -> TokenStream {
        match resolve_builtin_type_ffi(self.0) {
            Some(type_) => type_,
            None => {
                quote! { uniffi::RustBuffer }
            }
        }
    }
}

impl TypeResolver for weedle::term::Boolean {
    fn resolve(&self) -> TokenStream {
        quote! { i8 }
    }

    fn resolve_ffi(&self) -> TokenStream {
        self.resolve()
    }
}

impl TypeResolver for weedle::types::FloatType {
    fn resolve(&self) -> TokenStream {
        if self.unrestricted.is_some() {
            panic!("we don't support `unrestricted float`");
        }
        quote! { f32 }
    }

    fn resolve_ffi(&self) -> TokenStream {
        self.resolve()
    }
}

impl TypeResolver for weedle::types::DoubleType {
    fn resolve(&self) -> TokenStream {
        if self.unrestricted.is_some() {
            panic!("we don't support `unrestricted double`");
        }
        quote! { f64 }
    }

    fn resolve_ffi(&self) -> TokenStream {
        quote! { bool }
    }
}

pub fn return_type(typ: &ReturnType) -> TokenStream {
    let ReturnType::Type(typ) = typ else {
        panic!("can't handle undefined return type")
    };
    let ret = typ.resolve();
    quote! { -> #ret }
}

pub fn return_type_ffi(typ: &ReturnType) -> TokenStream {
    let ReturnType::Type(typ) = typ else {
        panic!("can't handle undefined return type")
    };
    let ret = typ.resolve_ffi();
    quote! { -> #ret }
}
