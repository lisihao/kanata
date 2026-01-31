# Kanata Code Agent — 产品需求文档 (PRD)

**版本**: v1.1
**日期**: 2026-01-31
**状态**: Draft

---

## 1. 产品愿景

打造下一代 AI Code Agent —— **Kanata**，在能力维度全面对标并超越 Claude Code，同时解决其在多模型支持、团队协作、成本控制、多终端覆盖等方面的短板，成为业界最具竞争力的智能编程助手平台。

**核心差异化定位**:
- Claude Code = 单模型 + 单用户 + CLI only
- Kanata = 多模型 + 多团队 + 全终端 + 成本可控 + 自我进化

---

## 2. 核心原则：AI Native 全生命周期

> **Kanata 不是"用 AI 辅助开发的工具"，而是"用 AI 构建、用 AI 运行、用 AI 进化的 AI 原生系统"。**

整个产品的全生命周期都以 AI Native 为第一原则贯穿：

### 2.1 AI Native 原则矩阵

| 生命周期阶段 | 传统方式 | Kanata AI Native 方式 |
|-------------|---------|----------------------|
| **需求分析** | 人工撰写 PRD | AI 分析竞品 + 用户反馈，自动生成需求建议 |
| **架构设计** | 架构师手动设计 | AI 推荐架构方案，自动评估 Trade-off |
| **编码实现** | 人写代码，AI 辅助 | AI 写代码，人审核批准 |
| **代码审查** | 人工 Review | AI 自动 Review + 人工终审 |
| **测试** | 人写测试用例 | AI 自动生成测试 + 变异测试 + 覆盖率分析 |
| **部署** | CI/CD 脚本 | AI 分析变更影响，自动决定部署策略 |
| **监控** | 规则告警 | AI 异常检测 + 自动根因分析 |
| **迭代** | 人工排期 | AI 分析竞品动态 + 用户数据，自动建议迭代优先级 |
| **文档** | 人工编写维护 | AI 自动生成并随代码同步更新 |
| **Onboarding** | 人工培训 | AI Agent 交互式引导新成员 |

### 2.2 AI Native 设计准则

1. **AI First, Human Approve**: 所有可自动化的环节默认由 AI 执行，人类负责审批和决策
2. **自描述系统**: 系统的每个组件能被 AI 理解和解释，配置即文档，代码即规范
3. **反馈驱动进化**: 每次人类干预都作为训练信号反馈给系统，减少未来同类干预
4. **AI 开发 AI**: Kanata 自身的开发、测试、迭代也由 Kanata 自己驱动（dogfooding）
5. **Prompt as Code**: System Prompt、Skill 定义、Agent 行为规范全部版本化管理，可审查、可回滚

### 2.3 AI Native 自举 (Bootstrapping)

Kanata 的终极目标是实现**自举**——用 Kanata 开发 Kanata：

```
Phase 1: 人类用其他工具写出 Kanata MVP
Phase 2: 用 Kanata MVP 开发 Kanata v2 的部分功能
Phase 3: Kanata v2 承担自身 80% 的开发工作
Phase 4: Kanata 自主发现问题、提出方案、实现迭代，人类仅审批
```

---

## 3. 目标用户

| 角色 | 描述 |
|------|------|
| 个人开发者 | 独立使用，追求高效编码体验 |
| 团队 Leader | 管理团队 API Key、模型配置、成本预算 |
| 组织管理员 | 统一管理多团队、多项目的资源与权限 |
| DevOps 工程师 | 利用云端测试床进行 CI/CD 与部署 |
| 移动端开发者 | 在移动设备上进行代码审查和轻量编辑 |

---

## 4. 功能模块详细设计

---

### 4.1 模块一：核心 Agent 引擎 — 全面对标并超越 Claude Code

**目标**: 在代码理解、生成、重构、调试、测试等所有维度上达到或超越 Claude Code 水平。

#### 4.1.1 代码能力矩阵

| 能力 | Claude Code 基线 | Kanata 目标 |
|------|------------------|-------------|
| 代码库探索 | Glob/Grep/Read 工具链 | 同等工具链 + 语义代码图谱（Code Graph） |
| 代码生成 | 单文件/多文件编辑 | 同等能力 + 架构感知生成（Architecture-Aware Generation） |
| 代码重构 | 基于文本替换 | AST 级重构 + 跨文件引用安全重构 |
| Bug 修复 | 上下文推理 | 同等能力 + 运行时错误回溯分析 |
| 测试生成 | 基础测试生成 | 覆盖率驱动的测试生成 + 变异测试 |
| 代码审查 | PR Review | PR Review + 安全扫描 + 性能分析 + 架构合规检查 |
| 项目脚手架 | 基础 scaffolding | 模板市场 + 最佳实践脚手架 + 框架适配 |
| 文档生成 | 按需生成 | 自动文档同步 + API 文档 + 架构图生成 |
| 终端操作 | Bash 工具 | 同等能力 + 沙箱隔离执行 + 危险命令拦截 |
| 上下文管理 | 自动摘要 | 分层上下文管理 + 项目记忆 + 长期知识库 |

#### 4.1.2 Agent 编排架构（含 Skill Registry）

```
┌──────────────────────────────────────────────────────────┐
│                      Orchestrator                         │
│          (任务分解 / 子 Agent 调度 / 结果合成)              │
├──────────┬───────────┬────────────┬──────────────────────┤
│ Explorer │  Coder    │  Reviewer  │  DevOps Agent        │
│ Agent    │  Agent    │  Agent     │                      │
├──────────┼───────────┼────────────┼──────────────────────┤
│ Planner  │  Debugger │  Test      │  Doc Agent           │
│ Agent    │  Agent    │  Agent     │                      │
└──────────┴───────────┴────────────┴──────────────────────┘
      ↕            ↕            ↕            ↕
┌──────────┐ ┌──────────┐ ┌──────────┐ ┌──────────┐
│  Tool    │ │  Skill   │ │  Memory  │ │  Model   │
│  Registry│ │  Registry│ │  Store   │ │  Router  │
└──────────┘ └──────────┘ └──────────┘ └──────────┘
```

