package cursor

import (
	"github.com/anthropics/agentsmesh/runner/internal/agentkit"
	"github.com/anthropics/agentsmesh/runner/internal/tokenusage"
)

func init() {
	// Cursor CLI 的 session 文件在 ~/.local/share/cursor-agent/versions/ 下，
	// 格式未公开 — 首版 opt out，避免 cross-agent contract test 要求 fixture。
	// 同时覆盖 slug ("cursor-cli") 和运行时 token-collection 用的 launch_command
	// ("cursor-agent")：tokenusage.Collect 以 pod.Agent=LaunchCommand 为 key，
	// 两个都登记才能在未来有人误加 parser 时仍命中 opt-out（参考 claude 同时
	// 注册 "claude"/"claude-code" 双 alias）。
	tokenusage.RegisterParserOptOut([]string{"cursor-cli", "cursor-agent"})
	// 严禁注册 "cursor" — Cursor IDE 启动器同名会冲突。
	//
	// 已知系统级隐患：Cursor CLI 通过 bundled Node 运行
	// (~/.local/share/cursor-agent/versions/<ver>/node)。agentkit 默认
	// processNameSet 已包含 "node"，monitor_check 用全局 set 而不是
	// (slug → process-name) 映射，所以 PTY 子树里出现的 node 进程会被
	// 笼统识别为 "agent process"，但无法区分属于 cursor / claude /
	// opencode 的哪一个。当前 cursor-agent 进程会先于 bundled node 在
	// process tree 中出现，所以正常路径 OK；若未来 Cursor 改成 wrapper
	// 直接 exec node 而非 cursor-agent，监控会退化为 ambiguous。
	// 修复方向：把 agentkit.processNameSet 改成 (process-name → slug) 映射。
	agentkit.RegisterProcessNames("cursor-agent")
}
