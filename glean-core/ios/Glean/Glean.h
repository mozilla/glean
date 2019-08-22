//
//  Glean.h
//  Glean
//
//  Created by Jan-Erik Rediger on 21.03.19.
//  Copyright © 2019 Jan-Erik Rediger. All rights reserved.
//

#import <UIKit/UIKit.h>

//! Project version number for Glean.
FOUNDATION_EXPORT double GleanVersionNumber;

//! Project version string for Glean.
FOUNDATION_EXPORT const unsigned char GleanVersionString[];

// In this header, you should import all the public headers of your framework using statements like #import <Glean/PublicHeader.h>

typedef const char *FfiStr;
typedef const char *const *RawStringArray;

typedef struct ExternError {
    int32_t code;
    char *message; // note: nullable
} ExternError;

typedef struct {
    FfiStr data_dir;
    FfiStr package_name;
    uint8_t upload_enabled;
    const int32_t *max_events;
} FfiConfiguration;

uint64_t glean_initialize(const FfiConfiguration *cfg);
void glean_test_clear_all_stores(uint64_t glean_handle);
void glean_set_upload_enabled(uint64_t glean_handle, uint8_t flag);

uint64_t glean_new_counter_metric(FfiStr category,
                                  FfiStr name,
                                  RawStringArray send_in_ping,
                                  int32_t send_in_pings_len,
                                  int32_t lifetime,
                                  uint8_t disabled);

void glean_counter_add(uint64_t glean_handle, uint64_t metric_id, int32_t amount);
int32_t glean_counter_test_get_value(uint64_t glean_handle,
                                     uint64_t metric_id,
                                     FfiStr storage_name);
uint8_t glean_counter_test_has_value(uint64_t glean_handle,
                                     uint64_t metric_id,
                                     FfiStr storage_name);
void glean_destroy_glean(uint64_t v, ExternError *err);
