/* This Source Code Form is subject to the terms of the Mozilla Public
 * License, v. 2.0. If a copy of the MPL was not distributed with this
 * file, You can obtain one at http://mozilla.org/MPL/2.0/. */

/* DO NOT MODIFY THIS MANUALLY! This file was generated using cbindgen.
 * To generate this file:
 *   1. Get the latest cbindgen using `cargo install --force cbindgen`
 *      a. Alternatively, you can clone `https://github.com/eqrion/cbindgen` and use a tagged release
 *   2. Run `make cbindgen`
 */

typedef const char *FfiStr;
typedef uint64_t TimerId;


#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef const int64_t *RawInt64Array;

typedef const int32_t *RawIntArray;

typedef const char *const *RawStringArray;

/**
 * Configuration over FFI.
 *
 * **CAUTION**: This must match _exactly_ the definition on the Kotlin side.
 * If this side is changed, the Kotlin side need to be changed, too.
 */
typedef struct {
  FfiStr data_dir;
  FfiStr package_name;
  uint8_t upload_enabled;
  const int32_t *max_events;
} FfiConfiguration;

void glean_boolean_set(uint64_t glean_handle, uint64_t metric_id, uint8_t value);

uint8_t glean_boolean_test_get_value(uint64_t glean_handle,
                                     uint64_t metric_id,
                                     FfiStr storage_name);

uint8_t glean_boolean_test_has_value(uint64_t glean_handle,
                                     uint64_t metric_id,
                                     FfiStr storage_name);

void glean_counter_add(uint64_t glean_handle, uint64_t metric_id, int32_t amount);

int32_t glean_counter_test_get_value(uint64_t glean_handle,
                                     uint64_t metric_id,
                                     FfiStr storage_name);

uint8_t glean_counter_test_has_value(uint64_t glean_handle,
                                     uint64_t metric_id,
                                     FfiStr storage_name);

void glean_custom_distribution_accumulate_samples(uint64_t glean_handle,
                                                  uint64_t metric_id,
                                                  RawInt64Array raw_samples,
                                                  int32_t num_samples);

char *glean_custom_distribution_test_get_value_as_json_string(uint64_t glean_handle,
                                                              uint64_t metric_id,
                                                              FfiStr storage_name);

