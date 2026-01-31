# 05 - 目录结构与文件归档规范

## 项目根目录结构

- 🔴 `[MUST]` 项目根目录遵循以下布局:

```
kanata/
├── .claude/                  # AI 辅助开发配置与工程规范
│   ├── CLAUDE.md             # 项目概览（AI 上下文入口）
│   ├── settings.json         # 权限、钩子、环境变量
│   ├── README.md             # .claude/ 目录说明
│   ├── standards/            # 分级工程规范 (00-xx 编号)
│   ├── adrs/                 # 架构决策记录 (ADR-xxx)
│   ├── commands/             # 自定义 AI 命令
│   └── skills/               # 自定义 AI 技能
│
├── docs/                     # 项目文档（分类归档）
│   ├── README.md             # 文档索引与阅读指南
│   ├── product/              # 产品文档
│   │   ├── PRD.md
│   │   └── PRD-SUPPLEMENT.md
│   ├── architecture/         # 架构与技术文档
│   │   ├── EXECUTION-PLAN.md
│   │   └── TECH-STANDARDS.md
│   ├── process/              # 流程与协作文档
│   │   └── COLLABORATION-GUIDE.md
│   └── changelog/            # 版本变更记录
│       └── CHANGELOG.md
│
├── crates/                   # Cargo workspace 成员
│   ├── kanata-types/
│   ├── kanata-cli/
│   ├── kanata-core/
│   ├── kanata-tools/
│   ├── kanata-skills/
│   └── kanata-model/
│
├── tests/                    # 集成测试 & E2E 测试
├── benches/                  # 性能基准测试
├── scripts/                  # 构建/部署/工具脚本
│
├── Cargo.toml                # Workspace 根配置
├── Cargo.lock                # 依赖锁定
├── rustfmt.toml              # 格式化配置
├── clippy.toml               # Lint 配置（如需要）
├── LICENSE                   # 开源许可证
├── README.md                 # 项目入口 README
├── CONTRIBUTING.md           # 贡献指南
└── .gitignore
```

## 文档目录规范

### 分类原则

- 🔴 `[MUST]` `docs/` 按以下三类子目录归档，禁止在 `docs/` 根下直接放业务文档:
  | 子目录 | 归档内容 | 示例 |
  |--------|---------|------|
  | `product/` | 产品需求、用户故事、竞品分析 | PRD.md, PRD-SUPPLEMENT.md |
  | `architecture/` | 架构设计、技术选型、接口契约 | EXECUTION-PLAN.md, TECH-STANDARDS.md |
  | `process/` | 团队协作、工作流、会议记录 | COLLABORATION-GUIDE.md |
  | `changelog/` | 版本记录 | CHANGELOG.md |

- 🔴 `[MUST]` `docs/README.md` 作为文档索引必须存在，包含:
  - 文档地图（分类 + 一句话说明）
  - 推荐阅读顺序
  - 最近更新记录

### 文件命名

- 🔴 `[MUST]` 文档文件名统一使用 `UPPER-KEBAB-CASE.md`（全大写 + 连字符）
  - ✅ `PRD.md`, `PRD-SUPPLEMENT.md`, `TECH-STANDARDS.md`
  - ❌ `PRD-supplement.md`（小写混入）, `prd.md`（全小写）
- 🔴 `[MUST]` 规范文件使用 `数字前缀-kebab-case.md`（如 `01-code-style.md`）
- 🟡 `[SHOULD]` 文件名反映内容，不用缩写（`COLLABORATION-GUIDE.md` 而非 `COLLAB.md`）

### 文档内容格式

- 🔴 `[MUST]` 每个文档必须包含头部元信息:
  ```markdown
  # 文档标题

  **版本**: vX.Y
  **日期**: YYYY-MM-DD
  **作者**: Name
  **状态**: Draft | Active | Deprecated
  ```
- 🔴 `[MUST]` 代码块必须标注语言（` ```rust `, ` ```bash `, ` ```toml `, ` ```yaml `, ` ```text `）
- 🟡 `[SHOULD]` ASCII 图表使用 ` ```text ` 标注
- 🟡 `[SHOULD]` 文档间引用使用相对路径: `[参见架构](../architecture/EXECUTION-PLAN.md)`

## Crate 目录规范

- 🔴 `[MUST]` 每个 crate 目录结构:
  ```
  kanata-xxx/
  ├── Cargo.toml
  ├── src/
  │   ├── lib.rs          # re-export only, 无业务逻辑
  │   ├── traits.rs       # 公开 trait 定义（如有）
  │   ├── error.rs        # 错误类型定义（如有）
  │   ├── mod1.rs         # 功能模块
  │   └── mod1/           # 或子目录形式
  │       ├── mod.rs
  │       └── sub.rs
  └── tests/              # 集成测试（可选）
  ```

- 🔴 `[MUST]` `lib.rs` 只做 `pub mod` 声明和 `pub use` 重导出
- 🟡 `[SHOULD]` 单个源文件不超过 300 行
- 🟡 `[SHOULD]` 模块嵌套不超过 3 层

## ADR 归档规范

- 🟡 `[SHOULD]` 架构决策记录存放在 `.claude/adrs/`
- 🟡 `[SHOULD]` 文件命名: `ADR-NNN-短标题.md`（如 `ADR-001-use-rust.md`）
- 🟡 `[SHOULD]` 包含: 背景、决策、理由、后果、状态

## .gitignore 规范

- 🔴 `[MUST]` 以下内容必须排除:
  ```gitignore
  /target/
  *.log
  .env
  .env.*
  *.pem
  *.key
  .DS_Store
  Thumbs.db
  ```
- 🔴 `[MUST]` 禁止提交 `target/`、密钥文件、本地环境配置
