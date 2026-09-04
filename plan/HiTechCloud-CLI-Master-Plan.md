# HiTechCloud CLI — Master Product & Engineering Plan (v2)

**Command:** `hitechcloud`
**Package/Repo:** `hitechcloud-cli`
**Category:** Multi-provider AI Coding Agent / Developer CLI / Agent Platform + VS Code Extension
**Target platform:** Linux, macOS, Windows + VS Code (all OS)
**Chính sản phẩm:** `cli.hitechcloud.vn` · **MCP:** `mcp-cli.hitechcloud.vn`
**Định vị:** Kết hợp trải nghiệm coding agent kiểu Claude Code và Codex CLI, nhưng xây theo kiến trúc provider-agnostic, extensible, marketplace-first, có editor integration (VS Code) chạy chung lõi với CLI.

> **Thay đổi so với bản v1:** lệnh gọi CLI đổi từ `htc` → `hitechcloud`; bổ sung chương Domain/Subdomain Infrastructure; bổ sung chương đầy đủ về VS Code Extension (kiến trúc, tính năng, giao tiếp với CLI core, packaging, publish).

---

## Mục lục

1. Executive Summary
2. Product Vision
3. Product Goals
4. Competitive Design Goals
5. Core Architecture
6. Provider Abstraction Layer
7. API Compatibility (OpenAI/Anthropic-compatible)
8. Model Registry & Routing
9. Agent Runtime
10. Built-in Tools
11. Permission Engine & Security Model
12. Context Engine & Repository Intelligence
13. Session Engine & Checkpoints
14. Skills System
15. Plugin System
16. MCP Integration
17. Agents / Sub-agents
18. Hooks
19. Web / Research Mode
20. CLI Command Map (`hitechcloud`)
21. Interactive UX & In-session Commands
22. Project & Global Configuration
23. **Domain & Subdomain Infrastructure**
24. **VS Code Extension — Full Plan**
25. Repository Structure
26. Technology Stack
27. Marketplace Architecture & Security
28. Enterprise Control Plane
29. HiTechCloud Ecosystem Integration
30. Observability, Logging, Cost Tracking
31. Reliability & Fallback
32. Distribution & Auto-update
33. Testing Strategy & Security Hardening
34. Roadmap 12 tháng
35. MVP Definition of Done
36. Success Metrics
37. Team & Development Principles
38. Final Architecture & Recommendation
39. Appendix A — Command Cheat Sheet
40. Appendix B — Repository Bootstrap
41. Appendix C — First Engineering Tickets

---

## 1. Executive Summary

**HiTechCloud CLI** (lệnh gọi: `hitechcloud`) là một AI-native command-line agent dành cho developer, DevOps, security engineer và enterprise, đi kèm **VS Code Extension** dùng chung lõi (core engine) để mang trải nghiệm agent vào ngay trong editor.

Mục tiêu không phải chỉ xây một CLI gọi LLM, mà xây một **Agent Platform** với:

- Multi-provider, multi-model (OpenAI-compatible, Anthropic-compatible, native adapters).
- Tool calling, coding agent, sub-agents.
- Skills, Plugins, MCP, Hooks.
- Web/research mode.
- Permission/approval, session/resume, context management.
- Marketplace (skills/plugins/agents/MCP).
- VS Code Extension đồng bộ với CLI (cùng config, cùng session, cùng permission engine).
- Hạ tầng domain riêng: `cli.hitechcloud.vn`, `mcp-cli.hitechcloud.vn` và các subdomain mở rộng.
- Enterprise control plane, tích hợp hệ sinh thái HiTechCloud (Cloud/Node.AI/Secure/Data/Mail).

Nguyên tắc cốt lõi: đổi provider/model **không đổi workflow**, dù dùng qua terminal hay qua VS Code.

```bash
$ hitechcloud
```

sẽ khởi động interactive agent, dùng chung profile/model/skills/plugins với extension VS Code.

---

## 2. Product Vision

**Vision:** *One command. Any model. Any provider. Any editor. Any workflow.*

```
Developer (Terminal hoặc VS Code)
        │
        ▼
┌──────────────────────────────┐
│        HiTechCloud Core       │   ← dùng chung cho CLI & Extension
├──────────────────────────────┤
│ Agent Runtime                 │
│ Context Engine                │
│ Tool Runtime                  │
│ Permission Engine              │
│ Session Engine                 │
├──────────────────────────────┤
│ Skills · Plugins · Agents      │
│ Sub-agents · MCP · Hooks        │
├──────────────────────────────┤
│ Provider Abstraction           │
├──────────────────────────────┤
│ OpenAI · Anthropic · Azure     │
│ OpenAI-compatible               │
│ Anthropic-compatible            │
│ Local · Custom · HiTechCloud GW │
└──────────────────────────────┘
        │                    │
        ▼                    ▼
   `hitechcloud` CLI    VS Code Extension
```

---

## 3. Product Goals

**P0 — Core**
- Terminal-native AI agent (`hitechcloud`).
- Coding workflow: đọc/sửa file, chạy shell, git.
- Model/provider switching không restart.
- Streaming, tool calling, session persistence, approval system.

**P1 — Ecosystem**
- Skills, Plugins, MCP, custom Agents, Hooks.
- Marketplace, provider/model registry.
- **VS Code Extension** (chat panel, inline agent, diff review).

**P2 — Enterprise & Infrastructure**
- SSO, RBAC, central policy, private marketplace, audit.
- HiTechCloud AI Gateway.
- Domain/subdomain hạ tầng đầy đủ cho registry, gateway, auth, docs, status.

---

## 4. Competitive Design Goals

