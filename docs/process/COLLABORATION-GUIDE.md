# Kanata 团队协同指南

**版本**: v1.0
**日期**: 2026-01-31
**适用**: Baoxing Huai (A) / Junjie Duan (B) / Sihao Li (C) / Jianmin Lu (Sponsor)

---

## 一、协同核心原则

```
┌─────────────────────────────────────────────────────────────┐
│                                                             │
│   原则 1: 接口即契约 — 三人靠 trait 连接，不靠口头约定        │
│   原则 2: Mock 即自由 — 有 Mock 就能独立跑，不等任何人        │
│   原则 3: main 即真相 — 每天合并 main，谁都不在分支里藏代码   │
│   原则 4: CI 即裁判  — 格式/lint/测试 不过就不合并，无例外    │
│   原则 5: 15 分钟规则 — 卡住 15 分钟就喊人，不要闷头死磕     │
│                                                             │
└─────────────────────────────────────────────────────────────┘
```

---

## 二、三人协作拓扑

### 2.1 代码依赖关系

```
                  kanata-types (共有契约层)
                 /       |        \
                /        |         \
    kanata-cli      kanata-core      kanata-model
      Dev A            Dev B            Dev C
        \             / \              /
         \           /   \            /
        kanata-tools   kanata-skills
              Dev B         Dev B
```

**关键点**: 三人的代码**只在 `kanata-types` 处交汇**。除此之外，A 不 import B 的 crate，C 不 import B 的 crate。`kanata-core` 通过 trait object 调用 tools 和 model，编译时不依赖具体实现。

### 2.2 谁给谁提供什么

```
┌─────────────────────────────────────────────────────────┐
│                                                         │
│   Dev C ───提供 MockLLMClient──→ Dev B                  │
│          (D1 下午交付)                                   │
│          预设返回: 纯文本回复 / tool_use 回复 / 错误回复   │
│                                                         │
│   Dev B ───提供 MockAgentSession──→ Dev A               │
│          (D1 下午交付)                                   │
│          预设返回: TextDelta 流 / ToolStart+End 流       │
│                                                         │
│   Dev A ───提供 CLI 需求反馈──→ Dev B                    │
│          (AgentEvent 够不够用? 缺什么字段?)               │
│                                                         │
│   Dev B ───提供 ToolDefinition 列表──→ Dev C            │
│          (工具的 JSON Schema, 用于 LLM 的 tools 参数)    │
│                                                         │
└─────────────────────────────────────────────────────────┘
```

### 2.3 接口变更协议

`kanata-types` 在 D1 锁定后，**任何字段变更都需要三人同意**。

```
变更流程:

1. 发起人在 IM 群发消息:
   "[接口变更] AgentEvent 需要增加 Confidence(f32) 变体，
    原因: 置信度展示需要。影响范围: A 的渲染 + B 的事件生成"

2. 三人在 15 分钟内回复 ✅ 或提出替代方案

3. 全部 ✅ 后，发起人修改 kanata-types 并提 PR

4. 三人各自在自己的 crate 中适配变更

5. 同一天内合并到 main
```

**分级**:

| 变更类型 | 示例 | 流程 |
|---------|------|------|
| 新增字段/变体 (向后兼容) | AgentEvent 加一个新变体 | IM 通知 → 三人 ✅ → 当天合并 |
| 修改已有字段 (不兼容) | 改 TokenUsage 的字段类型 | 必须站会讨论 → 同步修改 → 同天合并 |
| 新增 trait 方法 | LLMClient 加新方法 | 站会讨论 + 提供 default 实现 → 不阻塞其他人 |

---

## 三、每日协同节奏

### 3.1 一天的时间线

