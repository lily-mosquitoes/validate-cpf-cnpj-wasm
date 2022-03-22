# WebAssembly🕸️ + Rust🦀 = ❤️

This is an example of a function written in Rust and compiled to WebAssembly with [`wasm-pack`](https://crates.io/crates/wasm-pack).

The function is exposed to JavaScript with [`wasm-bindgen`](https://crates.io/crates/wasm-bindgen) and a minimal working example of usage is coded in `webassembly.html`. The function is used to validate the Brazilian national registry number, both for people (CPF) and legal entities (CNPJ).

This was made only for educational purposes, the source code is not particularly well optimized and may not be suitable for production.

## to run:

- build the package by running:

`wasm-pack build --target web`

- run any simple http server, for example:

`python3 -m http.server 8080`

- you should be able to test the WebAssembly/Rust function by going to:

[`http://127.0.0.1:8080/webassembly.html`](http://127.0.0.1:8080/webassembly.html)

- for performance comparison the same function is also implemented in pure JavaScript, you can test by going to:

[`http://127.0.0.1:8080/javascript.html`](http://127.0.0.1:8080/javascript.html)`