#### 4.1.3 Skill 注册与管理系统

Skill 是 Kanata 的一等公民——可组合、可发现、可热更新的能力单元。

**Skill 定义**:
```yaml
# skills/code-review.skill.yaml
apiVersion: kanata/v1
kind: Skill
metadata:
  name: code-review
  version: 1.2.0
  author: kanata-team
  tags: [review, quality, security]
  description: "AI-powered code review with security scanning"

spec:
  # 触发条件：何时自动激活
  triggers:
    - event: pr.created
    - event: pr.updated
    - command: /review

  # 输入 Schema
  input:
    type: object
    properties:
      files:
        type: array
        description: "Files to review"
      depth:
        type: string
        enum: [quick, standard, deep]
        default: standard

  # 依赖的其他 Skill
  dependencies:
    - skill: security-scan
      version: ">=1.0.0"
    - skill: style-check
      optional: true

  # 使用的工具
  tools:
    - Read
    - Grep
    - Glob
    - Bash

  # 模型偏好
  model:
    preferred: claude-sonnet-4
    minimum_capability: code-review

  # System Prompt (版本化)
  prompt: |
    You are a code reviewer. Analyze the provided code changes for:
    1. Correctness and logic errors
    2. Security vulnerabilities (OWASP Top 10)
    3. Performance implications
    4. Code style and best practices
```

**Skill Registry 功能**:

| 功能 | 说明 |
|------|------|
| 注册与发现 | 内置 Skill 市场，支持搜索、分类、评分 |
| 版本管理 | 语义化版本，支持多版本共存与灰度切换 |
| 依赖解析 | 自动解析 Skill 间依赖，拓扑排序执行 |
| 热更新 | Skill 更新无需重启，运行时生效 |
| 权限控制 | Skill 可声明所需权限（文件/网络/终端），用户可审批 |
| 组合编排 | 多个 Skill 可组合为 Workflow（DAG 编排） |
| 自定义 Skill | 用户/团队可编写私有 Skill，优先级高于内置 |
| AI 生成 Skill | 描述需求，AI 自动生成 Skill 定义 |
| 指标追踪 | 每个 Skill 的调用次数、成功率、Token 消耗、用户评分 |

**内置 Skill 分类**:

```
kanata-skills/
├── coding/              # 编码类
│   ├── code-complete    # 代码补全
│   ├── code-generate    # 代码生成
│   ├── code-refactor    # 重构
│   └── code-explain     # 代码解释
├── quality/             # 质量类
│   ├── code-review      # 代码审查
│   ├── security-scan    # 安全扫描
│   ├── test-generate    # 测试生成
│   └── perf-analyze     # 性能分析
├── devops/              # 运维类
│   ├── deploy           # 部署
│   ├── monitor          # 监控
│   └── incident         # 故障处理
├── doc/                 # 文档类
│   ├── doc-generate     # 文档生成
│   ├── api-doc          # API 文档
│   └── changelog        # 变更日志
└── workflow/            # 工作流类
    ├── pr-workflow       # PR 完整流程
    ├── release           # 发版流程
    └── onboarding        # 新人入职
```

**Skill Workflow 编排**:
```yaml
# workflows/pr-workflow.yaml
apiVersion: kanata/v1
kind: Workflow
metadata:
  name: pr-workflow
spec:
  steps:
    - skill: code-review
      id: review
    - skill: security-scan
      id: security
      parallel: true  # 与 review 并行
    - skill: test-generate
      id: test
      depends_on: [review]
      condition: "review.issues.count > 0"
    - skill: doc-generate
      id: doc
      depends_on: [review, security]
```

#### 4.1.4 核心超越点

1. **语义代码图谱 (Code Graph)**
   - 构建项目级别的代码语义图，包含函数调用关系、类继承、模块依赖
   - 支持增量更新，文件变更时局部刷新
   - 查询语言支持：按调用链、按影响范围、按依赖深度

2. **自适应规划引擎 (Adaptive Planning)**
   - 任务复杂度自动评估，动态决定是否进入 Plan Mode
   - 多方案生成与评估，附带 Trade-off 分析
   - 执行中动态调整计划，异常自动回退

3. **项目记忆系统 (Project Memory)**
   - 跨会话的项目知识持久化
   - 自动提取项目约定、架构决策、技术栈偏好
   - 团队共享记忆 vs 个人记忆分层

4. **安全沙箱 (Secure Sandbox)**
   - 所有代码执行、终端命令在隔离沙箱中运行
   - 文件系统权限白名单
   - 网络访问控制
   - 资源用量限制（CPU/内存/磁盘/时间）

---

### 4.2 模块二：多模型支持平台

**目标**: 全面支持北美与中国主流大模型，管理员可灵活配置模型策略。

#### 4.2.1 支持模型列表

| 区域 | 模型厂商 | 模型 |
|------|---------|------|
| 北美 | Anthropic | Claude Opus 4.5, Claude Sonnet 4, Claude Haiku 3.5 |
| 北美 | OpenAI | GPT-4o, GPT-o1, GPT-o3 |
| 北美 | Google | Gemini 2.0 Pro, Gemini 2.0 Flash |
| 北美 | Meta | Llama 4 (自部署) |
| 北美 | Mistral | Mistral Large, Codestral |
| 中国 | 百度 | 文心一言 4.0 |
| 中国 | 阿里 | 通义千问 Qwen-Max, Qwen-Coder |
| 中国 | 字节 | 豆包大模型 |
| 中国 | 深度求索 | DeepSeek-V3, DeepSeek-Coder |
| 中国 | 智谱 | GLM-4, CodeGeeX |
| 中国 | 月之暗面 | Kimi |
| 自部署 | 任意 | 兼容 OpenAI API 格式的任意模型 |

