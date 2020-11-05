# image-wasm

Proof of concept image manipulation using the Rust `image` crate.

## Running

```shell
cargo install wasm-pack
wasm-pack build --target web
python server.py
```

Then visit localhost:3000 in a browser.