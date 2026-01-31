# 04 - Git 工作流规范

## 分支策略

- 🔴 `[MUST]` `main` 分支永远可编译、可运行，禁止直接 push
- 🔴 `[MUST]` 所有改动通过 PR 合并到 `main`
- 🔴 `[MUST]` 分支命名: `<type>/<short-description>`
  - `feat/cli-input-handler`
  - `fix/model-timeout-retry`
  - `refactor/core-session-lifecycle`
- 🟡 `[SHOULD]` 分支生命周期不超过 2 天，及时合并或关闭

## Commit 规范

- 🔴 `[MUST]` 遵循 Conventional Commits 格式:
  ```
  <type>(<scope>): <description>

  [optional body]
  ```
- 🔴 `[MUST]` type 取值:
  | type | 用途 |
  |------|------|
  | `feat` | 新功能 |
  | `fix` | Bug 修复 |
  | `refactor` | 重构（不改行为） |
  | `test` | 测试 |
  | `docs` | 文档 |
  | `ci` | CI/CD 配置 |
  | `chore` | 构建、依赖等杂务 |
- 🔴 `[MUST]` scope 取值: `cli`, `core`, `tools`, `skills`, `model`, `types`, `ci`
- 🔴 `[MUST]` description 使用英文小写，不以句号结尾
- 🟡 `[SHOULD]` 单个 commit 只做一件事

## PR 规范

- 🔴 `[MUST]` PR 标题遵循 Commit 规范格式
- 🔴 `[MUST]` PR 必须有至少 1 人 Review 后合并
- 🔴 `[MUST]` PR 合并前 CI 必须全绿
- 🟡 `[SHOULD]` PR 描述包含:
  - **What**: 改了什么
  - **Why**: 为什么改
  - **Test**: 怎么验证
- 🟡 `[SHOULD]` PR 不超过 400 行改动，大功能拆分多个 PR
- 🟡 `[SHOULD]` Review 轮转: A→B, B→C, C→A

## 合并策略

- 🔴 `[MUST]` 使用 **Squash Merge** 合并 PR，保持 `main` 历史干净
- 🔴 `[MUST]` 禁止 `git push --force` 到 `main`
- 🟡 `[SHOULD]` 合并前 rebase 到最新 `main`，解决冲突

## Tag 与发布

- 🟡 `[SHOULD]` 版本号遵循 SemVer: `vMAJOR.MINOR.PATCH`
- 🟡 `[SHOULD]` MVP 阶段使用 `v0.x.y`
- 🟢 `[MAY]` 每日构建自动生成 nightly tag
