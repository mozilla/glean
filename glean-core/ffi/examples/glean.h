#include <stdint.h>

struct ExternError {
  int32_t code;
  char *message; // note: nullable
};

void glean_initialize();

uint8_t glean_is_upload_enabled();

uint64_t glean_new_boolean_metric(const char* name, const char* category, struct ExternError *err);

uint64_t glean_new_counter_metric(const char* name, const char* category, struct ExternError *err);
void glean_counter_add(uint64_t handle, uint32_t amount, struct ExternError *err);

uint64_t glean_new_string_metric(const char* name, const char* category, struct ExternError *err);

void glean_set_upload_enabled(uint8_t flag);

char *glean_ping_collect(const char* ping_name, struct ExternError *error);

void glean_destroy_boolean_metric(uint64_t handle, struct ExternError *error);

void glean_str_free(char* ptr);
