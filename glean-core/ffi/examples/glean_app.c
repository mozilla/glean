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
  uint64_t glean = glean_initialize(&cfg);
  uint64_t store1 = glean_new_ping_type("store1", true);
  glean_register_ping_type(glean, store1);

  printf("Glean upload enabled? %d\n", glean_is_upload_enabled(glean));
  glean_set_upload_enabled(glean, 0);
  printf("Glean upload enabled? %d\n", glean_is_upload_enabled(glean));
  glean_set_upload_enabled(glean, 1);

  const char *pings[2];
  pings[0] = "store1";
  pings[1] =  NULL;
  uint64_t metric = glean_new_counter_metric("local", "counter", pings, 1, 0, 0);
  printf("Created counter: %llu\n", metric);

  glean_counter_add(glean, metric, 2);

  char *payload = glean_ping_collect(glean, store1);
  printf("Payload:\n%s\n", payload);
  glean_str_free(payload);

  ExternError err;
  glean_destroy_counter_metric(metric, &err);

  glean_destroy_glean(glean, &err);

  return 0;
}
