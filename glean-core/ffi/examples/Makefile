objects = glean_app.c $(wildcard ../../../target/release/libglean_ffi.so) $(wildcard ../../../target/release/libglean_ffi.dylib)
headers = glean.h

glean_app: $(objects) $(headers)
	$(CC) $(CFLAGS) -o $@ $(objects)


../../../target/debug/libglean_ffi.dylib: ../src/lib.rs
	cargo build

../../../target/release/libglean_ffi.dylib: ../src/lib.rs
	cargo build --release