#### 4.2.2 模型路由策略

```yaml
model_routing:
  strategies:
    - name: "cost_optimized"
      description: "成本优先，小任务用轻量模型"
      rules:
        - task: ["code_completion", "simple_edit"]
          model: "haiku-3.5"
          fallback: "qwen-coder"
        - task: ["architecture_design", "complex_refactor"]
          model: "claude-opus-4.5"
          fallback: "gpt-o3"
        - task: ["code_review", "bug_fix"]
          model: "claude-sonnet-4"
          fallback: "deepseek-v3"

    - name: "quality_first"
      description: "质量优先，全部使用最强模型"
      rules:
        - task: ["*"]
          model: "claude-opus-4.5"
          fallback: "gpt-o3"

    - name: "china_compliant"
      description: "合规模式，仅使用中国大模型"
      rules:
        - task: ["*"]
          model: "deepseek-v3"
          fallback: "qwen-max"
```

#### 4.2.3 管理员模型管理功能

- **模型注册/下线**: 动态添加或禁用模型端点
- **路由策略配置**: 按任务类型、用户角色、团队配置不同模型
- **质量基准测试**: 对接入模型进行编码能力基准评测
- **模型健康监控**: 延迟、可用性、错误率实时监控
- **合规管控**: 按地域法规限制可用模型范围
- **A/B 测试**: 支持灰度切换模型并对比效果

---

### 4.3 模块三：全终端支持

**目标**: 覆盖 CLI、Web、移动端三大终端形态，支持多模态输入。

#### 4.3.1 终端矩阵

```
┌────────────────────────────────────────────────────────┐
│                    Kanata 统一后端                       │
│              (API Gateway + Agent Core)                 │
├──────────┬──────────────┬──────────────┬───────────────┤
│  CLI     │  Web App     │  Mobile App  │  IDE Plugin   │
│  终端    │  (React)     │  (RN/Flutter)│  (VS Code等)  │
├──────────┼──────────────┼──────────────┼───────────────┤
│ 完整开发 │ 可视化协作   │ 代码审查     │ 内嵌编辑器    │
│ 终端操作 │ 项目管理     │ 轻量编辑     │ 无缝编码      │
│ Git 操作 │ 实时协作     │ 语音交互     │ 上下文感知    │
│ 全键盘   │ 文件浏览     │ 图片输入     │ 行内补全      │
└──────────┴──────────────┴──────────────┴───────────────┘
```

#### 4.3.2 CLI 终端 (旗舰终端)

- **功能对等**: 与 Claude Code CLI 功能完全对等并超越
- **交互增强**:
  - 富文本终端渲染（语法高亮、diff 预览、进度条）
  - Vim/Emacs 快捷键支持
  - 可自定义快捷键绑定
- **会话管理**: 多会话并行、会话恢复、会话共享
- **管道集成**: 支持 stdin/stdout 管道，可嵌入脚本和 CI
- **离线模式**: 本地模型支持，无网络环境降级使用

#### 4.3.3 Web 终端

- **实时协作编辑器**: 类 VS Code 的 Web 编辑器
- **可视化项目仪表盘**: 项目状态、成本、团队活跃度一览
- **文件管理器**: 可视化浏览和管理项目文件
- **会话历史**: 全量会话记录与搜索
- **分享与嵌入**: 会话结果可生成分享链接

#### 4.3.4 移动终端

- **核心场景**: 代码审查、快速修复、项目状态监控
- **语音交互**:
  - 语音转文字输入编码指令
  - 支持语音描述 Bug，Agent 自动定位和修复
  - 多语言语音识别（中/英/日等）
- **图片输入**:
  - 拍照识别错误截图，自动分析和修复建议
  - UI 截图转代码
  - 手绘草图转 UI 组件
- **推送通知**: CI/CD 状态、PR Review 结果、部署状态推送
- **离线缓存**: 项目结构与最近会话离线可查

#### 4.3.5 IDE 插件

- **VS Code / JetBrains / Neovim** 插件
- 内嵌 Agent 面板，无需切换窗口
- 当前文件/选中代码上下文自动传递
- 行内代码建议与快速修复

#### 4.3.6 多模态输入处理管线

```
语音输入 ──→ ASR (Whisper/Paraformer) ──→ 意图识别 ──┐
图片输入 ──→ Vision Model 分析 ──────────→ 结构化描述 ──┼──→ Agent Core
文本输入 ──→ 直接传递 ─────────────────────────────────┘
```

---

### 4.4 模块四：API Key 池化管理与团队模式

**目标**: 支持组织级别的 API Key 统一采购、分配和用量管理。

#### 4.4.1 架构设计

```
┌─────────────────────────────────────────┐
│              组织 (Organization)          │
│  ┌─────────────────────────────────┐    │
│  │       API Key Pool              │    │
│  │  ┌─────┐ ┌─────┐ ┌─────┐      │    │
│  │  │Key1 │ │Key2 │ │Key3 │ ...  │    │
│  │  └─────┘ └─────┘ └─────┘      │    │
│  └──────────────┬──────────────────┘    │
│                 │                        │
│    ┌────────────┼────────────┐          │
│    ↓            ↓            ↓          │
│  ┌──────┐  ┌──────┐  ┌──────┐         │
│  │Team A│  │Team B│  │Team C│         │
│  │      │  │      │  │      │         │
│  │User1 │  │User4 │  │User7 │         │
│  │User2 │  │User5 │  │User8 │         │
│  │User3 │  │User6 │  │User9 │         │
│  └──────┘  └──────┘  └──────┘         │
└─────────────────────────────────────────┘
```

#### 4.4.2 Key 池管理

