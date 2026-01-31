# Kanata 技术栈与工程规范

**版本**: v1.0
**日期**: 2026-01-31

---

## 一、技术栈决策

### 1.1 选型原则

| 原则 | 含义 | 反例 |
|------|------|------|
| **高效** | 编译/运行性能优先，开发效率并重 | 不用 Python 写 CLI (启动慢) |
| **安全** | 内存安全 + 类型安全 + 运行时安全 | 不用 C/C++ (内存不安全) |
| **可复用** | 优先选生态成熟、社区活跃的库 | 不造轮子 |
| **业界验证** | 选大厂/明星项目在用的方案 | 不用 star < 1K 的实验库 |

### 1.2 核心技术栈

```
┌──────────────────────────────────────────────────────────────┐
│  Kanata MVP 技术栈全景                                        │
│                                                              │
│  Language:     Rust 1.75+ (2024 edition)                     │
│  Async:        tokio 1.x (runtime + macros)                  │
│  Serialization: serde + serde_json + serde_yaml              │
│                                                              │
│  ── CLI 层 (Dev A) ──────────────────────────────────────    │
│  TUI Framework:  ratatui 0.28+                               │
│  Terminal:        crossterm 0.28+                             │
│  Markdown:        termimad 0.30+                             │
│  Syntax Highlight: syntect 5.x                               │
│  Diff Display:    similar 2.x                                │
│                                                              │
│  ── 编排层 (Dev B) ──────────────────────────────────────    │
│  File Search:     globset 0.4+ (ripgrep 底层)                │
│  Regex Search:    grep-regex 0.1+ / regex 1.x                │
│  Process:         tokio::process (Bash 执行)                  │
│  Template:        minijinja 2.x (Prompt 模板)                 │
│                                                              │
│  ── 模型层 (Dev C) ──────────────────────────────────────    │
│  HTTP:            reqwest 0.12+ (with rustls)                │
│  SSE:             reqwest-eventsource 0.6+ 或手写解析         │
│  Retry:           backon 1.x                                 │
│  Rate Limit:      governor 0.6+                              │
│                                                              │
│  ── 基础层 (共用) ────────────────────────────────────────   │
│  Error:           thiserror 2.x + anyhow 1.x                │
│  Logging:         tracing 0.1+ + tracing-subscriber          │
│  Config:          serde_yaml + dirs 5.x (~/.kanata/ 路径)     │
│  UUID:            uuid 1.x                                   │
│  Time:            chrono 0.4+                                │
│  Test Mock:       mockall 0.13+                              │
│  Async Trait:     async-trait 0.1+ (直到 Rust AFIT 稳定)      │
│                                                              │
│  ── 构建与发布 ──────────────────────────────────────────    │
│  Build:           cargo + cargo-nextest (并行测试)            │
│  Lint:            clippy (deny warnings)                     │
│  Format:          rustfmt (强制)                              │
│  Security Audit:  cargo-audit (CI 中运行)                     │
│  Cross Compile:   cross 0.2+ (多平台构建)                     │
│  CI/CD:           GitHub Actions                             │
│  Release:         cargo-dist 或 GitHub Release                │
│                                                              │
│  ── 后续阶段 (非 MVP) ───────────────────────────────────   │
│  Web Frontend:    React 19 + TypeScript 5.x + Vite           │
│  Web Editor:      Monaco Editor                              │
│  Realtime:        WebSocket + Yjs (CRDT)                     │
│  Database:        PostgreSQL 16 + SQLx (Rust)                │
│  Cache:           Redis 7                                    │
│  Analytics:       ClickHouse                                 │
│  Vector Store:    Qdrant                                     │
│  Container:       Docker + Kubernetes                        │
│  Observability:   OpenTelemetry + Grafana                    │
│  Mobile:          React Native 0.76+                         │
└──────────────────────────────────────────────────────────────┘
```

### 1.3 选型理由详述

