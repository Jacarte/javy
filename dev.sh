rustup install stable && rustup default stable
rustup target add wasm32-wasi
cargo install wasmtime-cli
cargo install cargo-wasi
cargo install wizer --all-features