- **Key 注册与验证**: 批量导入 API Key，自动验证有效性和余额
- **智能调度**:
  - Round-Robin：轮询分配
  - Rate-Limit Aware：感知各 Key 的 RPM/TPM 限制，自动避让
  - Cost-Balance：均衡各 Key 消耗金额
  - Priority：VIP 用户优先使用高配额 Key
- **Key 健康检查**: 定期检测 Key 可用性，自动剔除失效 Key
- **Key 生命周期**: 支持到期提醒、自动轮换、临时停用

#### 4.4.3 配额与预算管理

```yaml
organization:
  name: "Acme Corp"
  monthly_budget: 50000  # USD
  alert_thresholds: [50%, 75%, 90%]

  teams:
    - name: "Backend Team"
      monthly_budget: 20000
      per_user_daily_limit: 200
      allowed_models: ["claude-opus-4.5", "deepseek-v3"]

    - name: "Frontend Team"
      monthly_budget: 15000
      per_user_daily_limit: 150
      allowed_models: ["claude-sonnet-4", "gpt-4o"]

    - name: "Intern Team"
      monthly_budget: 5000
      per_user_daily_limit: 50
      allowed_models: ["haiku-3.5", "qwen-coder"]
```

#### 4.4.4 权限体系 (RBAC)

| 角色 | 权限 |
|------|------|
| Org Admin | 全量管理：Key 池、团队、预算、模型、用户 |
| Team Admin | 团队内管理：成员、预算分配、模型选择 |
| Developer | 使用 Agent、查看个人用量 |
| Viewer | 只读：查看项目、会话历史，不可执行 Agent |

#### 4.4.5 SSO 与身份管理

- 支持 SAML 2.0 / OIDC / OAuth 2.0
- 对接企业 AD/LDAP
- 多因子认证 (MFA)
- 审计日志全量记录

---

### 4.5 模块五：业界追踪与自我迭代

**目标**: 建立系统化机制，持续跟踪业界领先 Code Agent，识别关键特性并快速迭代。

#### 4.5.1 竞品追踪矩阵

| 竞品 | 厂商 | 关注维度 |
|------|------|---------|
| Claude Code | Anthropic | Agent 架构、工具链、上下文管理 |
| GitHub Copilot Workspace | GitHub/Microsoft | 代码补全、IDE 集成、工作流 |
| Cursor | Cursor Inc. | 编辑器体验、代码生成质量 |
| Devin | Cognition | 自主编程、端到端任务完成 |
| Windsurf (Codeium) | Codeium | 代码搜索、多文件编辑 |
| Augment Code | Augment | 企业级代码理解 |
| Amazon Q Developer | AWS | 云服务集成、安全扫描 |
| Codex CLI | OpenAI | 命令行 Agent、工具调用 |

#### 4.5.2 特性雷达系统

```
┌──────────────────────────────────────────────┐
│             Feature Radar Pipeline            │
│                                              │
│  数据采集 ──→ 特性提取 ──→ 影响评估 ──→ 决策  │
│                                              │
│  Sources:                Scoring:            │
│  - GitHub Releases       - 用户价值 (1-10)    │
│  - 官方博客              - 技术可行性 (1-10)  │
│  - 论文 (arXiv)          - 竞争紧迫度 (1-10)  │
│  - 社区反馈              - 实现成本 (1-10)    │
│  - HN/Reddit/Twitter                        │
│  - 基准评测 (SWE-Bench)                      │
└──────────────────────────────────────────────┘
```

#### 4.5.3 自我迭代机制

**节奏: MVP 1周交付，此后每日迭代**

| 阶段 | 节奏 | 内容 |
|------|------|------|
| MVP | 第1周 | 核心 Agent + CLI + 单模型，可用即发布 |
| 日常迭代 | 每天 | 每日站会 → AI 辅助开发 → 自动测试 → 每日发布 |
| 竞品响应 | 48小时内 | 竞品重大特性发布后，48 小时内完成评估并启动开发 |
| 基准评测 | 每周 | SWE-Bench / HumanEval 自动评测，结果驱动优先级 |
| 架构演进 | 每月 | 月度架构 Review，基于数据决定重构方向 |

**每日迭代流水线**:
```
09:00  AI 扫描竞品动态 + 用户反馈 → 生成今日建议
09:30  团队站会，确认今日目标
10:00  开发（Kanata 辅助自身开发）
16:00  自动化测试 + AI Review
17:00  自动发布 Nightly Build
17:30  AI 生成当日迭代报告
```

#### 4.5.4 开源生态跟踪

- 自动监控 100+ 核心开源 Agent 项目的 Release
- 识别可复用的开源组件和技术方案
- 维护内部技术预研队列

---

### 4.6 模块六：Token 可视化跟踪与成本优化

**目标**: 让每一个 Token 的消耗透明可见，并通过智能优化大幅降低成本。

#### 4.6.1 Token 追踪仪表盘

```
┌───────────────────────────────────────────────────┐
│  Kanata Token Dashboard                           │
├───────────────────────────────────────────────────┤
│                                                   │
│  [今日消耗] 1.2M tokens  |  [今日成本] $18.50     │
│  [本月累计] 28.5M tokens |  [本月成本] $425.00    │
│  [预算剩余] $575.00      |  [预估用完] 2月15日    │
│                                                   │
│  ── Token 消耗趋势 (近7天) ──────────────────────  │
│  █████████████████████████████                    │
│  ████████████████████                             │
│  ██████████████████████████                       │
│  ████████████████████████████████                 │
│                                                   │
│  ── 按模型分布 ──────────────────────────────────  │
│  Claude Opus:   45% ████████████████              │
│  DeepSeek V3:   30% ██████████                    │
│  Haiku 3.5:     20% ███████                       │
│  其他:           5%  ██                            │
│                                                   │
│  ── 按用途分布 ──────────────────────────────────  │
│  代码生成:      35%                               │
│  代码理解:      25%                               │
│  代码审查:      20%                               │
│  测试生成:      10%                               │
│  其他:          10%                               │
│                                                   │
│  ── 按用户 TOP5 ─────────────────────────────────  │
│  1. Alice:  5.2M tokens ($82)                     │
│  2. Bob:    4.1M tokens ($65)                     │
│  3. Carol:  3.8M tokens ($58)                     │
│  ...                                              │
└───────────────────────────────────────────────────┘
```

