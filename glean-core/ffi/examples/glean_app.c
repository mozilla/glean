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

  // task == 0 (wait)
  // task == 1 (upload)
  // task == 2 (done)
  FfiPingUploadTask task = glean_get_upload_task();
  while (task.tag == 1) {
    printf("tag: %d\n", task.tag);
    if (task.tag == 1) {
      printf("uuid: %s\n", task.upload.uuid);
      printf("path: %s\n", task.upload.path);
      printf("body: %s\n", task.upload.body);
    }
    glean_process_ping_upload_response(task.upload.uuid, 200);
    task = glean_get_upload_task();
  }
  printf("tag: %d\n", task.tag);

  glean_destroy_counter_metric(metric);

  glean_destroy_glean();

  return 0;
}