| 技术 | 选它 | 不选什么 | 理由 |
|------|------|---------|------|
| **Rust** | ✓ | Go, TypeScript, Python | CLI 冷启动 < 50ms；内存安全无 GC 停顿；跨平台单二进制；Claude Code (TS) 启动慢是已知痛点 |
| **ratatui** | ✓ | cursive, ink (TS) | Rust 生态最成熟的 TUI 库；Netflix/AWS 在用；活跃维护 |
| **reqwest + rustls** | ✓ | hyper, ureq | reqwest 封装好用；rustls 避免系统 OpenSSL 依赖，跨平台编译无痛 |
| **tokio** | ✓ | async-std, smol | 事实标准；reqwest/tracing 等库都基于 tokio |
| **serde_yaml** | ✓ | toml, json (配置) | YAML 适合深嵌套配置和 Skill 定义；人类可读性最佳 |
| **syntect** | ✓ | tree-sitter | VS Code 同款高亮引擎；开箱即用支持 200+ 语言；tree-sitter MVP 太重 |
| **thiserror + anyhow** | ✓ | 手写 Error enum | thiserror 给库用 (类型化错误)，anyhow 给应用用 (快速开发)；业界标准搭配 |
| **tracing** | ✓ | log, env_logger | 结构化日志 + 分布式追踪；后续接 OpenTelemetry 零改造 |
| **globset + grep-regex** | ✓ | 手写 glob/grep | ripgrep 底层库，经过十亿级文件量验证；性能远超手写 |
| **GitHub Actions** | ✓ | GitLab CI, CircleCI | 与代码仓库同平台；免费额度足够；矩阵构建支持好 |

### 1.4 关键依赖版本锁定 (Cargo.toml)

```toml
# === workspace Cargo.toml ===
[workspace]
resolver = "2"
members = [
    "crates/kanata-types",
    "crates/kanata-cli",
    "crates/kanata-core",
    "crates/kanata-tools",
    "crates/kanata-skills",
    "crates/kanata-model",
]

[workspace.dependencies]
# Async
tokio = { version = "1.42", features = ["full"] }
async-trait = "0.1"
futures = "0.3"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
serde_yaml = "0.9"

# HTTP & Networking
reqwest = { version = "0.12", default-features = false, features = [
    "json", "stream", "rustls-tls"
] }

# CLI & TUI
ratatui = "0.29"
crossterm = "0.28"
syntect = "5.2"
termimad = "0.30"
similar = "2.6"

# File Operations
globset = "0.4"
regex = "1.11"
walkdir = "2.5"

# Error Handling
thiserror = "2.0"
anyhow = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Utilities
chrono = "0.4"
uuid = { version = "1.11", features = ["v4"] }
dirs = "5.0"
clap = { version = "4.5", features = ["derive"] }

# Template
minijinja = "2.5"

# Testing
mockall = "0.13"

[workspace.package]
version = "0.1.0"
edition = "2024"
license = "MIT OR Apache-2.0"
repository = "https://github.com/JUNJIE-DUAN/kanata"
```

---

## 二、工程规范

### 2.1 代码风格

```toml
# rustfmt.toml (项目根目录)
edition = "2024"
max_width = 100
tab_spaces = 4
use_small_heuristics = "Max"
imports_granularity = "Module"
group_imports = "StdExternalCrate"
reorder_imports = true
```

```toml
# clippy.toml (项目根目录)
cognitive-complexity-threshold = 25
too-many-arguments-threshold = 8
```

```toml
# .cargo/config.toml
[target.'cfg(all())']
rustflags = [
    "-D", "warnings",           # 所有 warning 视为 error
    "-W", "clippy::pedantic",   # 更严格的 lint
    "-A", "clippy::module_name_repetitions",  # 允许模块名重复
    "-A", "clippy::must_use_candidate",
]
```

