# DND2ndCharsheet
D&amp;D 2nd Ed Charsheet using mogwai 

## Prereqs
* [Rust](https://rustup.rs/)
* [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)
* `cargo install basic-http-server`

## To Run
```bash
wasm-pack build --target no-modules &&  basic-http-server -a 127.0.0.1:8888
```
