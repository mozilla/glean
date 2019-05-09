/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen.
 * To generate this file:
 *   1. Get the latest cbindgen using `cargo install --force cbindgen`
 *      a. Alternatively, you can clone `https://github.com/eqrion/cbindgen` and use a tagged release
 *   2. Run `cbindgen glean-core/ffi --lockfile Cargo.lock -o glean-core/ffi/examples/glean.h`
 */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

/**
 * A wrapper around error codes, which is represented identically to an i32 on the other side of
 * the FFI. Essentially exists to check that we don't accidentally reuse success/panic codes for
 * other things.
 */
typedef int32_t ErrorCode;

/**
 * Represents an error that occured within rust, storing both an error code, and additional data
 * that may be used by the caller.
 * Misuse of this type can cause numerous issues, so please read the entire documentation before
 * usage.
 * ## Rationale
 * This library encourages a pattern of taking a `&mut ExternError` as the final parameter for
 * functions exposed over the FFI. This is an "out parameter" which we use to write error/success
 * information that occurred during the function's execution.
 * To be clear, this means instances of `ExternError` will be created on the other side of the FFI,
 * and passed (by mutable reference) into Rust.
 * While this pattern is not particularly ergonomic in Rust (although hopefully this library
 * helps!), it offers two main benefits over something more ergonomic (which might be `Result`
 * shaped).
 * 1. It avoids defining a large number of `Result`-shaped types in the FFI consumer, as would
 * be required with something like an `struct ExternResult<T> { ok: *mut T, err:... }`
 * 2. It offers additional type safety over `struct ExternResult { ok: *mut c_void, err:... }`,
 * which helps avoid memory safety errors. It also can offer better performance for returning
 * primitives and repr(C) structs (no boxing required).
 * It also is less tricky to use properly than giving consumers a `get_last_error()` function, or
 * similar.
 * ## Caveats
 * Note that the order of the fields is `code` (an i32) then `message` (a `*mut c_char`), getting
 * this wrong on the other side of the FFI will cause memory corruption and crashes.
 * The fields are public largely for documentation purposes, but you should use
 * [`ExternError::new_error`] or [`ExternError::success`] to create these.
 * ## Layout/fields
 * This struct's field are not `pub` (mostly so that we can soundly implement `Send`, but also so
 * that we can verify rust users are constructing them appropriately), the fields, their types, and
 * their order are *very much* a part of the public API of this type. Consumers on the other side
 * of the FFI will need to know its layout.
 * If this were a C struct, it would look like
 * ```c,no_run
 * struct ExternError {
 * int32_t code;
 * char *message; // note: nullable
 * };
 * ```
 * In rust, there are two fields, in this order: `code: ErrorCode`, and `message: *mut c_char`.
 * Note that ErrorCode is a `#[repr(transparent)]` wrapper around an `i32`, so the first property
 * is equivalent to an `i32`.
 * #### The `code` field.
 * This is the error code, 0 represents success, all other values represent failure. If the `code`
 * field is nonzero, there should always be a message, and if it's zero, the message will always be
 * null.
 * #### The `message` field.
 * This is a null-terminated C string containing some amount of additional information about the
 * error. If the `code` property is nonzero, there should always be an error message. Otherwise,
 * this should will be null.
 * This string (when not null) is allocated on the rust heap (using this crate's
 * [`rust_string_to_c`]), and must be freed on it as well. Critically, if there are multiple rust
 * packages using being used in the same application, it *must be freed on the same heap that
 * allocated it*, or you will corrupt both heaps.
 * Typically, this object is managed on the other side of the FFI (on the "FFI consumer"), which
 * means you must expose a function to release the resources of `message` which can be done easily
 * using the [`define_string_destructor!`] macro provided by this crate.
 * If, for some reason, you need to release the resources directly, you may call
 * `ExternError::release()`. Note that you probably do not need to do this, and it's
 * intentional that this is not called automatically by implementing `drop`.
 * ## Example
 * ```rust,no_run
 * use ffi_support::{ExternError, ErrorCode};
 * #[derive(Debug)]
 * pub enum MyError {
 * IllegalFoo(String),
 * InvalidBar(i64),
 *  ...
 * }
 *  Putting these in a module is obviously optional, but it allows documentation, and helps
 *  avoid accidental reuse.
 * pub mod error_codes {
 *  note: -1 and 0 are reserved by ffi_support
 * pub const ILLEGAL_FOO: i32 = 1;
 * pub const INVALID_BAR: i32 = 2;
 *  ...
 * }
 * fn get_code(e: &MyError) -> ErrorCode {
 * match e {
 * MyError::IllegalFoo(_) => ErrorCode::new(error_codes::ILLEGAL_FOO),
 * MyError::InvalidBar(_) => ErrorCode::new(error_codes::INVALID_BAR),
 *  ...
 * }
 * }
 * impl From<MyError> for ExternError {
 * fn from(e: MyError) -> ExternError {
 * ExternError::new_error(get_code(&e), format!("{:?}", e))
 * }
 * }
 * ```
 */