**强制规则** (CI 拦截):
1. `cargo fmt --check` 必须通过 — 不通过不允许合并
2. `cargo clippy` 零 warning — 不允许 `#[allow]` 绕过 (除非 PR 中注释说明理由)
3. 每个 `pub fn` 必须有文档注释 (`///`)
4. 每个 `pub` 类型必须有文档注释
5. 不允许 `unwrap()` — 使用 `?` 或 `expect("reason")` 并说明原因
6. 不允许 `unsafe` — 除非 PR 中附带安全性论证

### 2.2 命名规范

| 类型 | 规范 | 示例 |
|------|------|------|
| crate 名 | `kanata-xxx` (kebab-case) | `kanata-core`, `kanata-model` |
| 模块名 | snake_case | `token_tracker`, `key_pool` |
| struct / enum | PascalCase | `AgentSession`, `StreamEvent` |
| trait | PascalCase (名词/形容词) | `LLMClient`, `Tool`, `Configurable` |
| 函数 / 方法 | snake_case (动词开头) | `send_message()`, `parse_stream()` |
| 常量 | SCREAMING_SNAKE_CASE | `MAX_RETRIES`, `DEFAULT_MODEL` |
| 类型参数 | 单大写字母或描述性 | `T`, `E`, `Client` |
| 文件名 | snake_case | `key_pool.rs`, `stream_parser.rs` |

### 2.3 目录与模块规范

```
每个 crate 的结构:

crates/kanata-xxx/
├── Cargo.toml
├── src/
│   ├── lib.rs          # 只做 pub mod 声明和 re-export
│   ├── feature_a.rs    # 每个文件 < 500 行
│   ├── feature_b.rs
│   └── sub_module/     # 超过 3 个相关文件时创建子目录
│       ├── mod.rs
│       ├── part1.rs
│       └── part2.rs
└── tests/              # 集成测试 (跨模块)
    └── integration.rs
```

**规则**:
- 单文件不超过 **500 行** (超过必须拆分)
- 单函数不超过 **50 行** (超过必须拆分)
- 每个模块有明确的单一职责
- `lib.rs` 只做 `pub mod` 和 `pub use` re-export，不写逻辑

### 2.4 错误处理规范

```rust
// ═══ 库 crate (kanata-types, kanata-tools 等) 用 thiserror ═══

#[derive(Debug, thiserror::Error)]
pub enum KanataError {
    #[error("File not found: {path}")]
    FileNotFound { path: String },

    #[error("Tool execution failed: {tool_name}: {reason}")]
    ToolError { tool_name: String, reason: String },

    #[error("Model API error: {status} {message}")]
    ModelError { status: u16, message: String },

    #[error("Rate limited, retry after {retry_after_secs}s")]
    RateLimited { retry_after_secs: u64 },

    #[error("Context window exceeded: {used}/{limit} tokens")]
    ContextOverflow { used: u32, limit: u32 },

    #[error("Configuration error: {0}")]
    Config(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),

    #[error(transparent)]
    Http(#[from] reqwest::Error),

    #[error(transparent)]
    Json(#[from] serde_json::Error),
}

// ═══ 应用 crate (kanata-cli) 可以用 anyhow 简化 ═══

use anyhow::{Context, Result};

fn load_config() -> Result<KanataConfig> {
    let path = dirs::home_dir()
        .context("Cannot determine home directory")?
        .join(".kanata/config.yaml");
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Failed to read config: {}", path.display()))?;
    serde_yaml::from_str(&content)
        .context("Failed to parse config.yaml")
}
```

**规则**:
- 库 crate → `thiserror` 定义类型化错误
- 应用 crate → `anyhow` + `.context()` 增加上下文信息
- 禁止裸 `.unwrap()` / `.expect("unreachable")`
- 所有 I/O 错误必须包含路径/URL 等上下文信息

### 2.5 日志规范

