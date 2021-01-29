SRC = app.c
HEADERS = ../../glean-core/ffi/glean.h
CFLAGS = -I../../glean-core/ffi
LIBNAME = libmegazord

UNAME_S := $(shell uname -s)
ifeq ($(UNAME_S),Linux)
    OBJECTS_RELEASE = target/release/$(LIBNAME).so
    OBJECTS_DEBUG = target/debug/$(LIBNAME).so
endif
ifeq ($(UNAME_S),Darwin)
    OBJECTS_RELEASE = target/release/$(LIBNAME).dylib
    OBJECTS_DEBUG = target/debug/$(LIBNAME).dylib
endif

app: $(SRC) debug $(HEADERS)
	$(CC) $(CFLAGS) -o $@ $(SRC) $(OBJECTS_DEBUG)

app_release: $(SRC) release $(HEADERS)
	$(CC) $(CFLAGS) -o $@ $(SRC) $(OBJECTS_RELEASE)

debug:
	cargo build
.PHONY: debug

release:
	cargo build --release
.PHONY: release

run: debug app
	RUST_LOG=debug ./app

run-release: release app_release
	RUST_LOG=debug ./app_release
