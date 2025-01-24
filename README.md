# Haverie
Game developed for the GlobalGameJam 2025 at the SAE Institute Hannover.
### Run web build
```shell

```

### Build web release build
``` shell
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "mygame" \
    ./target/wasm32-unknown-unknown/release/mygame.wasm
```