**Học từ Claude Code:** skills, agents, hooks, MCP, plugin marketplace, project instructions, tool permissions.

**Học từ Codex CLI:** terminal-first UX, coding agent, approval workflow, repository understanding, long-running task execution.

**Khác biệt của HiTechCloud:**
- Multi-provider native (không lock-in).
- Provider abstraction SDK cho phép thêm provider mà không rebuild core.
- VS Code Extension dùng chung core với CLI (không phải sản phẩm tách rời).
- Hạ tầng domain/subdomain riêng phục vụ registry, gateway, auth.
- Tích hợp hệ sinh thái HiTechCloud (Cloud, Node.AI, Secure, Data, Mail).

---

## 5. Core Architecture

```
                    ┌─────────────────────┐
                    │   User (Terminal/    │
                    │   VS Code/IDE)        │
                    └──────────┬───────────┘
                               │
             ┌─────────────────┼─────────────────┐
             ▼                                    ▼
      hitechcloud CLI                    VS Code Extension
             │                                    │
             └─────────────────┬──────────────────┘
                                ▼
                   ┌────────────────────────┐
                   │   Agent Runtime (core)  │
                   ├────────────────────────┤
                   │ Planner · Executor      │
                   │ Observer · Verifier     │
                   │ Context Manager         │
                   └───────────┬────────────┘
                               │
        ┌──────────────────────┼──────────────────────┐
        ▼                      ▼                      ▼
 ┌─────────────┐       ┌──────────────┐       ┌──────────────┐
 │ Tool Runtime │       │ Permission   │       │ Session      │
 │              │       │ Engine       │       │ Engine       │
 └──────┬──────┘       └──────────────┘       └──────────────┘
        │
 ┌──────┴───────────────────────────────────────────┐
 │                                                    │
 ▼                                                    ▼
┌───────────────┐                          ┌────────────────────┐
│ Extensions     │                          │ Provider Layer      │
│ Skills/Plugins │                          │ OpenAI/Anthropic     │
│ Agents/MCP/Hooks│                         │ Compatible/Local/GW  │
└───────────────┘                          └─────────┬───────────┘
                                                       ▼
                                              Model Providers
```

Core engine (agent runtime, provider abstraction, tool runtime, permission engine, session engine) là **một thư viện dùng chung** (Rust core + language bindings), được cả CLI và VS Code Extension gọi vào — đảm bảo hành vi nhất quán, một nơi để fix bug/security.

---

## 6. Provider Abstraction Layer

```
Provider
├── authenticate()
├── listModels()
├── getModel()
├── createResponse()
├── streamResponse()
├── toolCall()
├── embeddings()
├── usage()
└── healthCheck()
```

**Provider types:**
- Built-in: `openai`, `anthropic`, `azure-openai`, `hitechcloud`, `nube` (Nube.SH multi-format gateway: OpenAI & Anthropic), `local`.
- Compatible: `openai-compatible`, `anthropic-compatible`.
- Tương lai: `google`, `mistral`, `xai`, `deepseek`, `qwen`, `ollama`, `llama.cpp`, `custom-http`.

Provider mới có thể thêm dưới dạng plugin, không cần rebuild core:

```toml
[providers.google]
type = "plugin"
plugin = "hitechcloud-provider-google"
```

---

## 7. API Compatibility & Nube.SH Integration

**API Access Config:** Cùng một API Key (`NUBE_API_KEY`) dùng được đồng thời cho cả giao thức OpenAI-compatible, Anthropic-compatible và mainstream AI tools.

**OpenAI-compatible:** Chat Completions/Responses style, streaming, tool calling, structured output, vision, usage.
- **Base URL:** `https://ai.nube-api.com/v1`

```toml
[providers.nube-openai]
type = "openai-compatible"
base_url = "https://ai.nube-api.com/v1"
api_key_env = "NUBE_API_KEY"
```

**Anthropic-compatible:** Messages API, streaming, tool use, system prompts, vision, extended context.
- **Base URL:** `https://ai.nube-api.com` *(tuyệt đối không nối thêm `/v1` thừa)*

```toml
[providers.nube-anthropic]
type = "anthropic-compatible"
base_url = "https://ai.nube-api.com"
api_key_env = "NUBE_API_KEY"
```

> **Lưu ý từ Nube.SH Integration:** Cùng 1 API key dùng được cho cả hai chuẩn API. Không tự ý nối thêm `/v1` thừa vào Anthropic URL. Không trộn lẫn Base URL giữa 2 giao thức. Hệ thống tích hợp sẵn endpoint lấy giá động public: `GET https://ai.nube-api.com/v1/models/pricing` để tính toán chi phí (USD/1M tokens) cho `Nube-Choice`, `DeepSeek-V4-Flash`, `GLM-5.3-Flash`, `Qwen3.8-27B`, `Kimi-K3`.

Provider có API khác biệt lớn (không map được vào 2 chuẩn trên) phải có adapter native riêng.

---

## 8. Model Registry & Routing

```yaml
id: ag/gemini-3.7-flash-high
provider: hitechcloud
context_window: 1000000
capabilities:
  streaming: true
  tools: true
  vision: true
  reasoning: true
  structured_output: true
  web: true
metadata:
  speed: high
  quality: high
  cost_tier: medium
```

**Routing strategies:** `fixed`, `fallback`, `fastest`, `cheapest`, `quality`, `balanced`, `latency`, `custom`.

```toml
[routing]
strategy = "fallback"
primary = "hitechcloud/ag/gemini-3.7-flash-high"
fallback = ["anthropic/claude-sonnet", "openai/gpt-5.1"]
```