```
09:00  ┌─ 站会 (15 min, 全员 + Sponsor) ────────────────┐
       │                                                 │
       │  每人三句话:                                     │
       │    1. 昨天完成了什么 (对应哪个任务)               │
       │    2. 今天计划做什么                              │
       │    3. 有没有阻塞 (卡住/需要别人配合)              │
       │                                                 │
       │  Sponsor:                                       │
       │    - 有没有需求/优先级变更                        │
       │    - 解除阻塞决策                                │
       └─────────────────────────────────────────────────┘

09:15  ┌─ 独立开发时段 ─────────────────────────────────┐
~ 12:00│                                                 │
       │  各自在自己的 crate 中编码                       │
       │  用 Mock 跑自己的测试                            │
       │  有问题随时 IM 问，不攒到站会                     │
       └─────────────────────────────────────────────────┘

12:00  午休

13:00  ┌─ 独立开发时段 ─────────────────────────────────┐
~ 16:00│                                                 │
       │  继续开发                                       │
       │  完成的功能提 PR                                 │
       └─────────────────────────────────────────────────┘

16:00  ┌─ 集成窗口 (30 min) ────────────────────────────┐
       │                                                 │
       │  1. 各自 push 最新代码到自己的分支               │
       │  2. 提 PR 到 main                               │
       │  3. Reviewer 审查 (见轮转表)                     │
       │  4. CI 通过 → 合并                              │
       │  5. 全员 git pull main → 验证 cargo build       │
       │                                                 │
       │  如果有冲突: 相关两人 screen share 一起解决       │
       └─────────────────────────────────────────────────┘

16:30  ┌─ Review 时段 (30 min) ─────────────────────────┐
       │                                                 │
       │  审查别人的 PR                                   │
       │  提 comment 或 approve                          │
       └─────────────────────────────────────────────────┘

17:00  收工 (或继续开发明天的内容)
```

### 3.2 站会模板

```
# Daily Standup - 2026-02-XX

## Dev A (Baoxin)
- 昨天: 完成 CLI 输入组件，支持多行 + 历史
- 今天: 做 Markdown 渲染 + 代码高亮
- 阻塞: 无

## Dev B (Junjie)
- 昨天: Agent 循环基本跑通，Read/Write 工具完成
- 今天: 做 tool_use 调度 + Edit/Glob/Grep
- 阻塞: 需要确认 ToolResult 是否需要加 metadata 字段

## Dev C (Sihao)
- 昨天: Anthropic SSE 解析完成，纯文本流正常
- 今天: 做 tool_use 类型的流式解析
- 阻塞: 无

## Sponsor (Jianmin)
- 决策: ToolResult 暂不加 metadata，MVP 后再说
- 提醒: D3 结束时 Dev B 应该能用 Mock 跑通完整 Agent Loop
```

---

## 四、代码协同规范

### 4.1 分支策略

```
main (保护分支)
  │
  ├── dev/a-cli-input          Dev A: CLI 输入组件
  ├── dev/a-cli-render         Dev A: 输出渲染
  ├── dev/a-cli-session        Dev A: 会话管理
  │
  ├── dev/b-agent-loop         Dev B: Agent 主循环
  ├── dev/b-tools-rw           Dev B: Read/Write 工具
  ├── dev/b-tools-search       Dev B: Glob/Grep 工具
  │
  ├── dev/c-anthropic          Dev C: Anthropic 适配器
  ├── dev/c-stream-parser      Dev C: SSE 解析
  ├── dev/c-token-tracker      Dev C: Token 统计
  │
  └── fix/42-sse-parsing       Bug 修复分支
```

**规则**:
- 每个功能点一个分支，**不在一个分支里堆多个功能**
- 分支生命周期 ≤ 2 天，超过说明粒度太粗，需要拆分
- 合并后立即删除分支

### 4.2 PR 流程

```
开发者提 PR
    │
    ↓
CI 自动运行 ──→ 失败 → 开发者修复 → 重新推送
    │
    ↓ 通过
Reviewer 审查 ──→ 有 comment → 开发者修改 → 重新审查
    │
    ↓ Approved
合并到 main (Squash Merge)
    │
    ↓
其他人 git pull main，确认 cargo build 通过
```

**Review 轮转表**:

| 提交者 | Reviewer | 备选 |
|--------|---------|------|
| Dev A (Baoxin) | Dev B (Junjie) | Dev C |
| Dev B (Junjie) | Dev C (Sihao) | Dev A |
| Dev C (Sihao) | Dev A (Baoxin) | Dev B |

**Review 时效**: PR 提交后 **2 小时内**必须完成 Review。如果 Reviewer 忙，备选 Reviewer 接手。

**Review 关注点** (按优先级):

| 优先级 | 关注什么 | 示例 |
|--------|---------|------|
| P0 | 接口契约是否正确使用 | trait 方法的调用方式、参数传递 |
| P0 | 安全问题 | unwrap、路径注入、密钥泄露 |
| P1 | 错误处理是否完整 | 缺少 error case、错误信息不清 |
| P1 | 测试是否覆盖关键路径 | 缺少异常路径测试 |
| P2 | 代码可读性 | 命名不清、函数过长 |
| P3 | 性能 | MVP 阶段不纠结性能，除非明显问题 |

