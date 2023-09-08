# Rust-Zig-FFI
A fun proof of concept for linking Rust code against libc to call into the Zig code. This is possible because Zig can export a C abi! 

# Building 

1. Build Zig: `zig build-lib src/main.zig -dynamic -femit-h`
2. Run Rust code: `cargo run`
