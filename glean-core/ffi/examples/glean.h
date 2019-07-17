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

typedef const int32_t *RawIntArray;

typedef const char *const *RawStringArray;

void glean_boolean_set(uint64_t glean_handle, uint64_t metric_id, uint8_t value);

uint8_t glean_boolean_should_record(uint64_t glean_handle, uint64_t metric_id);

uint8_t glean_boolean_test_get_value(uint64_t glean_handle,
                                     uint64_t metric_id,
                                     FfiStr storage_name);

uint8_t glean_boolean_test_has_value(uint64_t glean_handle,
                                     uint64_t metric_id,
                                     FfiStr storage_name);

void glean_counter_add(uint64_t glean_handle, uint64_t metric_id, int32_t amount);

uint8_t glean_counter_should_record(uint64_t glean_handle, uint64_t metric_id);

int32_t glean_counter_test_get_value(uint64_t glean_handle,
                                     uint64_t metric_id,
                                     FfiStr storage_name);

uint8_t glean_counter_test_has_value(uint64_t glean_handle,
                                     uint64_t metric_id,
                                     FfiStr storage_name);

void glean_datetime_set(uint64_t glean_handle,
                        uint64_t metric_id,
                        int32_t year,
                        uint32_t month,
                        uint32_t day,
                        uint32_t hour,
                        uint32_t minute,
                        uint32_t second,
                        int64_t nano,
                        int32_t offset_seconds);

uint8_t glean_datetime_should_record(uint64_t glean_handle, uint64_t metric_id);

char *glean_datetime_test_get_value_as_string(uint64_t glean_handle,
                                              uint64_t metric_id,
                                              FfiStr storage_name);

uint8_t glean_datetime_test_has_value(uint64_t glean_handle,
                                      uint64_t metric_id,
                                      FfiStr storage_name);

/**
 * Initialize the logging system based on the target platform. This ensures
 * that logging is shown when executing Glean unit tests.
 */
void glean_enable_logging(void);

void glean_event_record(uint64_t glean_handle,
                        uint64_t metric_id,
                        uint64_t timestamp,
                        RawIntArray extra_keys,
                        RawStringArray extra_values,
                        int32_t extra_len);

uint8_t glean_event_should_record(uint64_t glean_handle, uint64_t metric_id);

char *glean_event_test_get_value_as_json_string(uint64_t glean_handle,
                                                uint64_t metric_id,
                                                FfiStr storage_name);

uint8_t glean_event_test_has_value(uint64_t glean_handle, uint64_t metric_id, FfiStr storage_name);

uint64_t glean_initialize(FfiStr data_dir, FfiStr application_id, uint8_t upload_enabled);

uint8_t glean_is_upload_enabled(uint64_t glean_handle);

uint64_t glean_new_boolean_metric(FfiStr category,
                                  FfiStr name,
                                  RawStringArray send_in_pings,
                                  int32_t send_in_pings_len,
                                  int32_t lifetime,
                                  uint8_t disabled);

uint64_t glean_new_counter_metric(FfiStr category,
                                  FfiStr name,
                                  RawStringArray send_in_pings,
                                  int32_t send_in_pings_len,
                                  int32_t lifetime,
                                  uint8_t disabled);

uint64_t glean_new_datetime_metric(FfiStr category,
                                   FfiStr name,
                                   RawStringArray send_in_pings,
                                   int32_t send_in_pings_len,
                                   int32_t lifetime,
                                   uint8_t disabled,
                                   int32_t time_unit);

uint64_t glean_new_event_metric(FfiStr category,
                                FfiStr name,
                                RawStringArray send_in_pings,
                                int32_t send_in_pings_len,
                                int32_t lifetime,
                                uint8_t disabled,
                                RawStringArray extra_keys,
                                int32_t extra_keys_len);

uint64_t glean_new_ping_type(FfiStr ping_name, uint8_t include_client_id);

uint64_t glean_new_string_list_metric(FfiStr category,
                                      FfiStr name,
                                      RawStringArray send_in_pings,
                                      int32_t send_in_pings_len,
                                      int32_t lifetime,
                                      uint8_t disabled);

uint64_t glean_new_string_metric(FfiStr category,
                                 FfiStr name,
                                 RawStringArray send_in_pings,
                                 int32_t send_in_pings_len,
                                 int32_t lifetime,
                                 uint8_t disabled);

uint64_t glean_new_uuid_metric(FfiStr category,
                               FfiStr name,
                               RawStringArray send_in_pings,
                               int32_t send_in_pings_len,
                               int32_t lifetime,
                               uint8_t disabled);

void glean_on_ready_to_send_pings(uint64_t glean_handle);

char *glean_ping_collect(uint64_t glean_handle, uint64_t ping_type_handle);

void glean_register_ping_type(uint64_t glean_handle, uint64_t ping_type_handle);

uint8_t glean_send_ping(uint64_t glean_handle, uint64_t ping_type_handle, uint8_t log_ping);

uint8_t glean_send_ping_by_name(uint64_t glean_handle, FfiStr ping_name, uint8_t log_ping);

void glean_set_upload_enabled(uint64_t glean_handle, uint8_t flag);

void glean_string_list_add(uint64_t glean_handle, uint64_t metric_id, FfiStr value);

void glean_string_list_set(uint64_t glean_handle,
                           uint64_t metric_id,
                           RawStringArray values,
                           int32_t values_len);

uint8_t glean_string_list_should_record(uint64_t glean_handle, uint64_t metric_id);

char *glean_string_list_test_get_value_as_json_string(uint64_t glean_handle,
                                                      uint64_t metric_id,
                                                      FfiStr storage_name);

uint8_t glean_string_list_test_has_value(uint64_t glean_handle,
                                         uint64_t metric_id,
                                         FfiStr storage_name);

void glean_string_set(uint64_t glean_handle, uint64_t metric_id, FfiStr value);

uint8_t glean_string_should_record(uint64_t glean_handle, uint64_t metric_id);

char *glean_string_test_get_value(uint64_t glean_handle, uint64_t metric_id, FfiStr storage_name);

uint8_t glean_string_test_has_value(uint64_t glean_handle, uint64_t metric_id, FfiStr storage_name);

uint8_t glean_test_has_ping_type(uint64_t glean_handle, FfiStr ping_name);

void glean_uuid_set(uint64_t glean_handle, uint64_t metric_id, FfiStr value);

uint8_t glean_uuid_should_record(uint64_t glean_handle, uint64_t metric_id);

char *glean_uuid_test_get_value(uint64_t glean_handle, uint64_t metric_id, FfiStr storage_name);

uint8_t glean_uuid_test_has_value(uint64_t glean_handle, uint64_t metric_id, FfiStr storage_name);

void glean_destroy_glean(uint64_t handle, ExternError *error);
void glean_destroy_boolean_metric(uint64_t handle, ExternError *error);
void glean_destroy_string_metric(uint64_t handle, ExternError *error);
void glean_destroy_counter_metric(uint64_t handle, ExternError *error);
void glean_str_free(char *ptr);

