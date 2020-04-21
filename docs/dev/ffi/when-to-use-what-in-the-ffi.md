# When to use what method of passing data between Rust and Java/Swift

There are a bunch of options here. For the purposes of our discussion,
there are two kinds of values you may want to pass over the FFI.

1. Types with identity (includes stateful types, resource types, or anything that
   isn't really serializable).
2. Plain ol' data.

## Types with identity

Examples of this are all metric type implementations, e.g. `StringMetric`.
These types are complex, implemented in Rust and make use of the global Glean singleton on the Rust side,
They have an equivalent on the wrapper side (Kotlin/Swift/Python), that forwards calls through FFI.

They all follow the same pattern:

A `ConcurrentHandleMap` stores all instances of the same type,
A handle is passed back and forth as a `u64` from Rust, `Long` from Kotlin, `UInt64` from Swift
or other equivalent type in other languages.

This is recommended for most cases, as it's the hardest to mess up.
Additionally, for types T such that `&T: Sync + Send`, or that you
need to call `&mut self` method, this is the safest choice.

Additionally, this will ensure panic-safety, as it will poison the internal Mutex, making further access impossible.

The [`ffi_support::handle_map` docs](https://docs.rs/ffi-support/*/ffi_support/handle_map/index.html) are good,
and under `ConcurrentHandleMap` include an example of how to set this up.

## Plain Old Data

This includes both primitive values, strings, arrays, or arbitrarily nested
structures containing them.

### Primitives

Numeric primitives are the easiest to pass between languages.
The main recommendation is: use the equivalent and same-sized type as the one provided by Rust.

There are a couple of exceptions/caveats, especially for Kotlin. All of them are caused by JNA/Android issues.
Swift has very good support for calling over the FFI.

1. `bool`: Don't use it. JNA doesn't handle it well. Instead, use a numeric type
    (like `u8`) and represent 0 for `false` and 1 for `true` for interchange over the
    FFI, converting back to Kotlin's `Boolean` or Swift's `Bool` after (as to
    not expose this somewhat annoying limitation in our public API).
    All wrappers already include utility functions to turn 8-bit integers (`u8`) back to booleans
    (`toBool()` or equivalent methods).

2. `usize`/`isize`: These cause the structure size to be different based on the
   platform. JNA does handle this if you use `NativeSize`, but it's awkward.
   Use the size-defined integers instead, such as `i64`/`i32` and their language-equivalents
   (Kotlin: `Long`/`Int`, Swift:`UInt64`/`UInt32`).
   *Caution:* In Kotlin integers are signed by default. You can use `u64`/`u32` for `Long`/`Int` if you ensure the values are non-negative through asserts or error reporting code.

3. `char`: Avoid these if possible. If you really have a use case consider `u32` instead.

    If you do this, you should probably be aware of the fact that Java chars are 16
    bit, and Swift `Character`s are actually strings (they represent Extended
    Grapheme Clusters, not codepoints).

### Strings

These we pass as null-terminated UTF-8 C-strings.

For return values, used `*mut c_char`, and for input, use
[`ffi_support::FfiStr`](https://docs.rs/ffi-support/*/ffi_support/struct.FfiStr.html)

1. If the string is returned from Rust to Kotlin/Swift, you need to expose a
   string destructor from your ffi crate. See
   [`ffi_support::define_string_destructor!`](https://docs.rs/ffi-support/*/ffi_support/macro.define_string_destructor.html)).

    For converting to a `*mut c_char`, use either
   [`rust_string_to_c`](https://docs.rs/ffi-support/*/ffi_support/fn.rust_string_to_c.html)
    if you have a `String`, or
   [`opt_rust_string_to_c`](https://docs.rs/ffi-support/*/ffi_support/fn.opt_rust_string_to_c.html)
    for `Option<String>` (None becomes `std::ptr::null_mut()`).

    **Important**: In Kotlin, the type returned by a function that produces this
    must be `Pointer`, and not `String`, and the parameter that the destructor takes
    as input must also be `Pointer`.

    Using `String` will *almost* work. JNA will convert the return value to
    `String` automatically, leaking the value Rust provides. Then, when passing
    to the destructor, it will allocate a temporary buffer, pass it to Rust, which
    we'll free, corrupting both heaps 💥. Oops!

2. If the string is passed into Rust from Kotlin/Swift, the Rust code should
   declare the parameter as a [`FfiStr<'_>`](https://docs.rs/ffi-support/*/ffi_support/struct.FfiStr.html).
   and things should then work more or less automatically. The `FfiStr` has methods
   for extracting it's data as `&str`, `Option<&str>`, `String`, and `Option<String>`.

### Aggregates

This is any type that's more complex than a primitive or a string (arrays,
structures, and combinations there-in).
There are two options we recommend for these cases:

1. Passing data as JSON. This is very easy and useful for prototyping, but is
   much slower and requires a great deal of copying and redundant encode/decode
   steps (in general, the data will be copied at least 4 times to make this
   work, and almost certainly more in practice).
   It can be done relatively easily by `derive(Serialize, Deserialize)`,
   and converting to a JSON string using `serde_json::to_string`.

   This is a viable option for test-only functions, where the overhead is not important.

2. Use `repr(C)` structures and copy the data across the boundary,
   carefully replicating the same structure on the wrapper side.
   In Kotlin this will require `@Structure.FieldOrder` annotations.
   Swift can directly handle C types.

   **Caution:** This is error prone! Structures, enumerations and unions need to be kept the same across all layers
   (Rust, generated C header, Kotlin, Swift, ...).
   Be extra careful, avoid adding references to structures and ensure the structures are correctly freed inside Rust.
   Copy out the data and convert into language-appropriate types (e.g. convert `*mut c_char` into Swift/Kotlin strings) as soon as possible.
