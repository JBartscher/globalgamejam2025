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


### Assets:
* Audio - DeepDive https://opengameart.org/content/deep-dive-0 
* Fonts - kenny.nl - kenny-fonts https://kenney.nl/assets/kenney-fonts
* 3D Modelle - kenny.nl - watercraft-kit https://kenney.nl/assets/watercraft-kit
* 3D Modelle - kenny.nl - priate-kit https://kenney.nl/assets/pirate-kit