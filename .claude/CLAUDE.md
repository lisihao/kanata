# Kanata - AI Code Agent

## Project Overview

Kanata 是一个下一代 AI Code Agent，目标是超越 Claude Code。使用 Rust 构建，追求极致性能、内存安全和跨平台单二进制分发。

## Tech Stack

- **Language**: Rust 2024 edition, Cargo workspace
- **Async**: tokio 1.x
- **CLI/TUI**: ratatui + crossterm + syntect + termimad
- **HTTP**: reqwest + rustls
- **Serialization**: serde + serde_json + serde_yaml
- **Error**: thiserror (libraries) + anyhow (application binaries)
- **Logging**: tracing + tracing-subscriber
- **Testing**: cargo test + mockall + cargo-nextest
- **CI**: GitHub Actions

## Architecture

4-layer Cargo workspace:

```
kanata-cli      (L1 Presentation)   → ratatui TUI + CLI entry
kanata-core     (L2 Orchestration)  → AgentSession, tool dispatch, context management
kanata-tools    (L3 Capability)     → File/Search/Bash tool implementations
kanata-skills   (L3 Capability)     → Skill registry and management
kanata-model    (L3 Capability)     → LLM client, token management, streaming
kanata-types    (L4 Foundation)     → Shared types, traits, errors
```

## Key Conventions

- All public APIs use `snake_case`; types use `PascalCase`; constants use `SCREAMING_SNAKE_CASE`
- Every crate exposes traits; cross-crate communication is trait-based
- Error types: each library crate defines its own error enum via `thiserror`
- Application crate (`kanata-cli`) uses `anyhow::Result`
- All async functions use `tokio`; never mix runtimes
- Commit format: `type(scope): description` (Conventional Commits)

## Commands

- `cargo fmt --all` — format all code
- `cargo clippy --all-targets --all-features -- -D warnings` — lint
- `cargo nextest run` — run tests
- `cargo build --release` — release build
- `cargo audit` — security audit

## Important Files

- `docs/README.md` — Documentation index and reading guide
- `docs/product/PRD.md` — Product requirements
- `docs/product/PRD-SUPPLEMENT.md` — Strategic supplement (three barriers)
- `docs/architecture/EXECUTION-PLAN.md` — Architecture, team roles, schedule
- `docs/architecture/TECH-STANDARDS.md` — Full tech stack and coding standards
- `docs/process/COLLABORATION-GUIDE.md` — Team workflow
- `.claude/standards/` — Engineering standards (MUST/SHOULD/MAY)