Enterprise có thể route theo loại tác vụ: coding → model premium; task đơn giản → model nhanh; repo nhạy cảm → model private/local; nghiên cứu → model có web.

---

## 9. Agent Runtime

```
Understand → Plan → Select tools → Execute → Observe → Verify → Continue/Finish
```

**Agent states:** `IDLE, THINKING, PLANNING, EXECUTING, WAITING_APPROVAL, VERIFYING, COMPACTING, COMPLETED, FAILED, CANCELLED`.

---

## 10. Built-in Tools

| Nhóm | Công cụ |
|---|---|
| Files | read_file, write_file, edit_file, delete_file, move_file, list_directory, search_files, grep |
| Shell | shell, exec, process_list, process_kill |
| Git | git_status, git_diff, git_log, git_branch, git_checkout, git_commit, git_restore |
| Dev | test, lint, format, build, package_manager |
| Network | http_get, http_request, download (đều qua permission engine) |

---

## 11. Permission Engine & Security Model

**Permission categories:** filesystem.read/write/delete, shell.execute, process.execute, network.connect/download, git.read/write, secrets.read, package.install.

**Approval levels:** `suggest`, `auto-safe`, `auto`, `unsafe`.

```bash
hitechcloud --approval auto-safe
```

Agent có thể tự đọc file, search, `git diff`, chạy test — nhưng phải hỏi trước khi `rm -rf`, `git reset --hard`, deploy production, truy cập secret, hoặc network request lạ.

**Security principles:** least privilege, explicit permission, secret redaction (API_KEY, TOKEN, PASSWORD, PRIVATE_KEY, COOKIE, AUTHORIZATION), auditability, safe defaults, project isolation.

**Prompt-injection defense:** nội dung từ README, web, issue, MCP output, tool output đều là *untrusted* và không được override system/security policy.

**Supply-chain security** cho skill/plugin/MCP/agent: chữ ký, checksum, version pinning, khai báo permission, xác thực publisher, quét bảo mật.

---

## 12. Context Engine & Repository Intelligence

Context ưu tiên: `System > Security policy > Project policy > User > Skill > Tool output`.

Context strategies: `full`, `relevant`, `semantic`, `summary`, `hybrid`.

Repository intelligence: index file/symbol/function/class/import/route/config/docs/git history; hỗ trợ filename/text/symbol/semantic/dependency search; sinh repository map.

---

## 13. Session Engine & Checkpoints

```bash
hitechcloud session list
hitechcloud session resume <id>
hitechcloud session export <id>
hitechcloud session delete <id>
```

Session lưu: id, project, provider, model, messages, tool_calls, files_changed, commands, approval_events, usage, cost, timestamps.

Checkpoint trước thay đổi lớn, có thể `hitechcloud checkpoint restore <id>`.

---

## 14. Skills System

```
my-skill/
├── SKILL.md
├── scripts/
├── templates/
├── references/
└── assets/
```

```markdown
---
name: laravel-review
description: Review Laravel application for bugs, security and performance.
version: 1.0.0
---
# Laravel Review
Review: routes, controllers, models, queries, auth, validation, performance.
```

Scope: `global (~/.hitechcloud/skills)`, `project (.hitechcloud/skills)`, `plugin`. Progressive loading: chỉ nạp metadata trước, load chi tiết khi kích hoạt.

**Built-in skill packs:** coding, code-review, debugging, testing, git, linux, docker, kubernetes, terraform, ansible, nginx, mysql/postgresql/redis, laravel, nextjs, nodejs, typescript, python, rust, security-review, devsecops, aws/azure/gcp.

---

## 15. Plugin System

Plugin manifest `hitechcloud-plugin.json`:

```json
{
  "name": "hitechcloud-devops",
  "version": "1.0.0",
  "description": "HiTechCloud DevOps toolkit",
  "permissions": ["filesystem.read", "filesystem.write", "shell.execute", "network.connect"],
  "skills": ["docker", "kubernetes", "terraform"],
  "agents": ["devops"]
}
```

Namespacing tránh xung đột: `/hitechcloud-devops:docker`. Plugin có thể expose command riêng, ví dụ `hitechcloud hcloud server list`.

---

## 16. MCP Integration

```bash
hitechcloud mcp list
hitechcloud mcp add
hitechcloud mcp remove
hitechcloud mcp test
hitechcloud mcp auth
```

```toml
[mcp.github]
transport = "stdio"
command = "github-mcp-server"
args = ["stdio"]

[mcp.company]
transport = "http"
url = "https://mcp-cli.hitechcloud.vn/servers/company"
```

Mỗi MCP server phải khai báo phạm vi quyền (filesystem/network/credentials/database/shell); agent không tự động cấp quyền.

---

## 17. Agents / Sub-agents

Built-in: `coder, reviewer, tester, debugger, researcher, security, devops, architect, documentation`.

```yaml
name: security-reviewer
model: premium-security-model
permissions: [filesystem.read, git.read]
tools: [grep, read_file, git_diff]
```

Sub-agent có context isolation, chạy song song với giới hạn `max_agents`, `max_tokens`, `max_runtime`, `max_cost`; parent agent tổng hợp và verify kết quả.

---

## 18. Hooks

Events: `session.start/end, before_tool, after_tool, before_command, after_command, before_file_write, after_file_write, agent.error, model.error`.

```yaml
hooks:
  after_file_write:
    - command: npm run lint
```

---

## 19. Web / Research Mode

Web provider abstraction (không khóa vào một search vendor): search, open/fetch, extract, summarize, cite. Research mode chạy multi-step, lưu research artifact, có thể mở rộng qua plugin (browser, search engine riêng, private knowledge base).

