#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "glean.h"

int main(void)
{
  glean_enable_logging();
  FfiConfiguration cfg = {
    "/tmp/glean_data",
    "c-app",
    "C",
    true,
    NULL
  };
  glean_initialize(&cfg);
  uint64_t store1 = glean_new_ping_type("store1", true, false, NULL, 0);
  glean_register_ping_type(store1);

  printf("Glean upload enabled? %d\n", glean_is_upload_enabled());
  glean_set_upload_enabled(0);
  printf("Glean upload enabled? %d\n", glean_is_upload_enabled());
  glean_set_upload_enabled(1);

  const char *pings[2];
  pings[0] = "store1";
  pings[1] =  NULL;
  uint64_t metric = glean_new_counter_metric("local", "counter", pings, 1, 0, 0);
  printf("Created counter: %llu\n", metric);

  glean_counter_add(metric, 2);

  glean_submit_ping_by_name("store1", NULL);

  // Since we disabled upload and submitted a ping above,
  // we expect to have at least two pending pings: a deletion-request and a store1.
  //
  // The upload task.tag will either be:
  // 0 - "wait", glean is still parsing the pending_pings directory;
  // 1 - "upload", there is a new ping to upload and the task will also include the request data;
  // 2 - "done", there are no more pings to upload.
  //
  // NOTE: If, there are other ping files inside tmp/glean_data directory
  // they will also be consumed here by `glean_process_ping_upload_response`.
  FfiPingUploadTask task;
  glean_set_log_pings(1);

  for (;;) {
      glean_get_upload_task(&task);
      if (task.tag == FfiPingUploadTask_Done) {
          break;
      }

      printf("tag: %d\n", task.tag);

      if (task.tag == FfiPingUploadTask_Upload) {
          printf("path: %s\n", task.upload.path);
          printf("body length: %d\n", task.upload.body.len);

          glean_process_ping_upload_response(&task, UPLOAD_RESULT_HTTP_STATUS | 200);
      }
  }

  glean_destroy_counter_metric(metric);

  glean_destroy_glean();

  return 0;
}