**Review 原则**:
- **不阻塞**: Review comment 分为 `must fix` 和 `nit` (建议)，只有 `must fix` 阻塞合并
- **不吹毛求疵**: MVP 阶段代码能工作 > 代码完美
- **给方案不只给问题**: 指出问题时附带建议的修改方式

### 4.3 冲突预防与解决

**为什么冲突极少发生**:

```
三人的代码文件完全不重叠:

Dev A 只碰:  crates/kanata-cli/src/**
Dev B 只碰:  crates/kanata-core/src/**
             crates/kanata-tools/src/**
             crates/kanata-skills/src/**
             skills/**
             prompts/**
Dev C 只碰:  crates/kanata-model/src/**
             .github/workflows/**

唯一可能冲突:  crates/kanata-types/src/**  (D1 锁定后极少改)
```

**万一冲突了**:

| 场景 | 处理方式 |
|------|---------|
| `kanata-types` 冲突 | 涉及的两人 screen share 一起解决，第三人在 IM 确认 |
| `Cargo.toml` 依赖冲突 | 谁先合并谁的版本为准，后合并的人 rebase 适配 |
| 同一文件不同区域 | Git 自动合并，无需干预 |

### 4.4 代码所有权

```
┌──────────────────────────────────────────────────────────┐
│  CODEOWNERS (.github/CODEOWNERS)                         │
│                                                          │
│  # 共有层 — 任何修改需要全员同意                           │
│  crates/kanata-types/    @baoxin @junjie @sihao          │
│                                                          │
│  # 各自负责的 crate — 本人为 owner                        │
│  crates/kanata-cli/      @baoxin                         │
│  crates/kanata-core/     @junjie                         │
│  crates/kanata-tools/    @junjie                         │
│  crates/kanata-skills/   @junjie                         │
│  crates/kanata-model/    @sihao                          │
│                                                          │
│  # 基础设施                                               │
│  .github/                @sihao                          │
│  Cargo.toml              @baoxin @junjie @sihao          │
│                                                          │
│  # 文档                                                   │
│  docs/                   @jianmin                        │
│  prompts/                @junjie @jianmin                │
│  skills/                 @junjie                         │
└──────────────────────────────────────────────────────────┘
```

**含义**: 修改某个目录的文件时，对应 owner 会被自动加为 Reviewer。修改 `kanata-types` 时三人都会被通知。

---

## 五、沟通协议

### 5.1 沟通渠道与用途

| 渠道 | 用途 | 响应时效 | 示例 |
|------|------|---------|------|
| **IM 群 (飞书/微信)** | 日常问题、快速确认、接口讨论 | **15 分钟** | "B: `ToolResult` 需要加个 `truncated: bool` 吗？" |
| **IM 私聊** | 一对一技术讨论 | 30 分钟 | "A→B: 你的 `AgentEvent::ToolStart` 里 `input_preview` 是完整 JSON 还是截断的？" |
| **GitHub PR Comment** | 代码级讨论 | 2 小时 | Review 意见和回复 |
| **GitHub Issue** | Bug 记录、功能需求 | 下次站会 | "#15: SSE 解析器遇到空行会 panic" |
| **站会** | 进度同步、阻塞解除、决策 | 每日 09:00 | 见站会模板 |
| **临时 Huddle** | 复杂问题、冲突解决、设计讨论 | 即时 | "B+C: tool_use 的 JSON 分片逻辑我们对个屏" |

### 5.2 求助协议 (15 分钟规则)

```
┌────────────────────────────────────────────────────────┐
│                                                        │
│  遇到问题                                               │
│    │                                                    │
│    ↓                                                    │
│  自己查资料/调试 (15 分钟)                               │
│    │                                                    │
│    ├── 解决了 → 继续开发                                 │
│    │                                                    │
│    └── 没解决 → IM 群发消息求助                           │
│          │                                              │
│          │  格式:                                        │
│          │  "[求助] 问题描述"                             │
│          │  "我试过: xxx"                                │
│          │  "报错信息: xxx"                               │
│          │  "相关代码: 文件名:行号"                       │
│          │                                              │
│          ↓                                              │
│        有人知道 → IM 回复或 screen share                  │
│        没人知道 → 记录为 Issue，临时绕过，站会讨论         │
│                                                        │
└────────────────────────────────────────────────────────┘
```