typedef struct {
  ErrorCode code;
  char *message;
} ExternError;

/**
 * `FfiStr<'a>` is a safe (`#[repr(transparent)]`) wrapper around a
 * nul-terminated `*const c_char` (e.g. a C string). Conceptually, it is
 * similar to [`std::ffi::CStr`], except that it may be used in the signatures
 * of extern "C" functions.
 * Functions accepting strings should use this instead of accepting a C string
 * directly. This allows us to write those functions using safe code without
 * allowing safe Rust to cause memory unsafety.
 * A single function for constructing these from Rust ([`FfiStr::from_raw`])
 * has been provided. Most of the time, this should not be necessary, and users
 * should accept `FfiStr` in the parameter list directly.
 * ## Caveats
 * An effort has been made to make this struct hard to misuse, however it is
 * still possible, if the `'static` lifetime is manually specified in the
 * struct. E.g.
 * ```rust,no_run
 * # use ffi_support::FfiStr;
 *  NEVER DO THIS
 * #[no_mangle]
 * extern "C" fn never_do_this(s: FfiStr<'static>) {
 *  save `s` somewhere, and access it after this
 *  function returns.
 * }
 * ```
 * Instead, one of the following patterns should be used:
 * ```
 * # use ffi_support::FfiStr;
 * #[no_mangle]
 * extern "C" fn valid_use_1(s: FfiStr<'_>) {
 *  Use of `s` after this function returns is impossible
 * }
 *  Alternative:
 * #[no_mangle]
 * extern "C" fn valid_use_2(s: FfiStr) {
 *  Use of `s` after this function returns is impossible
 * }
 * ```
 */
typedef const char *FfiStr;

typedef const char *const *RawStringArray;

void glean_boolean_set(uint64_t glean_handle,
                       uint64_t metric_id,
                       uint8_t value,
                       ExternError *error);

void glean_counter_add(uint64_t glean_handle,
                       uint64_t metric_id,
                       uint64_t amount,
                       ExternError *error);

uint64_t glean_counter_test_get_value(uint64_t glean_handle,
                                      uint64_t metric_id,
                                      FfiStr storage_name);

uint8_t glean_counter_test_has_value(uint64_t glean_handle,
                                     uint64_t metric_id,
                                     FfiStr storage_name);

uint64_t glean_initialize(FfiStr data_dir, FfiStr application_id);

uint8_t glean_is_initialized(uint64_t glean_handle);

uint8_t glean_is_upload_enabled(uint64_t glean_handle);

uint64_t glean_new_boolean_metric(FfiStr category,
                                  FfiStr name,
                                  RawStringArray send_in_pings,
                                  int32_t send_in_pings_len,
                                  int32_t lifetime,
                                  ExternError *err);

uint64_t glean_new_counter_metric(FfiStr category,
                                  FfiStr name,
                                  RawStringArray send_in_pings,
                                  int32_t send_in_pings_len,
                                  int32_t lifetime,
                                  ExternError *err);

uint64_t glean_new_string_metric(FfiStr category,
                                 FfiStr name,
                                 RawStringArray send_in_pings,
                                 int32_t send_in_pings_len,
                                 int32_t lifetime,
                                 ExternError *err);

char *glean_ping_collect(uint64_t glean_handle, FfiStr ping_name, ExternError *error);

void glean_send_ping(uint64_t glean_handle, FfiStr ping_name);

void glean_set_upload_enabled(uint64_t glean_handle, uint8_t flag);

void glean_string_set(uint64_t glean_handle, uint64_t metric_id, FfiStr value, ExternError *error);

void glean_destroy_glean(uint64_t handle, ExternError *error);
void glean_destroy_boolean_metric(uint64_t handle, ExternError *error);
void glean_destroy_string_metric(uint64_t handle, ExternError *error);
void glean_destroy_counter_metric(uint64_t handle, ExternError *error);
void glean_str_free(char *ptr);

