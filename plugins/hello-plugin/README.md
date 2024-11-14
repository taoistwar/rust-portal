# Hello Plugin

## 开发

### 构建

```bash
cd plugins/hello-plugin
cargo build --target wasm32-unknown-unknown
```

### 测试

#### 不带 host_fn 时

```bash
extism call ../../target/wasm32-unknown-unknown/debug/hello_plugin.wasm greet --input "Benjamin"
```

#### 带 host_fn 时

添加 host_fn 后，不支持使用 extism 了，得使用 xtp。

## Extism 资料

Extism Rust PDK Plugin

See more documentation at <https://github.com/extism/rust-pdk> and
[join us on Discord](https://extism.org/discord) for more help.

参考：<https://extism.org/docs/quickstart/plugin-quickstart>