**禁止**:
- 闷头卡一上午不说话
- 在 IM 里发一句"不行"不说明原因和上下文
- 修改别人 crate 的代码不通知 owner

### 5.3 决策记录

影响架构或接口的决策需要记录：

```markdown
<!-- docs/decisions/001-sse-parsing.md -->

# ADR-001: SSE 解析策略

## 状态: Accepted
## 日期: 2026-02-02
## 参与人: B, C

## 背景
Anthropic 的 tool_use 响应中，JSON input 是分片发送的，
需要在客户端拼接完整 JSON。

## 方案 A: 边收边解析 (streaming JSON parser)
- 优点: 延迟低
- 缺点: 实现复杂，jq-like streaming parser 不成熟

## 方案 B: 缓冲后整体解析
- 优点: 实现简单，用标准 serde_json
- 缺点: tool_use 的 input 会有微小延迟

## 决策: 方案 B
MVP 阶段优先简单，tool_use input 体积小 (<10KB)，缓冲延迟可忽略。

## 影响
- Dev C 实现 stream.rs 时，ToolUseDelta 事件缓冲到内部 String
- Dev B 收到 ToolUseEnd 时一次性 parse JSON
```

---

## 六、联调协议

### 6.1 联调前置条件

```
联调 Checklist (D6 上午之前每人自检):

Dev A:
  □ MockAgentSession 所有 event 类型都能正确渲染
  □ 流式输出无闪烁、无乱码
  □ /help /cost /clear 命令工作正常
  □ 配置文件读写正确

Dev B:
  □ MockLLMClient 返回纯文本 → Agent 正确输出
  □ MockLLMClient 返回 tool_use → Agent 调用工具 → 结果回传
  □ 多轮对话上下文正确维护
  □ 每个工具 (Read/Write/Edit/Glob/Grep/Bash) 单元测试通过

Dev C:
  □ 对真实 Claude API 发送请求 → 收到流式响应 → 正确解析
  □ tool_use 类型响应能正确解析 (含 JSON 分片拼接)
  □ Token 计数正确 (与 API 返回的 usage 一致)
  □ 429 重试逻辑工作正常
```

### 6.2 联调步骤

```
D6 上午联调流程:

Step 1: 组装 (30 min)
  ┌──────────────────────────────────────────┐
  │  全员 git pull main                      │
  │  cargo build --workspace --release       │
  │  确认编译通过 (如果不过，全员立即修复)     │
  └──────────────────────────────────────────┘

Step 2: 冒烟测试 (30 min)
  ┌──────────────────────────────────────────┐
  │  用真实 API Key 跑 kanata-cli            │
  │  测试 1: 简单对话 "你好"                  │
  │  测试 2: 读文件 "读取 Cargo.toml"         │
  │  如果失败 → 对应负责人立即修复            │
  └──────────────────────────────────────────┘

Step 3: 核心场景测试 (2 hr)
  ┌──────────────────────────────────────────┐
  │  测试 3: 修改文件 (Edit 工具)             │
  │  测试 4: 多轮对话 (上下文连贯性)          │
  │  测试 5: /commit Skill                   │
  │  测试 6: 错误处理 (读不存在的文件)         │
  │  测试 7: Token 统计准确性                 │
  │                                          │
  │  每个测试:                                │
  │    通过 → 记录 ✅ → 下一个                │
  │    失败 → 记录现象 → 指定负责人 → 继续     │
  └──────────────────────────────────────────┘

Step 4: Bug Bash (D6 下午)
  ┌──────────────────────────────────────────┐
  │  所有 Step 3 中发现的 Bug 分配到人        │
  │  按优先级修复:                            │
  │    P0: 阻塞核心流程 → 必须当天修          │
  │    P1: 影响体验 → 尽量当天修              │
  │    P2: 小问题 → 记 Issue，MVP 后修        │
  └──────────────────────────────────────────┘
```

### 6.3 联调问题定位指南

遇到问题时，**先定位是哪一层的问题**:

