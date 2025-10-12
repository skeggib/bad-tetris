# Notes

## Setting-up the project

[Compiling from Rust to WebAssembly - MDN](https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm)

install wasm-pack: `cargo install wasm-pack`

create new project: `cargo new --lib <name>`

hello world in Rust-WASM:

```rust
use wasm_bindgen::prelude::*;

// binding a JavaScript function to Rust
#[wasm_bindgen]
extern "C" {
      pub fn alert(s: &str);
}

// binding a Rust function to JavaScript
#[wasm_bindgen]
pub fn greet(name: &str) {
      alert(&format!("Hello, {}!", name));
}
```

update Cargo.toml to add:
- 2021 edition
- cdylib crate-type
- wasm-bindgen 0.2 dependency

building the package: `wasm-pack build --target web`

call Rust from html:

```html
<script type="module">
  import init, { greet } from "./pkg/<name>.js";

  init().then(() => {
    greet("WebAssembly");
  });
</script>
```

[Making the package available to npm](https://developer.mozilla.org/en-US/docs/WebAssembly/Guides/Rust_to_Wasm#making_our_package_available_to_npm)

## Rust WebGL

add web-sys dependency: bindings for web APIs (including WebGL)

add canvas to html

the [start attribute](https://rustwasm.github.io/docs/wasm-bindgen/reference/attributes/on-rust-exports/start.html) configures the entrypoint of the Wasm executable:

```rust
#[wasm_bindgen(start)]
fn start() {}
```
## Starting drawing

provide a buffer to the vertex shader and fragment shader that will consume it using attributes
the vertex shader computes the position of the vertex (`gl_Position`)
the fragment shader computes the color of the vertex (`gl_FragColor`)
