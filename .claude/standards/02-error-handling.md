# 02 - 错误处理规范

## 错误类型策略

- 🔴 `[MUST]` Library crate (`kanata-types`, `kanata-model`, `kanata-tools`, `kanata-skills`, `kanata-core`) 使用 `thiserror` 定义枚举错误
- 🔴 `[MUST]` Application crate (`kanata-cli`) 使用 `anyhow::Result` 快速传播
- 🔴 `[MUST]` 每个 library crate 定义自己的 `Error` enum 和 `Result<T>` 类型别名:
  ```rust
  // kanata-model/src/error.rs
  #[derive(Debug, thiserror::Error)]
  pub enum Error {
      #[error("API request failed: {0}")]
      ApiRequest(#[from] reqwest::Error),
      #[error("Rate limited, retry after {retry_after_secs}s")]
      RateLimited { retry_after_secs: u64 },
  }
  pub type Result<T> = std::result::Result<T, Error>;
  ```

## 错误传播

- 🔴 `[MUST]` 使用 `?` 操作符传播错误，禁止 `.unwrap()` 出现在非测试代码中
- 🔴 `[MUST]` `.expect("reason")` 仅用于逻辑上不可能失败的场景，message 必须说明为什么
- 🟡 `[SHOULD]` 跨层错误转换使用 `#[from]` 或手动 `impl From<>`
- 🟡 `[SHOULD]` 错误信息使用小写开头，不以句号结尾（Rust 惯例）

## Panic 策略

- 🔴 `[MUST]` 生产代码禁止 `panic!()` / `unwrap()` / `todo!()`
- 🔴 `[MUST]` `todo!()` 仅允许在 MVP 开发阶段，必须附带 `// TODO(owner): description` 注释
- 🟡 `[SHOULD]` 使用 `debug_assert!()` 替代 `assert!()` 用于内部不变量检查

## 日志规范

- 🔴 `[MUST]` 使用 `tracing` 宏（`tracing::info!`, `tracing::error!` 等），禁止 `println!`/`eprintln!`
- 🔴 `[MUST]` 日志级别遵循:
  | 级别 | 用途 |
  |------|------|
  | `error!` | 影响用户的错误，需要立即关注 |
  | `warn!` | 可恢复的异常，降级处理 |
  | `info!` | 关键业务流程节点（session 开始/结束、tool 调用） |
  | `debug!` | 开发调试信息（请求/响应详情） |
  | `trace!` | 极细粒度追踪（token 计数、逐字节流） |
- 🟡 `[SHOULD]` 结构化日志包含关键字段:
  ```rust
  tracing::info!(session_id = %id, tool = "bash", "tool execution started");
  ```

## 用户错误展示

- 🟡 `[SHOULD]` 面向用户的错误信息需友好可读，避免暴露内部细节
- 🟡 `[SHOULD]` 提供 `--verbose` 标志控制错误详情输出
- 🟢 `[MAY]` 为常见错误提供修复建议（"Did you mean...?"）