#### 4.6.2 实时追踪维度

| 维度 | 说明 |
|------|------|
| 会话级 | 每次对话的 Input/Output Token 数、模型、延迟、成本 |
| 任务级 | 一个完整任务（含多轮对话）的总消耗 |
| 用户级 | 每个用户的日/周/月消耗与趋势 |
| 团队级 | 团队汇总统计、预算执行率 |
| 模型级 | 各模型的消耗占比、单价趋势 |
| 项目级 | 按项目维度统计，支持成本分摊 |

#### 4.6.3 成本优化引擎

1. **智能模型降级 (Smart Downgrade)** — 预估节省 30-50%
2. **上下文压缩 (Context Compression)** — 预估节省 20-40%
3. **缓存命中 (Semantic Cache)** — 预估节省 10-25%
4. **批量优化 (Batch Processing)** — 预估节省 50%
5. **预算告警与熔断** — 多级告警 + 超预算自动降级

---

### 4.7 模块七：云端测试床联动

**目标**: 构建最高性价比的云服务环境，支持一键构建、部署和测试。

#### 4.7.1 架构总览

```
┌──────────────────────────────────────────────────────┐
│                  Kanata Cloud Platform                │
│                                                      │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐    │
│  │ 开发环境    │  │ 测试环境    │  │ 预发布环境  │    │
│  │ (Dev)      │  │ (Test)     │  │ (Staging)  │    │
│  └────────────┘  └────────────┘  └────────────┘    │
│                                                      │
│  ┌──────────────────────────────────────────────┐   │
│  │           基础设施层                          │   │
│  │  Docker | K8s | Serverless | 数据库 | 缓存    │   │
│  └──────────────────────────────────────────────┘   │
│                                                      │
│  ┌──────────────────────────────────────────────┐   │
│  │           多云适配层                          │   │
│  │  AWS | GCP | Azure | 阿里云 | 腾讯云 | 华为云 │   │
│  └──────────────────────────────────────────────┘   │
└──────────────────────────────────────────────────────┘
```

#### 4.7.2 核心功能

1. **一键环境创建** — 30 秒内启动完整开发/测试环境
2. **智能资源调度** — Spot 实例优先，成本降低 60-80%
3. **测试自动化集成** — Agent 编码后自动在云端运行测试
4. **预览环境 (Preview Environments)** — 每个 PR 自动创建独立预览环境
5. **成本优化策略** — 自动休眠/销毁 + 多云价格比较

#### 4.7.3 安全与隔离

- 每个用户/团队独立命名空间
- 网络策略隔离
- Secrets 加密管理
- 审计日志全量记录

---

### 4.8 模块八：多人多团队协同开发

**目标**: 支持多人实时协作编码、代码审查、任务分配和知识共享。

#### 4.8.1 协同功能矩阵

| 功能 | 说明 |
|------|------|
| 实时协同编辑 | 多人同时编辑同一文件，类 Google Docs |
| 共享 Agent 会话 | 多人加入同一个 Agent 会话，协作解决问题 |
| 任务分配与追踪 | 将 Agent 生成的任务分配给团队成员 |
| 代码审查工作流 | 内建 PR Review 流程，Agent 辅助审查 |
| 知识库共享 | 团队项目约定、最佳实践、常见问题库 |
| 实时通信 | 内建 Chat，支持 @Agent 和 @成员 |
| 活动流 | 团队成员的 Agent 交互动态 Feed |

#### 4.8.2 协同架构

```
┌─────────────────────────────────────────┐
│           Collaboration Layer            │
├─────────┬──────────┬────────────────────┤
│  CRDT   │ Presence │  Notification      │
│  Engine │ System   │  System            │
│         │          │                    │
│ 文档协同 │ 在线状态 │  @提醒 / 推送通知   │
│ 冲突解决 │ 光标位置 │  邮件 / Webhook    │
│ 历史回溯 │ 活跃度   │  Slack/飞书集成    │
└─────────┴──────────┴────────────────────┘
```

#### 4.8.3 共享 Agent 会话

- **会话模式**: Solo / Pair / Team
- **权限控制**: 会话创建者可控制参与者的权限级别
- **会话回放**: 完整记录会话过程，支持回放和标注

#### 4.8.4 代码审查增强

1. Agent 自动生成 PR Summary 和 Review Comments
2. 自动检测安全漏洞、性能问题、代码风格违规
3. 审查者可直接在 Review 界面调用 Agent 解释代码
4. 审查建议一键应用，Agent 自动修改代码

#### 4.8.5 团队知识管理

- **项目 Wiki**: Agent 自动维护项目文档
- **决策记录 (ADR)**: 架构决策自动归档
- **问答库**: 团队高频问题自动沉淀
- **Onboarding Agent**: 新成员入职时自动介绍项目结构和规范

---

### 4.9 模块九：最强大脑 — 增强能力

**目标**: 引入前沿技术，构建 Kanata 独有的差异化优势。

#### 4.9.1 主动式 Agent (Proactive Agent)

- **代码健康巡检**: 后台定期扫描代码库，主动发现潜在 Bug、安全漏洞、性能瓶颈
- **依赖更新提醒**: 监控依赖包更新和安全公告，主动建议升级
- **技术债务检测**: 自动识别和量化技术债务，生成偿还计划
- **PR 自动跟进**: 长时间未合并的 PR 自动提醒和更新

#### 4.9.2 多 Agent 协同推理 (Multi-Agent Reasoning)

