SRC = glean_app.c
HEADERS = ../glean.h

UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
    OBJECTS_RELEASE = ../../../target/release/libglean_ffi.so
    OBJECTS_DEBUG = ../../../target/debug/libglean_ffi.so
endif
ifeq ($(UNAME_S),Darwin)
    OBJECTS_RELEASE = ../../../target/release/libglean_ffi.dylib
    OBJECTS_DEBUG = ../../../target/debug/libglean_ffi.dylib
endif

glean_app: $(SRC) $(OBJECTS_DEBUG) $(HEADERS)
	$(CC) $(CFLAGS) -I.. -o $@ $(SRC) $(OBJECTS_DEBUG)

glean_app_release: $(SRC) $(OBJECTS_RELEASE) $(HEADERS)
	$(CC) $(CFLAGS) -I.. -o $@ $(SRC) $(OBJECTS_RELEASE)

$(OBJECTS_DEBUG): ../src/lib.rs
	cargo build

$(OBJECTS_RELEASE): ../src/lib.rs
	cargo build --release
