# .claude/ 目录说明

本目录包含 Kanata 项目的 AI 辅助开发配置和工程规范。

## 目录结构

```
.claude/
├── CLAUDE.md              # 项目概览，AI 助手上下文入口
├── settings.json          # 权限、钩子、环境变量配置
├── README.md              # 本文件
├── standards/             # 工程规范（分级约束）
│   ├── 00-overview.md     # 规范总览与约束等级定义
│   ├── 01-code-style.md   # Rust 代码风格规范
│   ├── 02-error-handling.md # 错误处理规范
│   ├── 03-testing.md      # 测试规范
│   ├── 04-git-workflow.md # Git 工作流规范
│   └── 99-quick-ref.md    # 速查表
├── adrs/                  # 架构决策记录
├── commands/              # 自定义 AI 命令
└── skills/                # 自定义 AI 技能
```

## 约束等级

- 🔴 **MUST** — 必须遵守，严禁修改，CI 强制检查
- 🟡 **SHOULD** — 应该遵守，特殊情况可在 PR 中说明理由后豁免
- 🟢 **MAY** — 建议遵守，团队共识后可调整