```
        ┌─────────────┐
        │  Architect   │ ← 负责架构决策
        │  Agent       │
        └──────┬──────┘
               │
    ┌──────────┼──────────┐
    ↓          ↓          ↓
┌────────┐ ┌────────┐ ┌────────┐
│Frontend│ │Backend │ │DevOps  │ ← 各领域专家 Agent
│ Agent  │ │ Agent  │ │ Agent  │
└────────┘ └────────┘ └────────┘
    ↓          ↓          ↓
┌────────┐ ┌────────┐ ┌────────┐
│Reviewer│ │Security│ │  QA    │ ← 质量保障 Agent
│ Agent  │ │ Agent  │ │ Agent  │
└────────┘ └────────┘ └────────┘
```

#### 4.9.3 学习与进化系统

- **RLHF 反馈回路**: 用户采纳/拒绝反馈
- **项目适应性学习**: Agent 越用越了解项目
- **团队风格学习**: 自动学习编码风格、命名规范
- **错误记忆**: 记录犯过的错误，避免重复

#### 4.9.4 代码时间旅行 / 智能调试 / 安全守卫 / 可观测性

- 可视化项目演进历史 + Git 语义化理解
- 运行时错误自动诊断 + 性能 Profiling 集成
- 实时 OWASP Top 10 扫描 + CVE 监控 + 密钥泄露防护
- Agent 全链路追踪 (OpenTelemetry) + 异常检测 + 体验指标

---

## 5. 技术架构

### 5.1 系统架构总览

```
┌─────────────────────────────────────────────────────────────┐
│                        Client Layer                         │
│   CLI  │  Web App  │  Mobile App  │  IDE Plugin  │  API    │
└────────┴───────────┴──────────────┴──────────────┴─────────┘
                            │
                     ┌──────┴──────┐
                     │ API Gateway │  (认证/限流/路由)
                     └──────┬──────┘
                            │
┌───────────────────────────┴────────────────────────────────┐
│                     Service Layer                          │
│                                                            │
│  ┌──────────────┐  ┌──────────────┐  ┌────────────────┐  │
│  │ Agent Core   │  │ Skill        │  │ Cloud Platform │  │
│  │ Service      │  │ Registry     │  │ Service        │  │
│  └──────┬───────┘  └──────────────┘  └────────────────┘  │
│         │                                                  │
│  ┌──────┴───────┐  ┌──────────────┐  ┌────────────────┐  │
│  │ Model Router │  │ Key Manager  │  │ Token Tracker  │  │
│  │ Service      │  │ Service      │  │ Service        │  │
│  └──────────────┘  └──────────────┘  └────────────────┘  │
│                                                            │
│  ┌──────────────┐  ┌──────────────┐  ┌────────────────┐  │
│  │ User/Team    │  │ Collaboration│  │ Analytics      │  │
│  │ Service      │  │ Service      │  │ Service        │  │
│  └──────────────┘  └──────────────┘  └────────────────┘  │
└────────────────────────────────────────────────────────────┘
                            │
┌───────────────────────────┴────────────────────────────────┐
│                     Data Layer                             │
│                                                            │
│  PostgreSQL │ Redis │ S3/MinIO │ Qdrant │ ClickHouse      │
│  (业务数据)  │(缓存) │ (文件)   │(向量库) │ (分析数据)      │
└────────────────────────────────────────────────────────────┘
```

### 5.2 技术选型

| 层级 | 技术 | 说明 |
|------|------|------|
| CLI | Rust + Ratatui | 高性能、跨平台、富终端 UI |
| Web 前端 | React + TypeScript + Monaco Editor | 成熟生态、VS Code 同款编辑器 |
| 移动端 | React Native | 跨平台、共享 Web 组件逻辑 |
| API 网关 | Kong / Envoy | 高性能、插件化 |
| 后端核心 | Rust (Agent Core) + Go (微服务) | 性能与开发效率平衡 |
| 实时通信 | WebSocket + CRDT (Yjs) | 低延迟、冲突自动解决 |
| 数据库 | PostgreSQL + Redis + ClickHouse | OLTP + 缓存 + OLAP |
| 向量存储 | Qdrant | 语义缓存、代码搜索 |
| 对象存储 | S3 / MinIO | 文件、快照 |
| 消息队列 | NATS | 轻量高性能 |
| 容器编排 | Kubernetes | 云端环境调度 |
| 可观测性 | OpenTelemetry + Grafana | 全链路追踪 |

---

## 6. MVP 定义与 1 周交付计划

### 6.1 MVP 范围：可运行的 AI Code Agent

**MVP 目标**: 1 周内交付一个可通过 CLI 进行交互式编码的 Agent，核心体验对标 Claude Code 基础功能。

**MVP 包含**:

| 功能 | 优先级 | 说明 |
|------|--------|------|
| CLI 交互终端 | P0 | 终端输入/输出、会话管理、Markdown 渲染 |
| Agent 对话循环 | P0 | 用户指令 → 模型推理 → 工具调用 → 结果返回 |
| 文件操作工具 | P0 | Read / Write / Edit / Glob / Grep |
| Bash 执行工具 | P0 | 命令执行 + 基础安全拦截 |
| 单模型对接 | P0 | 对接 1 个模型 API（Claude 或 DeepSeek） |
| Skill 基础框架 | P0 | Skill 加载 + 注册 + 调用（内置 3 个核心 Skill） |
| Token 基础统计 | P1 | 每次会话显示 Token 消耗和成本 |
| 配置文件 | P1 | API Key 配置、模型选择、基础偏好 |

**MVP 不包含** (Phase 2+):
- Web/移动端、多模型路由、Key 池化、团队协作、云端测试床

### 6.2 MVP 架构（最小可行）

