objects = glean_app.c ../../../target/debug/libglean_ffi.dylib
headers = glean.h

glean_app: $(objects) $(headers)
	$(CC) $(CFLAGS) -o $@ $(objects)


../../../target/debug/libglean_ffi.dylib: ../src/lib.rs
	cargo build
