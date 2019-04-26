#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include "glean.h"

int main(void)
{
  glean_initialize();

  printf("Glean upload enabled? %d\n", glean_is_upload_enabled());
  glean_set_upload_enabled(0);
  printf("Glean upload enabled? %d\n", glean_is_upload_enabled());
  glean_set_upload_enabled(1);

  struct ExternError err;
  uint64_t metric = glean_new_counter_metric("counter", "local", &err);
  printf("Created counter: %llu\n", metric);

  glean_counter_add(metric, 2, &err);

  char *payload = glean_ping_collect("core", &err);
  printf("Payload:\n%s\n", payload);
  glean_str_free(payload);

  return 0;
}