```
┌──────────────┐     ┌──────────────┐     ┌──────────────┐
│   CLI TUI    │ ──→ │  Agent Core  │ ──→ │  Model API   │
│  (用户交互)   │ ←── │  (编排循环)   │ ←── │  (LLM 调用)  │
└──────────────┘     └──────┬───────┘     └──────────────┘
                            │
                    ┌───────┴────────┐
                    │                │
              ┌─────┴─────┐   ┌─────┴─────┐
              │   Tool    │   │   Skill   │
              │  Registry │   │  Registry │
              │           │   │           │
              │ - Read    │   │ - commit  │
              │ - Write   │   │ - review  │
              │ - Edit    │   │ - explain │
              │ - Glob    │   │           │
              │ - Grep    │   │           │
              │ - Bash    │   │           │
              └───────────┘   └───────────┘
```

---

## 7. 团队分工 (3~5 人)

### 7.1 分工原则

1. **模块边界清晰**: 每人负责独立模块，通过明确的接口（Trait/Interface）交互
2. **零阻塞**: 模块间依赖通过 Mock/Stub 解耦，任何人不需要等另一个人完成才能开发
3. **接口先行**: Day 1 先定义所有模块间接口（Rust trait / Go interface），然后各自实现
4. **每日集成**: 每天下班前合并一次主干，保持集成频率

### 7.2 角色定义与分工

```
┌─────────────────────────────────────────────────────────────┐
│                     MVP 模块依赖图                           │
│                                                             │
│                    ┌──────────┐                             │
│                    │  P1:CLI  │                             │
│                    │  终端层   │                             │
│                    └────┬─────┘                             │
│                         │ 调用                              │
│                    ┌────┴─────┐                             │
│                    │ P2:Agent │                             │
│                    │ 编排核心  │                             │
│                    └──┬────┬──┘                             │
│                 调用 │    │ 调用                             │
│            ┌────────┘    └────────┐                         │
│       ┌────┴─────┐          ┌────┴─────┐                   │
│       │ P3:Tool  │          │ P4:Model │                   │
│       │ & Skill  │          │ & Token  │                   │
│       └──────────┘          └──────────┘                   │
│                                                             │
│                    ┌──────────┐                             │
│                    │ P5:Infra │  (可选第5人，否则分摊)        │
│                    │ CI/测试   │                             │
│                    └──────────┘                             │
└─────────────────────────────────────────────────────────────┘
```

#### P1: CLI 终端层 — 1 人

**职责**: 用户看到和交互的一切

| 交付物 | 说明 |
|--------|------|
| 终端 UI | 输入框、输出渲染、Markdown/代码高亮、diff 预览 |
| 会话管理 | 多会话创建/切换/恢复/持久化 |
| 命令系统 | /help, /model, /cost, /clear 等斜杠命令 |
| 配置管理 | ~/.kanata/config.yaml 读写、首次引导 |
| 流式输出 | 模型 SSE 流式响应的实时渲染 |

**接口依赖**: 调用 Agent Core 的 `AgentSession` trait
**可独立开发**: 用 Mock AgentSession 跑通全部 UI 逻辑

#### P2: Agent 编排核心 — 1 人

**职责**: 大脑——任务理解、工具调度、上下文管理

| 交付物 | 说明 |
|--------|------|
| 对话循环 | User → LLM → Tool Call → Result → LLM → Response |
| 上下文管理 | 消息历史、Token 窗口管理、自动摘要 |
| 工具调度 | 解析 LLM 的 tool_use，分发到 Tool Registry |
| Skill 调度 | 解析 /command，加载并执行对应 Skill |
| 错误处理 | 工具执行失败、模型超时、上下文溢出的优雅处理 |
| System Prompt | 核心 System Prompt 编写和管理 |

**接口依赖**: 调用 Tool Registry 的 `Tool` trait，调用 Model Client 的 `LLMClient` trait
**可独立开发**: 用 Mock LLMClient (返回预设响应) + Mock Tool (返回预设结果)

#### P3: 工具层 & Skill 框架 — 1 人

**职责**: Agent 的手——所有与文件系统、终端、外部世界交互的能力

| 交付物 | 说明 |
|--------|------|
| 文件工具 | Read / Write / Edit / Glob / Grep 实现 |
| Bash 工具 | 命令执行、超时控制、输出截断、危险命令拦截 |
| Tool Registry | 工具注册、发现、Schema 生成（给 LLM 的 tool 定义） |
| Skill Registry | Skill YAML 加载、注册、版本管理、依赖解析 |
| 内置 Skill | commit / code-review / explain 三个种子 Skill |
| 安全层 | 文件路径白名单、命令黑名单 |

**接口依赖**: 实现 `Tool` trait，被 Agent Core 调用
**可独立开发**: 每个工具独立单元测试，不需要 Agent 也能验证正确性

#### P4: 模型对接 & Token 追踪 — 1 人

**职责**: 模型通信 + 成本可见性

| 交付物 | 说明 |
|--------|------|
| LLM Client | 统一的 LLM 调用抽象层 (trait LLMClient) |
| 模型适配器 | Anthropic API 适配器（MVP 主力）+ OpenAI 兼容适配器 |
| 流式处理 | SSE 流式响应解析、tool_use 流式解析 |
| Token 计数 | 请求/响应 Token 统计、成本计算 |
| Token 报告 | 会话结束时输出消耗汇总、CLI 显示实时 Token 数 |
| Key 配置 | API Key 读取、验证、多 Key 轮询（基础版） |
| 重试逻辑 | 速率限制处理、指数退避、自动 Fallback |

**接口依赖**: 实现 `LLMClient` trait，被 Agent Core 调用
**可独立开发**: 直接对模型 API 写集成测试，不需要 Agent

#### P5: 基础设施 & 质量 — 1 人（可选，或由团队分摊）

**职责**: 让团队每天能顺畅集成和发布

