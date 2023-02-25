# rust-opaque-example
Example of using opaque C structs in rust FFI project.

[More details](https://avivg.github.io/blog/2023/02/25/rust__ffi_opaque.html)

## Build the shared library
```bash
cd shared_lib
mkdir build
cd build
cmake ..
make
```

## Run the example
```bash
cd opaque-rs
LD_LIBRARY_PATH=$LD_LIBRARY_PATH:../shared_lib/build cargo test
```