```rust
use tracing::{debug, info, warn, error, instrument};

// 函数级追踪
#[instrument(skip(self, messages), fields(model = %self.model_name()))]
async fn chat_stream(&self, messages: &[Message], ...) -> Result<...> {
    info!(message_count = messages.len(), "Starting chat stream");

    // 正常流程用 debug
    debug!(token_count = usage.input_tokens, "Request sent");

    // 需要关注的用 info
    info!(tokens = usage.total(), cost = %cost, "Chat completed");

    // 预期内的异常用 warn
    warn!(retry_after = secs, "Rate limited, retrying");

    // 非预期错误用 error
    error!(status = resp.status(), body = %body, "API request failed");
}
```

**日志等级规则**:

| 等级 | 用途 | 示例 |
|------|------|------|
| `error` | 不可恢复错误，需要人工介入 | API 鉴权失败、文件系统损坏 |
| `warn` | 可恢复异常，系统自动处理 | Rate limit 重试、Key 失效切换 |
| `info` | 关键业务事件，生产环境可见 | 会话开始/结束、工具调用、Token 消耗 |
| `debug` | 开发调试信息 | 请求/响应详情、上下文装配过程 |
| `trace` | 极细粒度 | SSE 每个 event、JSON 解析步骤 |

**默认日志级别**: `info` (可通过 `KANATA_LOG=debug` 环境变量调整)

### 2.6 测试规范

**测试分层**:

```
┌────────────────────────────────────────────────┐
│  E2E Tests (kanata-test/)                      │ ← 数量少，每日 CI 跑
│  测试完整用户场景                               │
├────────────────────────────────────────────────┤
│  Integration Tests (各 crate/tests/)           │ ← 中等数量，每次 PR 跑
│  测试模块间交互                                 │
├────────────────────────────────────────────────┤
│  Unit Tests (各 crate/src/ 内 #[cfg(test)])    │ ← 大量，每次 commit 跑
│  测试单个函数/struct                            │
└────────────────────────────────────────────────┘
```

**测试命名规范**:
```rust
#[cfg(test)]
mod tests {
    use super::*;

    // 格式: test_<被测功能>_<输入条件>_<期望行为>
    #[test]
    fn test_read_tool_existing_file_returns_content() { ... }

    #[test]
    fn test_read_tool_missing_file_returns_error() { ... }

    #[tokio::test]
    async fn test_anthropic_client_stream_parses_text_delta() { ... }

    #[tokio::test]
    async fn test_agent_loop_tool_use_dispatches_correctly() { ... }
}
```

**覆盖率要求**:

| crate | 最低覆盖率 | 理由 |
|-------|-----------|------|
| kanata-types | 不要求 | 纯类型定义 |
| kanata-tools | **≥ 80%** | 每个工具必须有正常/异常路径测试 |
| kanata-model | **≥ 70%** | 流式解析必须覆盖所有 event 类型 |
| kanata-core | **≥ 60%** | Agent 循环复杂，用 Mock 测核心路径 |
| kanata-cli | **≥ 40%** | UI 代码难测，测命令解析和配置逻辑 |

**Mock 使用规范**:
```rust
// Dev C 提供的 MockLLMClient — Dev B 使用
use kanata_model::MockLLMClient;

#[tokio::test]
async fn test_agent_handles_tool_use_response() {
    let mut mock = MockLLMClient::new();
    mock.expect_chat_stream()
        .returning(|_, _, _| {
            Ok(Box::pin(futures::stream::iter(vec![
                Ok(StreamEvent::ToolUseStart {
                    id: "1".into(),
                    name: "Read".into(),
                }),
                Ok(StreamEvent::ToolUseDelta(r#"{"path":"/tmp/test.rs"}"#.into())),
                Ok(StreamEvent::ToolUseEnd),
            ])))
        });

    let agent = Agent::new(Box::new(mock), tools);
    let events: Vec<_> = agent.send_message("read the file").collect().await;
    assert!(events.iter().any(|e| matches!(e, AgentEvent::ToolStart { .. })));
}
```

### 2.7 安全规范