---

## 20. CLI Command Map (`hitechcloud`)

```
hitechcloud
hitechcloud exec "<prompt>"
hitechcloud init
hitechcloud model list|use|info
hitechcloud provider list|add|remove|test
hitechcloud auth login|logout|status
hitechcloud skill list|search|install|remove|update
hitechcloud plugin list|search|install|remove|update
hitechcloud agent list|run
hitechcloud mcp list|add|remove|test
hitechcloud session list|resume|export|delete
hitechcloud checkpoint list|restore
hitechcloud config get|set|edit
hitechcloud usage [--session|--month]
hitechcloud review [--diff]
hitechcloud ci "<prompt>"
hitechcloud serve
hitechcloud doctor
hitechcloud update
hitechcloud version
```

> Chỉ cần gõ `hitechcloud` (không tham số) là vào chế độ interactive agent.

---

## 21. Interactive UX & In-session Commands

```
╭──────────────────────────────────────╮
│        HiTechCloud CLI                │
│   Model: ag/gemini-3.7-flash-high     │
│   Provider: HiTechCloud                │
│                                        │
│ > Fix the authentication middleware    │
│                                        │
│ ◉ Analyzing repository                 │
│ ◉ Reading auth middleware               │
│ ◉ Updating middleware                   │
│ ◉ Running tests                         │
│                                        │
│ ✓ Tests passed                         │
╰──────────────────────────────────────╯
```

Trong session: `/help /model /provider /compact /context /skills /plugins /mcp /agents /session /diff /test /undo /clear /exit`.

---

## 22. Project & Global Configuration

**Global:** `~/.hitechcloud/` (config.toml, credentials/, sessions/, cache/, skills/, plugins/, agents/, mcp/, logs/).

**Project:** `HITECHCLOUD.md` + `.hitechcloud/` (config.toml, instructions.md, agents/, skills/, plugins/, hooks/, policies/).

```toml
[default]
provider = "hitechcloud"
model = "ag/gemini-3.7-flash-high"
approval = "auto-safe"

[providers.hitechcloud]
type = "openai-compatible"
base_url = "https://api-cli.hitechcloud.vn/v1"
api_key_env = "HITECHCLOUD_API_KEY"

[agent]
max_turns = 100
parallel_tools = true
checkpoint = true

[mcp]
enabled = true
registry_url = "https://mcp-cli.hitechcloud.vn"
```

**Env vars:** `HITECHCLOUD_API_KEY, HITECHCLOUD_BASE_URL, OPENAI_API_KEY, ANTHROPIC_API_KEY, HITECHCLOUD_MODEL, HITECHCLOUD_PROVIDER, HITECHCLOUD_APPROVAL, HITECHCLOUD_DEBUG`.

---

## 23. Domain & Subdomain Infrastructure (WAF SafeLine & Chuẩn Hóa `-cli`)

### 23.1 Bảo vệ qua SafeLine WAF (`vcwaf.hitechcloud.vn`) & Chống lộ IP gốc
- Toàn bộ subdomain của dịch vụ CLI & Agent được đặt sau cụm **SafeLine WAF (`vcwaf.hitechcloud.vn` - IP: 103.249.117.146)**.
- **Whitelist IP gốc**: Tường lửa tại các máy chủ ứng dụng nội bộ (Origin Servers) chỉ cho phép kết nối đến từ IP của SafeLine WAF, DROP toàn bộ lưu lượng trực tiếp từ Internet vào cổng 80/443 để ngăn chặn triệt để nguy cơ lộ IP gốc.
- DNS công khai chỉ trỏ CNAME về `vcwaf.hitechcloud.vn` hoặc bản ghi A về IP WAF.

### 23.2 Chuẩn hóa dải Subdomains có định danh `-cli`
Để tránh trùng lặp với các hệ thống hiện hữu của HiTechCloud, mọi subdomain liên quan đến CLI đều sử dụng hậu tố/tiền tố `-cli`:

| Subdomain | Vai trò | Qua SafeLine WAF |
|---|---|:---:|
| `cli.hitechcloud.vn` | Trang chính của sản phẩm CLI: landing page, tài liệu, hướng dẫn cài đặt, tải bản build, changelog. | ✅ |
| `mcp-cli.hitechcloud.vn` | MCP registry/gateway: liệt kê, xác thực, proxy các MCP server chính thức và của cộng đồng dùng cho `hitechcloud mcp add`. | ✅ |
| `docs-cli.hitechcloud.vn` | Tài liệu kỹ thuật đầy đủ (API reference, skill/plugin authoring guide, provider adapter guide). | ✅ |
| `api-cli.hitechcloud.vn` | Public API cho HiTechCloud AI Gateway (model routing, billing, usage) — provider `type = "hitechcloud"` trỏ vào đây. | ✅ |
| `auth-cli.hitechcloud.vn` | SSO/OAuth/Identity provider dùng chung cho CLI, VS Code Extension, Marketplace, Control Plane. | ✅ |
| `registry-cli.hitechcloud.vn` | Registry lõi cho Skills/Plugins/Agents/Templates (khác với `mcp-cli` chỉ dành riêng cho MCP). | ✅ |
| `marketplace-cli.hitechcloud.vn` | Giao diện web của Marketplace (official + community), nơi duyệt/tìm/review package. | ✅ |
| `download-cli.hitechcloud.vn` | CDN phân phối binary (Linux/macOS/Windows), checksum, chữ ký GPG/signing key. | ✅ |
| `update-cli.hitechcloud.vn` | Release channel endpoint cho `hitechcloud update` (stable/beta/nightly), version manifest. | ✅ |
| `status-cli.hitechcloud.vn` | Status page uptime cho Gateway/Registry/MCP/Auth. | ✅ |
| `telemetry-cli.hitechcloud.vn` | Thu thập telemetry ẩn danh (opt-in) — tách domain để dễ chặn/tắt theo policy enterprise. | ✅ |
| `enterprise-cli.hitechcloud.vn` | Control Plane cho tổ chức: user/team/policy/audit/billing riêng theo org. | ✅ |
| `community-cli.hitechcloud.vn` | Forum/Discussion cho skill/plugin authors, showcase, feedback. | ✅ |
| `gateway-sg-cli.hitechcloud.vn`, `gateway-eu-cli.hitechcloud.vn` *(khi scale)* | Multi-region routing cho AI Gateway để giảm latency theo khu vực. | ✅ |

