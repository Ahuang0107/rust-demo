# readme

```shell
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen --no-typescript --target web --out-dir dist --out-name wasm target/wasm32-unknown-unknown/release/rust_demo.wasm
```