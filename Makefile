wasm-parallel:
	RUSTFLAGS='-C target-feature=+atomics,+bulk-memory,+mutable-globals' rustup run nightly wasm-pack build --target web --out-dir pkg --features "parallel" -Z build-std=panic_abort,std