Nguyên tắc đặt subdomain:
- Chức năng lõi người dùng gõ trực tiếp trong config (`api.`, `auth.`, `registry.`, `mcp-cli.`) nên **ngắn, ổn định, gần như không đổi** vì sẽ nằm trong config file của hàng ngàn máy dev.
- Trang mang tính marketing/tài liệu/cộng đồng (`docs.`, `community.`, `marketplace.`) có thể linh hoạt hơn về UI/CMS.
- Domain vận hành nội bộ (`telemetry.`, `status.`) nên tách hẳn để áp rate-limit/permission khác với domain public-facing.

---

## 24. VS Code Extension — Full Plan

### 24.1 Mục tiêu

Mang trải nghiệm agent của `hitechcloud` CLI vào thẳng VS Code, dùng **chung core engine** (không phải reimplement logic agent riêng cho extension) để đảm bảo:

- Cùng provider/model config, cùng skills/plugins/MCP, cùng session.
- Người dùng có thể bắt đầu task trên CLI và tiếp tục trên VS Code (và ngược lại) nhờ session engine chung.
- Một nơi duy nhất áp permission/security policy.

### 24.2 Kiến trúc

```
┌─────────────────────────────┐
│      VS Code (Extension      │
│      Host, Node.js)          │
├─────────────────────────────┤
│ Extension Frontend            │
│  - Chat/Agent panel (Webview) │
│  - Inline diff/edit UI         │
│  - Command palette actions      │
│  - Status bar (model/provider)  │
└──────────────┬───────────────┘
               │ JSON-RPC / IPC (local socket hoặc stdio)
               ▼
┌─────────────────────────────┐
│   hitechcloud-agentd          │  ← cùng binary core với CLI,
│   (background agent daemon)    │    chạy dạng service local
├─────────────────────────────┤
│ Agent Runtime · Context Engine │
│ Tool Runtime · Permission Engine│
│ Session Engine · Provider Layer │
└─────────────────────────────┘
```

- Extension **không** tự gọi thẳng API model; mọi request đi qua `hitechcloud-agentd` — một tiến trình nền chạy cùng core Rust/binary với CLI (mở bằng `hitechcloud serve --local` hoặc extension tự spawn khi cần).
- Giao tiếp qua JSON-RPC trên local Unix socket / named pipe (Windows), tránh mở network port ra ngoài máy.
- Nếu người dùng đã đăng nhập trên CLI (`hitechcloud auth login`), extension tự nhận diện credentials có sẵn trong `~/.hitechcloud/credentials/` (không cần đăng nhập lại).

### 24.3 Tính năng chính

| Tính năng | Mô tả |
|---|---|
| Chat/Agent Panel | Sidebar webview: chat với agent, xem plan, xem tool call theo thời gian thực. |
| Inline Agent (Cmd/Ctrl+K kiểu) | Chọn đoạn code → yêu cầu sửa/giải thích/refactor, xem diff ngay trong editor. |
| Diff Review UI | Mọi thay đổi file agent đề xuất hiện dưới dạng diff (giống Git diff), approve/reject từng hunk. |
| File & Repo Context | Tự động đính kèm file đang mở, selection, hoặc để agent tự search repo (dùng chung repository intelligence với CLI). |
| Approval Prompt | Khi agent cần chạy shell/network/destructive action → hiện modal xin phép, đồng bộ approval level với config CLI. |
| Skills/Plugins Explorer | Tree view liệt kê skill/plugin/agent/MCP đang cài, bật/tắt trực tiếp từ VS Code. |
| Session Sync | Mở lại đúng session đã chạy trên terminal (`hitechcloud session resume`) ngay trong panel. |
| Model/Provider Switcher | Dropdown trên status bar để đổi model/provider, đọc từ cùng model registry. |
| Terminal Bridge | Nút "Run in terminal" để chuyển 1 lệnh agent đề xuất sang integrated terminal chạy thủ công nếu người dùng muốn kiểm soát. |
| Test/Lint Integration | Sau khi agent sửa code, tự chạy test/lint đã cấu hình trong `HITECHCLOUD.md` và hiện kết quả pass/fail ngay trong panel. |
| Multi-root Workspace | Hỗ trợ nhiều folder trong 1 workspace, mỗi folder có config `.hitechcloud/` riêng. |

### 24.4 Command Palette (`Ctrl/Cmd+Shift+P`)

```
HiTechCloud: Start Agent
HiTechCloud: Ask About Selection
HiTechCloud: Explain File
HiTechCloud: Fix Diagnostics (dùng lỗi Problems panel làm context)
HiTechCloud: Generate Tests
HiTechCloud: Review Changes (git diff)
HiTechCloud: Switch Model
HiTechCloud: Switch Provider
HiTechCloud: Manage Skills
HiTechCloud: Manage Plugins
HiTechCloud: Manage MCP Servers
HiTechCloud: Resume Session
HiTechCloud: Open Config
HiTechCloud: Doctor (chẩn đoán kết nối/provider/MCP)
```

