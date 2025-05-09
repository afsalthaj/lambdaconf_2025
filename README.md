### WASM TIME

```sh
cd add
cargo component build --release

cd ../calculator
cargo component build --release


cd ../myrunner
cargo component build --release
```

### Compose components

```sh
## from root
wac plug calculator/target/wasm32-wasip1/release/caclulator.wasm  --plug  add/target/wasm32-wasip1/release/add.wasm  -o calculator_add.wasm
wac plug myrunner/target/wasm32-wasip1/release/myrunner.wasm  --plug calculator_add.wasm  -o myrunner_composed.wasm
```

### Use forked wasmtime to invoke

```sh
/Users/afsalthaj/projects/ribbb/wasmtime/target/debug/wasmtime --invoke 'eval-expression("abc")' myrunner_composed.wasm
```
