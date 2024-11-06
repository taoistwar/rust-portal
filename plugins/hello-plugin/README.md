# Hello Plugin

Extism Rust PDK Plugin

See more documentation at https://github.com/extism/rust-pdk and
[join us on Discord](https://extism.org/discord) for more help.

## 开发

参考：<https://extism.org/docs/quickstart/plugin-quickstart>

### 构建

```bash
cd plugins/hello-plugin
cargo build --target wasm32-unknown-unknown
```

### 测试

```bash
extism call ../../target/wasm32-unknown-unknown/debug/hello_plugin.wasm greet --input "Benjamin"
```