### 24.5 Settings (VS Code `settings.json`)

```json
{
  "hitechcloud.provider": "hitechcloud",
  "hitechcloud.model": "ag/gemini-3.7-flash-high",
  "hitechcloud.approvalMode": "auto-safe",
  "hitechcloud.agentdPath": "auto",
  "hitechcloud.showInlineDiff": true,
  "hitechcloud.autoRunTestsAfterEdit": true,
  "hitechcloud.mcpRegistryUrl": "https://mcp-cli.hitechcloud.vn",
  "hitechcloud.telemetry.enabled": false
}
```

### 24.6 Bảo mật trong Editor

- Extension chạy trong Extension Host sandbox mặc định của VS Code; mọi hành động filesystem/shell thật sự vẫn đi qua Permission Engine của core (không bypass).
- Diff phải được người dùng **approve rõ ràng** trước khi ghi vào file khi ở approval mode `suggest`/`auto-safe`.
- Không tự động gửi toàn bộ workspace ra ngoài; context engine áp dụng chiến lược "relevant/hybrid" giống CLI.
- Credentials không lưu trong `settings.json`; luôn qua `~/.hitechcloud/credentials/` hoặc OS keychain.

### 24.7 Packaging & Distribution

- Build bằng `vsce` (Visual Studio Code Extension CLI), publish `.vsix`.
- Kênh phân phối:
  - **VS Code Marketplace** (chính thức, công khai).
  - **Open VSX** (cho các fork như VSCodium, Cursor nếu tương thích).
  - **Private VSIX** cho enterprise (tải từ `download.hitechcloud.vn`, không public marketplace).
- Extension tự phát hiện nếu `hitechcloud` CLI core chưa cài → gợi ý cài (`curl ... | sh` hoặc trỏ tới `download.hitechcloud.vn`), hoặc tự tải core binary phù hợp OS/arch vào `~/.hitechcloud/bin/`.
- Versioning độc lập với CLI nhưng khai báo **compatibility matrix** (extension v1.x tương thích core v1.y trở lên) để tránh lệch API IPC.

### 24.8 Roadmap riêng cho Extension

| Giai đoạn | Nội dung |
|---|---|
| Alpha | Chat panel cơ bản, inline diff, kết nối `hitechcloud-agentd` qua local socket. |
| Beta | Approval UI đầy đủ, Skills/Plugins explorer, session sync 2 chiều với CLI. |
| GA | Test/Lint integration, multi-root workspace, publish VS Code Marketplace + Open VSX. |
| Enterprise | Private VSIX, policy do `enterprise.hitechcloud.vn` đẩy xuống (chặn model/provider theo tổ chức ngay trong extension). |

---

## 25. Repository Structure

```
hitechcloud-cli/
├── apps/
│   ├── cli/                      # binary `hitechcloud`
│   └── vscode-extension/         # extension VS Code
├── crates/
│   ├── core/
│   ├── agent-runtime/
│   ├── provider-sdk/
│   ├── provider-openai/
│   ├── provider-anthropic/
│   ├── provider-compatible/
│   ├── context/
│   ├── tools/
│   ├── permissions/
│   ├── sessions/
│   ├── skills/
│   ├── plugins/
│   ├── mcp/
│   ├── agents/
│   ├── agentd/                   # background daemon dùng bởi extension
│   └── marketplace/
├── sdk/
│   ├── typescript/
│   └── python/
├── registry/
├── marketplace/
├── skills/
├── plugins/
├── docs/
├── examples/
└── tests/
```

---

## 26. Technology Stack

- **Core/CLI:** Rust — single binary, khởi động nhanh, cross-platform, quản lý process/sandbox tốt.
- **Agentd (daemon cho extension):** cùng codebase Rust, expose JSON-RPC qua local socket.
- **VS Code Extension:** TypeScript, VS Code Extension API, Webview (React) cho chat panel.
- **SDK author skill/plugin:** TypeScript và Python (tùy chọn, không bắt buộc cho MVP).
- **Config:** TOML/YAML/JSON cho manifest.
- **Local storage:** SQLite cho session/index/cache metadata.
- **Container:** hỗ trợ OCI cho sandbox/plugin packaging ở giai đoạn sau.

---

## 27. Marketplace Architecture & Security

```
                    Marketplace (marketplace-cli.hitechcloud.vn)
                                  │
              ┌───────────────────┼───────────────────┐
              ▼                   ▼                   ▼
          Official            Community            Private
    (registry-cli.hitechcloud.vn)       (enterprise-cli.hitechcloud.vn)
```

Mỗi package cần: name, version, publisher, checksum, signature, permissions, dependencies, compatibility, security_status. Có thể tích hợp malware scan, static analysis, dependency scan, signature verification, publisher verification.

```bash
hitechcloud marketplace add
hitechcloud marketplace list
hitechcloud marketplace remove
```

---

## 28. Enterprise Control Plane (`enterprise-cli.hitechcloud.vn`)

Quản lý: Organizations, Users, Teams, Models, Providers, Policies, Skills, Plugins, MCP, Usage, Billing, Audit — áp dụng đồng thời cho CLI và VS Code Extension.

```yaml
policy:
  allowed_providers: [hitechcloud]
  allowed_models: [approved-model-1, approved-model-2]
  allow_network: false
  allow_shell: true
  blocked_commands: ["rm -rf /", "shutdown", "reboot"]
  marketplace:
    mode: private
```