| 层面 | 规则 | 实施方式 |
|------|------|---------|
| **依赖安全** | 不使用已知漏洞的 crate 版本 | CI 中跑 `cargo audit` |
| **密钥安全** | API Key 不落日志、不存明文 | 日志中 Key 自动脱敏 `sk-...xxxx` |
| **文件安全** | 不允许读写项目目录之外的文件 | Tool 层路径白名单校验 |
| **命令安全** | 拦截危险 Bash 命令 | 黑名单: `rm -rf /`, `mkfs`, `dd`, `:(){:|:&};:` 等 |
| **输入安全** | 防止 prompt injection | System Prompt 中的安全边界指令 |
| **传输安全** | 所有 API 调用走 HTTPS | reqwest 默认 + rustls |
| **内存安全** | Rust 语言保证 | 编译期检查 + 禁止 unsafe |

**安全拦截示例**:
```rust
// kanata-tools/src/safety.rs

const DANGEROUS_COMMANDS: &[&str] = &[
    "rm -rf /",
    "rm -rf ~",
    "mkfs",
    "dd if=",
    ":(){:|:&};:",
    "chmod -R 777 /",
    "curl | sh",
    "wget | sh",
    "> /dev/sda",
];

const DANGEROUS_PATTERNS: &[&str] = &[
    r"rm\s+(-[a-zA-Z]*f[a-zA-Z]*\s+)?/(?!tmp)",  // rm -rf / (允许 /tmp)
    r">\s*/dev/",                                    // 写入设备文件
    r"chmod\s+-R\s+777\s+/",                        // 全局权限修改
];

pub fn is_dangerous(command: &str) -> Option<&'static str> {
    // 检查精确匹配和模式匹配
    // 返回拒绝原因
}
```

### 2.8 Git 规范

**Commit Message 格式** (Conventional Commits):
```
<type>(<scope>): <subject>

<body>

<footer>
```

| type | 说明 |
|------|------|
| `feat` | 新功能 |
| `fix` | Bug 修复 |
| `refactor` | 重构 (不改变行为) |
| `docs` | 文档 |
| `test` | 测试 |
| `chore` | 构建/CI/依赖更新 |
| `perf` | 性能优化 |

| scope | 说明 |
|-------|------|
| `cli` | kanata-cli |
| `core` | kanata-core |
| `tools` | kanata-tools |
| `skills` | kanata-skills |
| `model` | kanata-model |
| `types` | kanata-types |

**示例**:
```
feat(model): add Anthropic streaming client

Implement SSE parsing for Claude Messages API.
Supports text_delta and tool_use event types.

Closes #12
```

**分支命名**:
```
dev/a-<feature>     # Dev A 的功能分支
dev/b-<feature>     # Dev B 的功能分支
dev/c-<feature>     # Dev C 的功能分支
fix/<issue-number>  # Bug 修复
```

### 2.9 PR 规范

**PR 模板** (`.github/pull_request_template.md`):
```markdown
## Summary
<!-- 一句话描述改了什么 -->

## Changes
- [ ] 改动 1
- [ ] 改动 2

## Testing
- [ ] 单元测试通过
- [ ] 手动测试描述

## Checklist
- [ ] `cargo fmt` 通过
- [ ] `cargo clippy` 零 warning
- [ ] 新增 `pub` API 有文档注释
- [ ] 无 `unwrap()` 使用
- [ ] 无硬编码密钥或路径
```

**Review 轮转**:
| 提交者 | Reviewer |
|--------|---------|
| Dev A | Dev B |
| Dev B | Dev C |
| Dev C | Dev A |

### 2.10 CI/CD Pipeline

