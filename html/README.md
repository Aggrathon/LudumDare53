# Instructions for deploying to WASM

1. Install target: `rustup target install wasm32-unknown-unknown`
2. (Optional) Install runner for local tests (see *.cargo/config.toml*): `cargo install wasm-server-runner`
3. Install bindgen: `cargo install wasm-bindgen-cli`
4. Build for WASM: `cargo build --release --target wasm32-unknown-unknown`
5. Generate bindings: `wasm-bindgen --out-dir ./out/ --target web .\target\wasm32-unknown-unknown\release\ludum_dare_53.wasm`
6. Copy the html file to the root: `html/index.html`