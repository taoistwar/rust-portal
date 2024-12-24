# 门户 API

目的：管理各种插件服务，提供统一网关。

## Roadmap

- [x] Axum "Hello World"
- [x] Extism "Hello World"，插件函数（plugin_fn）。
- [x] Extism 宿主函数（host_fn）
- [x] 插件封装 plugin-mock 用于单元测试
- [ ] libp2p2 （发现节点、连接节点、发现数据、传输数据）
- [ ] 插件配置清单

## 功能

### 通知

- 插件
  - 邮件
  - 电话
  - 钉钉
- 联系人
  - 姓名、邮箱、手机号、
- 通知列表
  - 名称、联系人列表
- 接口
  - 内容
  - 标题

### 监控

- 条件
  - 开始条件
  - 恢复条件
- 周期
  - 每分钟？
- 通知

### 告警

- 条件：
  - 无：默认值，必然触发。相当于闹铃。
- 周期：
  - 每天：结束日期、
  - 工作日：
  - 股市营业日：
  - 单次
- 通知

## 参考资料

### axum

### Extism

- <https://extism.org/docs/overview>
- xtp-test <https://github.com/dylibso/xtp-test-rust>

### libp2p

- <https://docs.libp2p.io/concepts/>
- <https://github.com/libp2p/rust-libp2p>

### dioxus

- <https://dioxuslabs.com/learn/0.5/guide>
- <https://github.com/DioxusLabs/dioxus>

### tokio
