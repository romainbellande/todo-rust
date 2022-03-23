# Todo Rust (with WASM)

## Requirements

* tailwindcss installed globally
* rust
* Taskfile: https://taskfile.dev/#/installation
* Mkcert (to have ssl with traefik): https://github.com/FiloSottile/mkcert
* trunk: `cargo install trunk`
* wasm-bindgen-cli: `cargo install wasm-bindgen-cli`


## Client

### Update ./client/tailwind.css file

`task --watch client:gen:css`