uint8_t glean_custom_distribution_test_has_value(uint64_t glean_handle,
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

char *glean_datetime_test_get_value_as_string(uint64_t glean_handle,
                                              uint64_t metric_id,
                                              FfiStr storage_name);

uint8_t glean_datetime_test_has_value(uint64_t glean_handle,
                                      uint64_t metric_id,
                                      FfiStr storage_name);

void glean_destroy_boolean_metric(uint64_t v);

void glean_destroy_counter_metric(uint64_t v);

void glean_destroy_custom_distribution_metric(uint64_t v);

void glean_destroy_datetime_metric(uint64_t v);

void glean_destroy_event_metric(uint64_t v);

void glean_destroy_glean(uint64_t v);

void glean_destroy_labeled_boolean_metric(uint64_t v);

void glean_destroy_labeled_counter_metric(uint64_t v);

void glean_destroy_labeled_string_metric(uint64_t v);

void glean_destroy_memory_distribution_metric(uint64_t v);

void glean_destroy_ping_type(uint64_t v);

void glean_destroy_quantity_metric(uint64_t v);

void glean_destroy_string_list_metric(uint64_t v);

void glean_destroy_string_metric(uint64_t v);

void glean_destroy_timespan_metric(uint64_t v);

void glean_destroy_timing_distribution_metric(uint64_t v);

void glean_destroy_uuid_metric(uint64_t v);

/**
 * Initialize the logging system based on the target platform. This ensures
 * that logging is shown when executing the Glean SDK unit tests.
 */
void glean_enable_logging(void);

void glean_event_record(uint64_t glean_handle,
                        uint64_t metric_id,
                        uint64_t timestamp,
                        RawIntArray extra_keys,
                        RawStringArray extra_values,
                        int32_t extra_len);

char *glean_event_test_get_value_as_json_string(uint64_t glean_handle,
                                                uint64_t metric_id,
                                                FfiStr storage_name);

uint8_t glean_event_test_has_value(uint64_t glean_handle, uint64_t metric_id, FfiStr storage_name);

char *glean_experiment_test_get_data(uint64_t glean_handle, FfiStr experiment_id);

uint8_t glean_experiment_test_is_active(uint64_t glean_handle, FfiStr experiment_id);

uint64_t glean_initialize(const FfiConfiguration *cfg);

uint8_t glean_is_upload_enabled(uint64_t glean_handle);

/**
 * Create a new instance of the sub-metric of this labeled metric.
 */
uint64_t glean_labeled_boolean_metric_get(uint64_t handle, FfiStr label);

/**
 * Create a new instance of the sub-metric of this labeled metric.
 */
uint64_t glean_labeled_counter_metric_get(uint64_t handle, FfiStr label);

/**
 * Create a new instance of the sub-metric of this labeled metric.
 */
uint64_t glean_labeled_string_metric_get(uint64_t handle, FfiStr label);

void glean_memory_distribution_accumulate(uint64_t glean_handle,
                                          uint64_t metric_id,
                                          uint64_t sample);

void glean_memory_distribution_accumulate_samples(uint64_t glean_handle,
                                                  uint64_t metric_id,
                                                  RawInt64Array raw_samples,
                                                  int32_t num_samples);

char *glean_memory_distribution_test_get_value_as_json_string(uint64_t glean_handle,
                                                              uint64_t metric_id,
                                                              FfiStr storage_name);

uint8_t glean_memory_distribution_test_has_value(uint64_t glean_handle,
                                                 uint64_t metric_id,
                                                 FfiStr storage_name);

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

uint64_t glean_new_custom_distribution_metric(FfiStr category,
                                              FfiStr name,
                                              RawStringArray send_in_pings,
                                              int32_t send_in_pings_len,
                                              int32_t lifetime,
                                              uint8_t disabled,
                                              uint64_t range_min,
                                              uint64_t range_max,
                                              uint64_t bucket_count,
                                              int32_t histogram_type);

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

/**
 * Create a new labeled metric.
 */
uint64_t glean_new_labeled_boolean_metric(FfiStr category,
                                          FfiStr name,
                                          RawStringArray send_in_pings,
                                          int32_t send_in_pings_len,
                                          int32_t lifetime,
                                          uint8_t disabled,
                                          RawStringArray labels,
                                          int32_t label_count);

/**
 * Create a new labeled metric.
 */
uint64_t glean_new_labeled_counter_metric(FfiStr category,
                                          FfiStr name,
                                          RawStringArray send_in_pings,
                                          int32_t send_in_pings_len,
                                          int32_t lifetime,
                                          uint8_t disabled,
                                          RawStringArray labels,
                                          int32_t label_count);

/**
 * Create a new labeled metric.
 */
uint64_t glean_new_labeled_string_metric(FfiStr category,
                                         FfiStr name,
                                         RawStringArray send_in_pings,
                                         int32_t send_in_pings_len,
                                         int32_t lifetime,
                                         uint8_t disabled,
                                         RawStringArray labels,
                                         int32_t label_count);

uint64_t glean_new_memory_distribution_metric(FfiStr category,
                                              FfiStr name,
                                              RawStringArray send_in_pings,
                                              int32_t send_in_pings_len,
                                              int32_t lifetime,
                                              uint8_t disabled,
                                              int32_t memory_unit);

uint64_t glean_new_ping_type(FfiStr ping_name, uint8_t include_client_id);

uint64_t glean_new_quantity_metric(FfiStr category,
                                   FfiStr name,
                                   RawStringArray send_in_pings,
                                   int32_t send_in_pings_len,
                                   int32_t lifetime,
                                   uint8_t disabled);

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

uint64_t glean_new_timespan_metric(FfiStr category,
                                   FfiStr name,
                                   RawStringArray send_in_pings,
                                   int32_t send_in_pings_len,
                                   int32_t lifetime,
                                   uint8_t disabled,
                                   int32_t time_unit);

uint64_t glean_new_timing_distribution_metric(FfiStr category,
                                              FfiStr name,
                                              RawStringArray send_in_pings,
                                              int32_t send_in_pings_len,
                                              int32_t lifetime,
                                              uint8_t disabled,
                                              int32_t time_unit);

uint64_t glean_new_uuid_metric(FfiStr category,
                               FfiStr name,
                               RawStringArray send_in_pings,
                               int32_t send_in_pings_len,
                               int32_t lifetime,
                               uint8_t disabled);

uint8_t glean_on_ready_to_send_pings(uint64_t glean_handle);

char *glean_ping_collect(uint64_t glean_handle, uint64_t ping_type_handle);

void glean_quantity_set(uint64_t glean_handle, uint64_t metric_id, int64_t value);

int64_t glean_quantity_test_get_value(uint64_t glean_handle,
                                      uint64_t metric_id,
                                      FfiStr storage_name);

uint8_t glean_quantity_test_has_value(uint64_t glean_handle,
                                      uint64_t metric_id,
                                      FfiStr storage_name);

void glean_register_ping_type(uint64_t glean_handle, uint64_t ping_type_handle);

uint8_t glean_send_pings_by_name(uint64_t glean_handle,
                                 RawStringArray ping_names,
                                 int32_t ping_names_len);

void glean_set_experiment_active(uint64_t glean_handle,
                                 FfiStr experiment_id,
                                 FfiStr branch,
                                 RawStringArray extra_keys,
                                 RawStringArray extra_values,
                                 int32_t extra_len);

void glean_set_experiment_inactive(uint64_t glean_handle, FfiStr experiment_id);

void glean_set_upload_enabled(uint64_t glean_handle, uint8_t flag);

/**
 *Public destructor for strings managed by the other side of the FFI.
 */
void glean_str_free(char *s);

void glean_string_list_add(uint64_t glean_handle, uint64_t metric_id, FfiStr value);

void glean_string_list_set(uint64_t glean_handle,
                           uint64_t metric_id,
                           RawStringArray values,
                           int32_t values_len);

char *glean_string_list_test_get_value_as_json_string(uint64_t glean_handle,
                                                      uint64_t metric_id,
                                                      FfiStr storage_name);

uint8_t glean_string_list_test_has_value(uint64_t glean_handle,
                                         uint64_t metric_id,
                                         FfiStr storage_name);

void glean_string_set(uint64_t glean_handle, uint64_t metric_id, FfiStr value);

char *glean_string_test_get_value(uint64_t glean_handle, uint64_t metric_id, FfiStr storage_name);

uint8_t glean_string_test_has_value(uint64_t glean_handle, uint64_t metric_id, FfiStr storage_name);

void glean_test_clear_all_stores(uint64_t glean_handle);

uint8_t glean_test_has_ping_type(uint64_t glean_handle, FfiStr ping_name);

void glean_timespan_cancel(uint64_t metric_id);

void glean_timespan_set_raw_nanos(uint64_t glean_handle,
                                  uint64_t metric_id,
                                  uint64_t elapsed_nanos);

void glean_timespan_set_start(uint64_t glean_handle, uint64_t metric_id, uint64_t start_time);

void glean_timespan_set_stop(uint64_t glean_handle, uint64_t metric_id, uint64_t stop_time);

uint64_t glean_timespan_test_get_value(uint64_t glean_handle,
                                       uint64_t metric_id,
                                       FfiStr storage_name);

uint8_t glean_timespan_test_has_value(uint64_t glean_handle,
                                      uint64_t metric_id,
                                      FfiStr storage_name);

void glean_timing_distribution_accumulate_samples(uint64_t glean_handle,
                                                  uint64_t metric_id,
                                                  RawInt64Array raw_samples,
                                                  int32_t num_samples);

void glean_timing_distribution_cancel(uint64_t metric_id, TimerId timer_id);

TimerId glean_timing_distribution_set_start(uint64_t metric_id, uint64_t start_time);

void glean_timing_distribution_set_stop_and_accumulate(uint64_t glean_handle,
                                                       uint64_t metric_id,
                                                       TimerId timer_id,
                                                       uint64_t stop_time);

char *glean_timing_distribution_test_get_value_as_json_string(uint64_t glean_handle,
                                                              uint64_t metric_id,
                                                              FfiStr storage_name);

uint8_t glean_timing_distribution_test_has_value(uint64_t glean_handle,
                                                 uint64_t metric_id,
                                                 FfiStr storage_name);

void glean_uuid_set(uint64_t glean_handle, uint64_t metric_id, FfiStr value);

char *glean_uuid_test_get_value(uint64_t glean_handle, uint64_t metric_id, FfiStr storage_name);

uint8_t glean_uuid_test_has_value(uint64_t glean_handle, uint64_t metric_id, FfiStr storage_name);
