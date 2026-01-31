# 03 - 测试规范

## 测试框架

- 🔴 `[MUST]` 使用 `cargo nextest run` 运行测试（并行、输出友好）
- 🔴 `[MUST]` Mock 使用 `mockall` crate
- 🟡 `[SHOULD]` 集成测试放在 `tests/` 目录，单元测试用 `#[cfg(test)] mod tests`

## 覆盖率要求

- 🔴 `[MUST]` 每个 PR 新增代码必须有对应测试
- 🟡 `[SHOULD]` 各 crate 覆盖率目标:
  | Crate | 目标覆盖率 |
  |-------|-----------|
  | `kanata-types` | ≥ 90% |
  | `kanata-model` | ≥ 80% |
  | `kanata-core` | ≥ 80% |
  | `kanata-tools` | ≥ 75% |
  | `kanata-skills` | ≥ 70% |
  | `kanata-cli` | ≥ 60% |
- 🟢 `[MAY]` 使用 `cargo-llvm-cov` 生成覆盖率报告

## 测试命名

- 🟡 `[SHOULD]` 测试函数命名: `test_<function>_<scenario>_<expected>`
  ```rust
  #[test]
  fn test_parse_config_missing_key_returns_error() { ... }

  #[tokio::test]
  async fn test_llm_client_timeout_retries_three_times() { ... }
  ```

## Mock 规范

- 🔴 `[MUST]` 跨 crate 接口使用 trait，通过 `mockall` 生成 Mock
- 🟡 `[SHOULD]` Mock 对象在测试模块内构造，不共享全局状态
- 🟡 `[SHOULD]` 异步 trait mock 使用 `#[automock]` + `async-trait`

## CI 集成

- 🔴 `[MUST]` CI 流程必须包含: `fmt check` → `clippy` → `test` → `audit`
- 🔴 `[MUST]` 测试必须在 Linux/macOS/Windows 三平台通过
- 🟡 `[SHOULD]` CI 测试超时上限: 单个测试 30s，总计 10min
- 🟢 `[MAY]` 使用 `#[ignore]` 标记慢测试，CI 中单独运行
