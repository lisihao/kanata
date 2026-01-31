# 00 - 工程规范总览

## 约束等级定义

| 等级 | 标记 | 含义 | 违反后果 |
|------|------|------|---------|
| 🔴 **MUST** | `[MUST]` | 必须遵守，严禁修改 | CI 自动拒绝，PR 不可合并 |
| 🟡 **SHOULD** | `[SHOULD]` | 应该遵守 | PR 中说明理由后可豁免，需 Reviewer 确认 |
| 🟢 **MAY** | `[MAY]` | 建议遵守 | 团队共识后可调整 |

## 规范变更流程

1. 🔴 MUST 级规范变更需要 **全员一致同意** + Sponsor 审批，提交 ADR
2. 🟡 SHOULD 级规范变更需要 **2/3 团队成员同意**
3. 🟢 MAY 级规范变更需要 **任意 1 名成员提 PR + 1 人 Review**

## 规范文件索引

| 文件 | 覆盖范围 |
|------|---------|
| `01-code-style.md` | Rust 命名、格式化、模块组织、clippy |
| `02-error-handling.md` | 错误类型、传播、日志、panic 策略 |
| `03-testing.md` | 测试策略、覆盖率、Mock、CI 集成 |
| `04-git-workflow.md` | 分支、提交、PR、Review、发布 |
| `99-quick-ref.md` | 日常开发速查表 |

## 适用范围

本规范适用于 Kanata 项目所有 Rust 代码（workspace 内所有 crate），以及相关的 CI/CD 配置、文档和脚本。