```
用户输入 "读取 src/main.rs"
    │
    ↓
CLI 层 (Dev A) 是否正确传递给 Agent?
    │  验证: 在 AgentSession.send_message 入口打日志
    │  看到输入 → CLI 没问题，看 Agent
    │  没看到   → CLI 的问题，Dev A 修
    │
    ↓
Agent 层 (Dev B) 是否正确调用了 LLM?
    │  验证: 在 LLMClient.chat_stream 入口打日志
    │  看到 messages 数组 → Agent 没问题，看 Model
    │  没看到           → Agent 的问题，Dev B 修
    │
    ↓
Model 层 (Dev C) 是否正确返回了 tool_use?
    │  验证: 在 stream 中打日志看原始 SSE 事件
    │  看到 tool_use    → Model 没问题，看 Agent 的调度
    │  没看到 / 解析错误 → Model 的问题，Dev C 修
    │
    ↓
Agent 调度 (Dev B) 是否正确调用了 Read 工具?
    │  验证: 在 Tool.execute 入口打日志
    │  看到调用 → 看工具执行结果
    │  没看到   → 调度逻辑问题，Dev B 修
    │
    ↓
Tool 执行 (Dev B) 是否正确读取了文件?
    │  验证: 看 ToolResult 内容
    │  有内容 → 看结果是否正确回传给 LLM
    │  报错   → 工具实现问题，Dev B 修
    │
    ↓
结果渲染 (Dev A) 是否正确显示?
    │  验证: 看终端输出
    │  正确 → 全流程通过 ✅
    │  乱码/截断 → 渲染问题，Dev A 修
```

**快速定位命令**:

```bash
# 开启全量 debug 日志看完整链路
KANATA_LOG=debug cargo run -p kanata-cli

# 只看某一层的日志
KANATA_LOG=kanata_model=debug cargo run -p kanata-cli    # 只看 Model 层
KANATA_LOG=kanata_core=debug cargo run -p kanata-cli     # 只看 Agent 层
KANATA_LOG=kanata_tools=debug cargo run -p kanata-cli    # 只看 Tool 层
```

---

## 七、知识共享机制

### 7.1 每人维护自己的 crate README

```markdown
<!-- crates/kanata-model/README.md — Dev C 维护 -->

# kanata-model

## 职责
LLM 模型对接层，提供统一的流式调用抽象。

## 架构
```
ModelRouter → Provider (Anthropic / OpenAI) → StreamParser → StreamEvent
```

## 关键设计决策
- SSE 解析: 缓冲后整体解析 (ADR-001)
- tool_use JSON: 分片缓冲，ToolUseEnd 时一次性 parse

## 如何测试
```bash
cargo test -p kanata-model                        # 全部测试
cargo test -p kanata-model -- test_anthropic       # 只跑 Anthropic 相关
ANTHROPIC_API_KEY=sk-xxx cargo test -p kanata-model -- --ignored  # 集成测试 (需要真实 Key)
```

## 给其他人的接口
- `LLMClient` trait: Dev B 通过此 trait 调用
- `MockLLMClient`: Dev B 开发时用的 Mock
```

### 7.2 接口使用示例

每个 trait 的实现方必须提供**使用示例** (放在 doc comment 中):

```rust
/// LLM 客户端统一接口。
///
/// # 使用示例 (给 Dev B)
///
/// ```rust
/// let client = AnthropicClient::new(api_key, "claude-sonnet-4");
///
/// let stream = client.chat_stream(&messages, &tools, &system_prompt).await?;
///
/// pin_mut!(stream);
/// while let Some(event) = stream.next().await {
///     match event? {
///         StreamEvent::TextDelta(text) => print!("{}", text),
///         StreamEvent::ToolUseStart { id, name } => {
///             println!("Calling tool: {}", name);
///         }
///         StreamEvent::ToolUseDelta(json_chunk) => {
///             buffer.push_str(&json_chunk);
///         }
///         StreamEvent::ToolUseEnd => {
///             let input: Value = serde_json::from_str(&buffer)?;
///             // 调用对应工具...
///         }
///         StreamEvent::MessageEnd { usage } => {
///             println!("Tokens: {}", usage.input_tokens + usage.output_tokens);
///         }
///         StreamEvent::Error(e) => eprintln!("Error: {}", e),
///     }
/// }
/// ```
#[async_trait]
pub trait LLMClient: Send + Sync {
    async fn chat_stream(...) -> Result<...>;
    fn model_name(&self) -> &str;
}
```

### 7.3 技术分享

| 时机 | 形式 | 示例 |
|------|------|------|
| 遇到好的解决方案时 | IM 群发消息 + 代码片段 | "Rust 的 `pin_mut!` 宏处理 Stream 很方便，分享一下用法" |
| 踩坑时 | IM 群发消息 + 原因分析 | "Anthropic 的 SSE 有时会发空的 `event: ping`，需要过滤" |
| 每周五 | 15 min Demo + 讲解 | 每人演示本周做的功能，简要讲实现思路 |

---

## 八、异常处理协议

### 8.1 有人请假/生病

```
单人缺席 1 天:
  → 其他两人照常开发
  → 缺席人的当日任务顺延
  → 不阻塞其他人 (Mock 保证解耦)

