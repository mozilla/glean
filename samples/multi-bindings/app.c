// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at https://mozilla.org/MPL/2.0/.

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>

#include "glean.h"

// Exported from libmegazord.
void increment_native_metric(int amount);

int main(void)
{
  int exitcode = 0;

  glean_enable_logging();
  FfiConfiguration cfg = {
    "./tmp",
    "multi-bindings-sample",
    "C",
    true,
    NULL
  };

  increment_native_metric(7);

  glean_initialize(&cfg);
  glean_flush_rlb_dispatcher();

  uint64_t store1 = glean_new_ping_type("store1", true, false, NULL, 0);
  glean_register_ping_type(store1);

  const char *pings[2];
  pings[0] = "store1";
  pings[1] =  NULL;
  uint64_t metric = glean_new_counter_metric("test", "runs", pings, 1, Lifetime_Ping, 0);

  assert(1 == glean_counter_test_has_value(metric, "store1"));
  assert(7 == glean_counter_test_get_value(metric, "store1"));

  glean_counter_add(metric, 1);

  assert(1 == glean_counter_test_has_value(metric, "store1"));
  assert(8 == glean_counter_test_get_value(metric, "store1"));

  glean_submit_ping_by_name("store1", NULL);

  FfiPingUploadTask task;
  glean_set_log_pings(1);

  for (;;) {
      glean_get_upload_task(&task);
      if (task.tag == FfiPingUploadTask_Done) {
          break;
      }

      if (task.tag == FfiPingUploadTask_Upload) {
          glean_process_ping_upload_response(&task, UPLOAD_RESULT_HTTP_STATUS | 200);
      }
  }

  assert(0 == glean_counter_test_has_value(metric, "store1"));

  glean_destroy_counter_metric(metric);
  glean_destroy_glean();

  return exitcode;
}
