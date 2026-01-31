# 01 - Rust 代码风格规范

## 格式化

- 🔴 `[MUST]` 所有代码必须通过 `cargo fmt --all --check`，CI 强制检查
- 🔴 `[MUST]` 使用项目根 `rustfmt.toml` 配置，禁止个人覆盖
- 🔴 `[MUST]` `rustfmt.toml` 配置:
  ```toml
  edition = "2024"
  max_width = 100
  tab_spaces = 4
  use_field_init_shorthand = true
  use_try_shorthand = true
  ```

## 命名

- 🔴 `[MUST]` 函数、方法、变量、模块: `snake_case`
- 🔴 `[MUST]` 类型 (struct, enum, trait): `PascalCase`
- 🔴 `[MUST]` 常量、静态变量: `SCREAMING_SNAKE_CASE`
- 🔴 `[MUST]` Crate 名称: `kanata-xxx` (kebab-case)
- 🟡 `[SHOULD]` 布尔变量以 `is_`/`has_`/`can_`/`should_` 开头
- 🟡 `[SHOULD]` 返回 `Option` 的方法以 `try_` 或 `find_` 开头
- 🟡 `[SHOULD]` 异步方法不加 `_async` 后缀（Rust 惯例）

## Clippy

- 🔴 `[MUST]` `cargo clippy --all-targets --all-features -- -D warnings` 零警告
- 🔴 `[MUST]` 禁止 `#[allow(clippy::*)]` 除非附带注释说明原因
- 🟡 `[SHOULD]` 启用额外 lint:
  ```rust
  #![warn(clippy::pedantic)]
  #![allow(clippy::module_name_repetitions)] // crate 级别允许
  ```

## 模块组织

- 🔴 `[MUST]` 每个 crate 的 `lib.rs` 只做 re-export，不含业务逻辑
- 🔴 `[MUST]` 公开 trait 定义在独立文件中（如 `traits.rs` 或 `trait/mod.rs`）
- 🟡 `[SHOULD]` 单文件不超过 300 行，超过则拆分子模块
- 🟡 `[SHOULD]` `pub use` 集中在 `lib.rs` 顶部
- 🟢 `[MAY]` 内部辅助函数放在文件底部或独立的 `helpers.rs`

## 依赖管理

- 🔴 `[MUST]` 所有依赖在 workspace `Cargo.toml` 中统一声明版本
- 🔴 `[MUST]` 子 crate 使用 `dep.workspace = true` 引用
- 🔴 `[MUST]` 禁止引入 `star < 1000` 或最近 6 个月无更新的 crate
- 🟡 `[SHOULD]` 新增依赖需在 PR 中说明选型理由
- 🟡 `[SHOULD]` 定期运行 `cargo audit` 检查安全漏洞

## 文档注释

- 🟡 `[SHOULD]` 所有 `pub` 函数和 trait 方法需要 `///` 文档注释
- 🟡 `[SHOULD]` 文档注释包含简短说明 + 参数/返回值说明
- 🟢 `[MAY]` 复杂函数提供 `# Examples` 代码块
- 🔴 `[MUST]` 禁止无意义注释（如 `// 创建变量`），代码应自解释

## 类型使用

- 🔴 `[MUST]` 跨 crate 共享类型定义在 `kanata-types` 中
- 🔴 `[MUST]` 使用 `&str` 而非 `String` 作为函数参数（非所有权场景）
- 🟡 `[SHOULD]` 优先使用 `impl Into<String>` 提供灵活 API
- 🟡 `[SHOULD]` 避免 `clone()` 大对象，优先使用引用或 `Arc`
- 🟢 `[MAY]` 小型配置结构体可 `#[derive(Clone)]`