---

## 29. HiTechCloud Ecosystem Integration

| Hệ sinh thái | Tích hợp |
|---|---|
| HiTechCloud (hạ tầng) | VPS, Cloud, DNS, Network, Backup, Monitoring. |
| HiTechNode.AI | Model/agent/skill marketplace, GPU workloads, model gateway. |
| HiTechSecure | SafeLine WAF (`vcwaf.hitechcloud.vn`) reverse proxy chống lộ IP gốc, security scanning, SOC, DevSecOps skills. |
| HiTechMail | SMTP/DNS/DKIM/DMARC skills cho automation. |
| HiTechData | Object storage, backup, database/MCP tools. |

Provider gốc `type = "hitechcloud"` route qua **AI Gateway** (`api-cli.hitechcloud.vn`) tới OpenAI/Anthropic/Gemini/Mistral/Local — cho phép billing tập trung, routing, failover, kiểm soát usage theo tổ chức.

---

## 30. Observability, Logging, Cost Tracking

**Log levels:** error/warn/info/debug/trace, lưu tại `~/.hitechcloud/logs/`, không ghi plaintext secrets.

**Metrics:** request_count, token_input/output, latency, tool_latency, provider_error, model_error, session_duration, cost.

```bash
hitechcloud usage
hitechcloud usage --session
hitechcloud usage --month
```

Telemetry ẩn danh **tắt mặc định**, opt-in, gửi tới `telemetry-cli.hitechcloud.vn` riêng biệt được bảo vệ sau SafeLine WAF.

---

## 31. Reliability & Fallback

```
Provider A → (lỗi) → Provider B → (lỗi) → Provider C
```

Phân biệt rõ: `authentication_error, rate_limit, timeout, server_error, invalid_request, context_limit, tool_error, network_error`. Không fallback với lỗi logic của chính request (ví dụ input sai).

Offline/local mode hỗ trợ Ollama, llama.cpp, endpoint OpenAI/Anthropic-compatible chạy local (`http://127.0.0.1:11434/v1`).

---

## 32. Distribution & Auto-update

| OS | Gói |
|---|---|
| Linux | .deb, .rpm, .tar.gz |
| macOS | brew, .pkg, .tar.gz |
| Windows | winget, .exe, .msi, .zip |
| Container | `docker.io/hitechcloud/hitechcloud-cli` |
| VS Code | VS Code Marketplace, Open VSX, private .vsix (`download-cli.hitechcloud.vn`) |

```bash
hitechcloud update   # kiểm tra update-cli.hitechcloud.vn, kênh stable/beta/nightly
```

Enterprise có thể tắt auto-update và chỉ định version cố định qua policy.

---

## 33. Testing Strategy & Security Hardening

- **Unit:** provider adapters, config, permissions, context, skills, plugins, sessions.
- **Integration:** OpenAI-compatible, Anthropic-compatible, MCP, filesystem, shell, git, extension ↔ agentd IPC.
- **End-to-end:** tạo project → sửa code → test → git diff → resume, cả trên CLI lẫn VS Code.
- **Security:** command injection, path traversal, secret leakage, plugin permission escalation, MCP abuse, malicious skill, prompt injection.
- **Origin IP Protection:** Cấu hình firewall máy chủ gốc chỉ whitelist IP của SafeLine WAF (`vcwaf.hitechcloud.vn`), DROP toàn bộ kết nối thẳng.

---

## 34. Roadmap 12 tháng

| Giai đoạn | Thời gian | Nội dung |
|---|---|---|
| Phase 0 | 2 tuần | Architecture RFC, provider contract, security model, CLI UX, SafeLine WAF setup & domain/hạ tầng bootstrap (`cli.`, `mcp-cli.`, `auth-cli.`, `api-cli.`). |
| Phase 1 | Tháng 1–2 | MVP `hitechcloud` CLI: OpenAI/Anthropic-compatible, file/shell/git tools, streaming. |
| Phase 2 | Tháng 3–4 | Session, context engine, approvals, checkpoints, model profiles, fallback routing. |
| Phase 3 | Tháng 5–6 | Skills + Agents + MCP (`mcp-cli.hitechcloud.vn` go-live), built-in skill packs. |
| Phase 4 | Tháng 7–8 | Plugin SDK, marketplace registry (`registry-cli.`, `marketplace-cli.`), signing/verification. |
| Phase 5 | Tháng 9–10 | **VS Code Extension Alpha→Beta**, web/research, sub-agents, parallel orchestration. |
| Phase 6 | Tháng 11–12 | Enterprise control plane (`enterprise-cli.hitechcloud.vn`), VS Code Extension GA, packages đa nền tảng, public beta. |

---

## 35. MVP Definition of Done

- [ ] Cài và chạy `hitechcloud` trên Linux/macOS/Windows.
- [ ] Đăng nhập được ít nhất OpenAI-compatible và Anthropic-compatible.
- [ ] Agent đọc/sửa file, chạy test, xem git diff.
- [ ] Approval mode + command safety hoạt động.
- [ ] Session resume hoạt động.
- [ ] `/skills` và skill loader hoạt động.
- [ ] MCP client kết nối được `mcp-cli.hitechcloud.vn`.
- [ ] Model/provider switch không cần restart.
- [ ] Logs/debug mode.
- [ ] Test suite cho provider adapter, tool execution, permission, session persistence.

---

## 36. Success Metrics