| 交付物 | 说明 |
|--------|------|
| 项目脚手架 | Cargo workspace 结构、模块划分 |
| CI/CD | GitHub Actions：lint / test / build / release |
| 集成测试 | 端到端测试框架，模拟完整对话流程 |
| 发布流水线 | 自动构建多平台二进制、Nightly 发布 |
| 文档 | 接口文档、开发者指南、CONTRIBUTING.md |

### 7.3 接口契约 (Day 1 定义)

```rust
// ═══════ 模块间核心接口 ═══════

/// P4 实现，P2 调用
#[async_trait]
pub trait LLMClient: Send + Sync {
    /// 发送消息，返回流式响应
    async fn chat_stream(
        &self,
        messages: &[Message],
        tools: &[ToolDefinition],
    ) -> Result<Pin<Box<dyn Stream<Item = StreamEvent>>>>;

    /// 获取本次调用的 Token 用量
    fn last_usage(&self) -> TokenUsage;
}

/// P3 实现，P2 调用
#[async_trait]
pub trait Tool: Send + Sync {
    fn name(&self) -> &str;
    fn description(&self) -> &str;
    fn input_schema(&self) -> serde_json::Value;
    async fn execute(&self, input: serde_json::Value) -> Result<ToolResult>;
}

/// P3 实现，P2 调用
pub trait SkillRegistry: Send + Sync {
    fn register(&mut self, skill: SkillDefinition) -> Result<()>;
    fn get(&self, name: &str) -> Option<&SkillDefinition>;
    fn list(&self) -> Vec<&SkillDefinition>;
    fn resolve_workflow(&self, name: &str) -> Result<Vec<SkillStep>>;
}

/// P2 实现，P1 调用
#[async_trait]
pub trait AgentSession: Send + Sync {
    /// 发送用户消息，返回流式 Agent 响应
    async fn send_message(
        &self,
        content: UserContent,
    ) -> Result<Pin<Box<dyn Stream<Item = AgentEvent>>>>;

    /// 获取当前会话的 Token 统计
    fn token_stats(&self) -> SessionTokenStats;

    /// 执行斜杠命令
    async fn execute_command(&self, command: &str) -> Result<CommandResult>;
}
```

### 7.4 1 周 MVP 日程

| 日 | P1: CLI | P2: Agent Core | P3: Tool & Skill | P4: Model & Token |
|----|---------|---------------|-------------------|-------------------|
| **D1** | 项目结构搭建 + 接口定义 (全员协作) ||||
| **D2** | 基础 TUI 框架 + 输入输出 | 对话循环骨架 + Mock | Read/Write/Edit 实现 | Anthropic Client + 流式 |
| **D3** | Markdown 渲染 + 代码高亮 | Tool 调度 + 上下文管理 | Glob/Grep/Bash 实现 | Token 统计 + 重试逻辑 |
| **D4** | 会话管理 + 命令系统 | Skill 调度 + 错误处理 | Tool Registry + Skill 框架 | OpenAI 兼容适配器 |
| **D5** | 流式输出 + 配置管理 | System Prompt 调优 | 3 个内置 Skill | Token 报告 + Key 轮询 |
| **D6** | 集成联调 (全员协作) ||||
| **D7** | 端到端测试 + Bug Fix + 打包发布 (全员协作) ||||

### 7.5 3 人精简方案

如果只有 3 人，合并方案：

| 人员 | 职责 | 合并说明 |
|------|------|---------|
| **A** | CLI + 配置 | P1 全部 |
| **B** | Agent Core + Tool & Skill | P2 + P3，核心引擎一人贯穿 |
| **C** | Model + Token + CI | P4 + P5，对外通信和工程基础 |

---

## 8. 开发里程碑

### Phase 1: MVP (第 1 周)
- Agent 核心引擎 + CLI + 单模型 + Skill 框架 + Token 基础统计
- 交付标准：能跑通"读文件 → 理解 → 修改 → 写回"完整流程

### Phase 2: 多模型与管理 (第 2~3 周，每日迭代)
- 多模型路由 + API Key 池化 + 基础 Web 管理面板
- 成本优化引擎 V1

### Phase 3: 协同与终端 (第 4~6 周)
- Web 终端 + 团队管理 + 共享会话
- Skill 市场 V1

### Phase 4: 云端与增强 (第 7~10 周)
- 云端测试床 + 移动端 + IDE 插件
- 多 Agent 协同 + 主动式 Agent

### Phase 5: 自举与生态 (持续)
- 用 Kanata 开发 Kanata
- 插件市场 + 开源社区 + 企业级服务

---

## 9. 成功指标 (KPIs)

| 指标 | 目标 |
|------|------|
| MVP 交付 | 第 1 周结束可用 |
| 迭代频率 | MVP 后每日至少 1 个有效发布 |
| SWE-Bench 评测 | 超过 Claude Code 基准 5%+ |
| 任务完成率 | ≥ 85% |
| 平均响应延迟 | < 3 秒 (首 Token) |
| 成本节省 (vs 纯 Opus) | ≥ 40% |
| 用户日活留存 | ≥ 60% |
| 系统可用性 | ≥ 99.9% |

---

## 10. 风险与缓解

| 风险 | 影响 | 缓解措施 |
|------|------|---------|
| 1 周 MVP 延期 | 错过窗口期 | 严格砍需求，P0 only；D6 强制联调 |
| 模块集成困难 | 联调耗时 | Day 1 定接口 + 每日集成 + 充分 Mock |
| 模型厂商 API 变更 | 功能中断 | 统一抽象层 + 多模型冗余 |
| 中国合规要求变化 | 无法服务中国用户 | 独立部署方案 + 合规顾问 |
| Token 成本失控 | 用户流失 | 多级成本优化 + 预算熔断 |
| 竞品快速迭代 | 竞争优势缩小 | Feature Radar + 每日迭代响应 |

---

*文档维护者: Kanata Team*
*最后更新: 2026-01-31 v1.1*
