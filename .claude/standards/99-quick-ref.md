# 99 - 速查表

## 日常开发命令

```bash
# 格式化
cargo fmt --all

# Lint 检查
cargo clippy --all-targets --all-features -- -D warnings

# 运行所有测试
cargo nextest run

# 运行单个 crate 测试
cargo nextest run -p kanata-core

# 构建 (debug)
cargo build

# 构建 (release)
cargo build --release

# 安全审计
cargo audit

# 检查编译
cargo check --all-targets
```

## 🔴 MUST 速查 (严禁违反)

| 规则 | 检查方式 |
|------|---------|
| `cargo fmt` 通过 | CI 自动 |
| `clippy` 零警告 | CI 自动 |
| 禁止 `unwrap()` (非测试代码) | clippy + Review |
| 禁止 `println!` (用 `tracing`) | clippy + Review |
| 禁止直接 push `main` | GitHub Branch Protection |
| PR 至少 1 人 Review | GitHub 设置 |
| CI 全绿才合并 | GitHub 设置 |
| Conventional Commits 格式 | Review |
| 依赖版本 workspace 统一管理 | Review |
| Library crate 用 `thiserror` | Review |
| 每个 PR 有对应测试 | Review |

## 🟡 SHOULD 速查 (应该遵守)

| 规则 | 备注 |
|------|------|
| 单文件 ≤ 300 行 | 超出需拆分 |
| PR ≤ 400 行 | 大功能拆分 |
| 分支存活 ≤ 2 天 | 及时合并 |
| pub 函数有文档注释 | `///` |
| 布尔变量 `is_`/`has_` 前缀 | 可读性 |
| 结构化日志带关键字段 | `tracing::info!(k=v)` |

## Crate 职责速查

| Crate | 职责 | Owner |
|-------|------|-------|
| `kanata-types` | 共享类型、trait、error | 共有 |
| `kanata-cli` | TUI 界面、用户交互 | Dev A (Baoxing Huai) |
| `kanata-core` | Session 编排、Tool 分发 | Dev B (Junjie Duan) |
| `kanata-tools` | 文件/搜索/Bash 工具 | Dev B (Junjie Duan) |
| `kanata-skills` | Skill 注册与管理 | Dev B (Junjie Duan) |
| `kanata-model` | LLM 客户端、流式解析 | Dev C (Sihao Li) |

## 新增依赖 Checklist

- [ ] GitHub stars ≥ 1000
- [ ] 最近 6 个月有更新
- [ ] 在 workspace `Cargo.toml` 声明版本
- [ ] 子 crate 用 `dep.workspace = true`
- [ ] PR 中说明选型理由
