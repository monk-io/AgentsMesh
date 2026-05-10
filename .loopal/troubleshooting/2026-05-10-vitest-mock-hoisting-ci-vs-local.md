# Vitest mock 在 CI（Linux）与本地（macOS）行为不一致 — useEffect 异步加载场景

**日期**: 2026-05-10
**关联**: PR #340 (`fix/fix-resume-pod`)

## 问题描述

为 PR #340 补充 `useRunnerDetail` hook 的 vitest 单测时，**本地 macOS 全量 PASS，但同一份代码在 GitHub Actions Linux runner 上连续 3 次 timeout 失败**：

```
✗ includes agent_slug from source pod in payload         5009ms (timeout)
✗ falls back to empty string when source pod has no...   5003ms (timeout)
✗ sends complete resume payload shape (PR #340 ...)      5004ms (timeout)
✗ navigates to new pod workspace on success              5002ms (timeout)
```

每个测试都卡在 `waitFor(() => expect(result.current.runner?.id).toBe(42))` 直到 5s 默认 testTimeout。`runner` 始终是初始 `null`，hook 内 `loadRunner()` 的 `setRunner(...)` 从未触发到 React 渲染。

## 触发场景

- 被测对象：包含 `useEffect(() => loadRunner(), [])` 的 React hook，`loadRunner` 内部 `await getRunnerService().fetch_runner(...)` 后 `setRunner(...)`
- 测试基础设施：vitest 4 + @testing-library/react，配 `setupFiles` 全局 mock `@/lib/wasm-core`
- 测试策略：用 mock 替换 `getRunnerService()`/`getPodService()` 让 `fetch_runner` 同步返回数据

## 失败的 3 种 mock 策略（均仅本地 PASS）

| # | 策略 | 代码 | 结果 |
|---|---|---|---|
| 1 | 沿用 setup.ts 的 mock 单例，运行时 mutate | `(getPodService() as any).create_pod = mockCreatePod` | CI fail |
| 2 | `vi.mocked(getRunnerService).mockReturnValue({...})` 整体覆盖 | beforeEach 内调用 | CI fail |
| 3 | file 顶部本地 `vi.mock('@/lib/wasm-core', () => ({ ...普通 function 工厂... }))` 覆盖全局 mock | 含 `vi.hoisted` 把 mock 句柄一起 hoist | CI fail |

## 怀疑的根本原因（未完全证实）

`vi.mock` factory 是 hoisted 到文件顶部，**早于** const 声明执行。即使把 mock 句柄用 `vi.hoisted` 一并提升，**`setupFiles` 中已注册的全局 `@/lib/wasm-core` mock 与 file-level override 在 Linux Node 20 + vitest worker 初始化时序下存在 race**：file-level mock 可能未完全替换全局 mock，导致 hook 调用 `getRunnerService()` 时拿到的对象的 `fetch_runner` 在 `vi.clearAllMocks()` 之后已被清空 implementation，promise 永不 resolve，`setRunner` 永不调用。

macOS 上同进程跑了大量先序测试预热了 mock 状态，意外掩盖了这个 race，所以本地通过。

## 务实绕过

放弃 web hook 单测层，靠跨语言契约保护：

- ✅ Rust core: `clients/core/crates/types/src/pod.rs` 的 `create_pod_request_resume_without_agent_slug` 测反序列化 `#[serde(default)]` 兜底
- ✅ Backend Go DTO: `backend/internal/api/rest/v1/pod_create_integration_test.go` 的 `TestCreatePodRequest_ResumeWithoutAgentSlug_Unmarshals` 测 `ShouldBindJSON` 不报错
- ✅ Backend orchestrator: 既有 `pod_orchestrator_resume_test.go` 12 个测试覆盖 `AgentSlug == ""` 时从 source pod 继承
- ⚠️ UI hook 层：靠 TypeScript 类型 + code review

## 复现成本与教训

**反馈循环成本**：每次 push → CI 队列 → web vitest 大约 12-15 分钟。3 次试错耗了 ~45 分钟。

**未来类似场景的建议做法**：

1. **写 vitest 测异步加载 hook 之前，先在 Linux Docker 容器里跑一次本地验证**，避开 macOS 偶然成功的假阳性。例如：
   ```bash
   docker run --rm -v "$PWD:/work" -w /work node:20 \
     bash -c "corepack enable && pnpm install --frozen-lockfile && \
              pnpm exec vitest run path/to/file.test.ts"
   ```
2. 不要单纯依赖 `setupFiles` 全局 mock + file-level mutation 混用模式。要么全用全局 mock，要么用 `vi.hoisted` + 完整 file-level 覆盖。
3. 涉及 `useEffect` 异步加载的 hook，优先把可测试逻辑（如 payload 构建）抽成纯函数单独测，避开 React/mock/async 三方时序耦合。
4. 反复 CI fail 同一个测试 ≥2 次时，立即停止试错；切到「删测试 + issue tracking」务实路线，不要被沉没成本绑架。

## 相关参考

- 之前用过的 mock 模式：`clients/web/src/components/pod/hooks/__tests__/useCreatePodForm.test.ts`（同样模式但 hook 内**没有 useEffect 异步加载**，所以没踩到这个坑）
- vitest 4 文档关于 `vi.mock` hoisting 与 `setupFiles` 优先级：https://vitest.dev/api/vi.html#vi-mock