**Developer:** time-to-first-task < 5 phút, task success rate, thời gian hoàn thành trung vị.
**Performance:** CLI startup, first-token latency, tool execution latency, memory usage.
**Ecosystem:** số skill/plugin đã cài, MCP server đang hoạt động, package trên marketplace, số cài đặt VS Code Extension.
**Business:** active users, tổ chức enterprise, lưu lượng AI Gateway, retention.

---

## 37. Team & Development Principles

**Team tối thiểu cho MVP:** 1 Architect · 2 Rust/CLI Engineer · 1–2 Agent/LLM Engineer · 1 Security/DevOps · (thêm 1 Frontend/TypeScript Engineer khi bắt đầu VS Code Extension).

**Nguyên tắc:** Provider-agnostic first · Security by default · Local-first · Extension là first-class citizen · Everything observable · Stable API trước khi thêm tính năng · Backward compatibility · Không lock-in model.

---

## 38. Final Architecture & Recommendation

```
                         HITECHCLOUD ECOSYSTEM
                                  │
                    ┌─────────────┴─────────────┐
             HiTechCloud AI (api.)      HiTechNode.AI
                    │                           │
                    └─────────────┬─────────────┘
                                  ▼
                    AI Gateway / Registry / Auth
              (api. · registry. · mcp-cli. · auth.hitechcloud.vn)
                                  │
                                  ▼
                    ┌───────────────────────────┐
                    │      HiTechCloud Core      │
                    │  (Agent Runtime · Context  │
                    │   Permission · Session)    │
                    └─────────────┬─────────────┘
                       ┌──────────┴──────────┐
                       ▼                     ▼
              `hitechcloud` CLI       VS Code Extension
                       │                     │
                       └──────────┬──────────┘
                                  ▼
                       Provider Abstraction
                                  │
              ┌───────────────────┼────────────────────┐
              ▼                   ▼                    ▼
        OpenAI-compatible   Anthropic-compatible    Native
                                  │
                                  ▼
                           MODEL ECOSYSTEM
```

**Khuyến nghị cuối cùng:** đừng định vị đây là "CLI giống Claude Code/Codex" — định vị đúng là **một Agent Platform provider-agnostic**, có mặt cả ở terminal (`hitechcloud`) lẫn editor (VS Code), dùng chung một lõi. Ba lớp cần làm tốt nhất ngay từ đầu: **Agent Runtime, Provider Abstraction, Extension Runtime (Skills/Plugins/MCP/Agents/Hooks)**. Sau đó mới mở rộng Marketplace, VS Code Extension GA, Enterprise Control Plane và AI Gateway đa vùng.

---

## 39. Appendix A — Command Cheat Sheet

```bash
# Khởi động
hitechcloud

# One-shot
hitechcloud exec "Fix the failing tests"

# Khởi tạo project
hitechcloud init

# Model & provider
hitechcloud model list
hitechcloud model use <model>
hitechcloud provider list
hitechcloud provider add
hitechcloud provider test

# Auth
hitechcloud auth login
hitechcloud auth status

# Skills / Plugins
hitechcloud skill list
hitechcloud skill search docker
hitechcloud skill install docker
hitechcloud plugin list
hitechcloud plugin install hitechcloud-devops

# MCP (mcp-cli.hitechcloud.vn)
hitechcloud mcp list
hitechcloud mcp add
hitechcloud mcp test

# Agents
hitechcloud agent list
hitechcloud agent run security

# Sessions
hitechcloud session list
hitechcloud session resume

# Chẩn đoán & usage
hitechcloud doctor
hitechcloud usage
```

---

## 40. Appendix B — Repository Bootstrap

```bash
mkdir hitechcloud-cli && cd hitechcloud-cli
git init
cargo init --name hitechcloud-cli

mkdir -p \
  apps/cli apps/vscode-extension \
  crates/core crates/agent-runtime crates/provider-sdk \
  crates/provider-openai crates/provider-anthropic crates/provider-compatible \
  crates/context crates/tools crates/permissions crates/sessions \
  crates/skills crates/plugins crates/mcp crates/agents crates/agentd \
  crates/marketplace docs examples tests
```

**Milestone 0**
```
├── CLI `hitechcloud` boots
├── Config loads
├── Provider interface exists
├── OpenAI-compatible adapter works
├── Anthropic-compatible adapter works
├── Streaming works
├── Tool interface works
├── Agent loop works
└── agentd exposes JSON-RPC cho extension (skeleton)
```

---

## 41. Appendix C — First Engineering Tickets

```
HTC-001  Bootstrap Rust workspace (crates + apps/cli + apps/vscode-extension)
HTC-002  Define Provider trait
HTC-003  Implement OpenAI-compatible adapter
HTC-004  Implement Anthropic adapter
HTC-005  Implement streaming event model
HTC-006  Implement filesystem tools
HTC-007  Implement shell tool
HTC-008  Implement permission engine
HTC-009  Implement basic agent loop
HTC-010  Implement interactive terminal UI (`hitechcloud`)
HTC-011  Git tools
HTC-012  Session storage
HTC-013  Session resume
HTC-014  Context compaction
HTC-015  Model registry
HTC-016  Provider fallback
HTC-017  Skills loader
HTC-018  MCP client (point to mcp-cli.hitechcloud.vn)
HTC-019  Plugin manifest
HTC-020  Marketplace prototype (registry-cli.hitechcloud.vn)
HTC-021  agentd JSON-RPC skeleton (local socket)
HTC-022  VS Code Extension skeleton: connect to agentd, basic chat panel
HTC-023  VS Code Extension: inline diff review UI
HTC-024  VS Code Extension: approval modal synced with permission engine
HTC-025  Setup SafeLine WAF (vcwaf.hitechcloud.vn) + Origin IP whitelist & bootstrap -cli domains
```
