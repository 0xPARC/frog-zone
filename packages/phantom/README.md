# Phantom

## Test with `wasm-pack`

To install `wasm-pack`, please follow the guidance in [here](https://rustwasm.github.io/wasm-pack/installer).

### Test on `node`

```
wasm-pack test --node --release --test node --features dev
```

### Test on `browser`

```
wasm-pack test --firefox --headless --release --test browser --features dev
```