单人缺席 ≥ 2 天:
  → Sponsor 决定是否调整任务分配
  → 关键路径任务由另一人临时接手
  → 接手规则:
      A 缺席 → B 接 CLI 关键 Bug
      B 缺席 → A 接简单工具, C 接 Agent 循环关键逻辑
      C 缺席 → B 接 Mock LLM, 用 Mock 先跑
```

### 8.2 技术方案分歧

```
分歧处理流程:

1. IM 或站会中各自陈述方案 (各 2 分钟)
2. 对比 Trade-off:
   - 实现复杂度
   - 对其他人的影响
   - 后续扩展性
3. 尝试达成共识 (5 分钟)
4. 达不成共识 → Sponsor 拍板
5. 决策记录到 ADR
6. 不翻旧账 — 决策一旦做出，全员执行
```

### 8.3 进度落后

```
某人进度落后预期:

1. 本人在站会中主动报告
2. 分析原因:
   a. 任务评估偏差 → 砍范围 (Sponsor 决定砍什么)
   b. 技术难点 → 求助 (15 分钟规则)
   c. 外部依赖 → 用 Mock/Stub 绕过

3. 调整方式 (Sponsor 决定):
   - 砍当前任务范围 (MVP 只做最核心的)
   - 其他人帮忙 (只限接口边界清晰的子任务)
   - 顺延到下一天 (不影响关键路径的前提下)

严格红线: D6 联调不能推迟
  → 如果某人 D5 还没完成，D5 晚上必须完成最低可联调版本
  → 最低可联调版本 = 编译通过 + 核心路径能跑 + 其他返回 todo!()
```

---

## 九、工具链统一

### 9.1 所有人安装相同工具

```bash
# 必装 — 版本必须一致
rustup default stable                     # 同一 Rust 版本
rustup component add rustfmt clippy

cargo install cargo-nextest               # 测试运行器
cargo install cargo-audit                 # 安全审计
cargo install cargo-watch                 # 热重载开发

# 推荐
cargo install cargo-expand                # 查看宏展开
cargo install tokio-console               # 异步任务调试
```

### 9.2 IDE 配置统一

```json
// .vscode/settings.json (推荐提交到仓库)
{
    "rust-analyzer.check.command": "clippy",
    "rust-analyzer.check.extraArgs": ["--workspace"],
    "editor.formatOnSave": true,
    "[rust]": {
        "editor.defaultFormatter": "rust-lang.rust-analyzer"
    },
    "rust-analyzer.imports.granularity.group": "module",
    "rust-analyzer.imports.prefix": "self"
}
```

### 9.3 环境变量约定

```bash
# .env.example (提交到仓库，实际 .env 不提交)

# 必须
ANTHROPIC_API_KEY=sk-ant-xxx

# 可选
OPENAI_API_KEY=sk-xxx
DEEPSEEK_API_KEY=sk-xxx

# 开发用
KANATA_LOG=info                    # 日志级别 (trace/debug/info/warn/error)
KANATA_CONFIG_DIR=~/.kanata        # 配置目录 (默认)
```

---

## 十、Checklist 汇总

### 每日 Checklist (每人下班前自检)

```
□ 今天的代码已 push 到自己的分支
□ cargo fmt + cargo clippy 通过
□ 新增功能有对应的单元测试
□ 新增 pub API 有文档注释
□ 如果改了 kanata-types，已通知其他人
□ PR 已提交 (如果有完成的功能)
□ 明天的任务清楚
```

### 每周 Checklist (周五 Sponsor Review)

```
□ 所有 PR 已合并到 main
□ main 分支 cargo build + cargo test 通过
□ Nightly Build 发布成功
□ 本周完成的功能列表更新到 CHANGELOG
□ 下周计划确认
□ 遗留 Bug / 技术债记录在 Issues 中
```

---

*本文档由 Kanata Team 共同遵守*
*最后更新: 2026-01-31*