```yaml
# .github/workflows/ci.yml
name: CI

on:
  push:
    branches: [main]
  pull_request:
    branches: [main]

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy
      - uses: Swatinem/rust-cache@v2

      - name: Format check
        run: cargo fmt --all --check

      - name: Clippy
        run: cargo clippy --workspace --all-targets -- -D warnings

      - name: Security audit
        run: cargo install cargo-audit && cargo audit

  test:
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        os: [ubuntu-latest, macos-latest, windows-latest]
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2

      - name: Run tests
        run: cargo test --workspace

  build:
    needs: [check, test]
    runs-on: ${{ matrix.os }}
    strategy:
      matrix:
        include:
          - os: ubuntu-latest
            target: x86_64-unknown-linux-gnu
          - os: macos-latest
            target: aarch64-apple-darwin
          - os: windows-latest
            target: x86_64-pc-windows-msvc
    steps:
      - uses: actions/checkout@v4
      - uses: dtolnay/rust-toolchain@stable
      - uses: Swatinem/rust-cache@v2

      - name: Build release
        run: cargo build --release --target ${{ matrix.target }}

      - uses: actions/upload-artifact@v4
        with:
          name: kanata-${{ matrix.target }}
          path: target/${{ matrix.target }}/release/kanata*
```

---

## 三、配置文件规范

### 3.1 用户配置 (`~/.kanata/config.yaml`)

```yaml
# Kanata 配置文件
# 首次运行时由引导程序创建

# 默认模型
default_model: claude-sonnet-4

# API Keys (首次运行时引导配置)
api_keys:
  anthropic: "sk-ant-xxxxx"
  openai: "sk-xxxxx"          # 可选，用于 GPT/DeepSeek 等 OpenAI 兼容 API
  deepseek: "sk-xxxxx"        # 可选

# 模型配置
models:
  claude-opus-4.5:
    provider: anthropic
    model_id: claude-opus-4-5-20251101
    max_tokens: 16384
  claude-sonnet-4:
    provider: anthropic
    model_id: claude-sonnet-4-20250514
    max_tokens: 16384
  deepseek-v3:
    provider: openai_compatible
    model_id: deepseek-chat
    base_url: "https://api.deepseek.com/v1"
    api_key_ref: deepseek

# 信任等级 (1-4, 默认 1)
trust_level: 1

# 日志等级
log_level: info

# 主题
theme: auto  # auto | dark | light
```

### 3.2 项目配置 (`.kanata/project.yaml`)

```yaml
# 项目级 Kanata 配置 (可 commit 到 Git)

# 项目描述 (给 Agent 的上下文)
project:
  name: "My App"
  description: "E-commerce platform built with React and Go"
  tech_stack: ["react", "typescript", "go", "postgresql"]

# 项目规则 (类似 CLAUDE.md)
rules:
  - "Use functional components with hooks, not class components"
  - "API responses follow { data, error, meta } format"
  - "Database fields use snake_case, TypeScript uses camelCase"

# 文件访问限制
file_access:
  allow:
    - "src/**"
    - "tests/**"
    - "docs/**"
  deny:
    - ".env*"
    - "*.pem"
    - "secrets/**"
```

---

## 四、Rust 开发环境标准化

### 4.1 开发环境要求

```bash
# 所有开发者统一安装以下工具
rustup default stable              # Rust stable channel
rustup component add rustfmt       # 格式化
rustup component add clippy        # Lint
cargo install cargo-nextest        # 并行测试运行器
cargo install cargo-audit          # 安全审计
cargo install cargo-watch          # 文件变化自动重编译
```

### 4.2 开发命令速查

```bash
# 日常开发
cargo watch -x check              # 文件变化自动检查
cargo watch -x 'test -p kanata-tools'  # 自动跑特定 crate 测试

# 提交前
cargo fmt --all                    # 格式化
cargo clippy --workspace           # Lint
cargo nextest run --workspace      # 并行测试

# 运行
cargo run -p kanata-cli            # 运行 CLI
cargo run -p kanata-cli -- --verbose  # Debug 模式
KANATA_LOG=debug cargo run -p kanata-cli  # 详细日志
```

---

*本文档由 Kanata Team 制定，版本 v1.0*
