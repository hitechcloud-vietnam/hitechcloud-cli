# Kế Hoạch 01 — Tầm Nhìn & Kiến Trúc Tổng Thể (Vision & Core Architecture)

**Command:** `hitechcloud`  
**Package/Repo:** `hitechcloud-cli`  
**Category:** Multi-provider AI Coding Agent / Developer CLI / Agent Platform + VS Code Extension  
**Target platform:** Linux, macOS, Windows + VS Code (all OS)  
**Chính sản phẩm:** `cli.hitechcloud.vn` · **MCP:** `mcp-cli.hitechcloud.vn`  

---

## 1. Executive Summary

**HiTechCloud CLI** (lệnh gọi: `hitechcloud`) là một AI-native command-line agent dành cho developer, DevOps, security engineer và enterprise, đi kèm **VS Code Extension** dùng chung lõi (core engine) để mang trải nghiệm agent vào ngay trong editor.

Mục tiêu không phải chỉ xây một CLI gọi LLM đơn thuần, mà xây một **Agent Platform** toàn diện với:

- **Multi-provider, multi-model**: Tương thích hoàn toàn OpenAI-compatible, Anthropic-compatible và các native adapters.
- **Tool calling, coding agent, sub-agents**: Tự động hóa tác vụ đọc/sửa file, phân tích code, chạy test, sửa lỗi.
- **Skills, Plugins, MCP, Hooks**: Hệ sinh thái mở rộng linh hoạt theo từng dự án hoặc cấp độ toàn cục.
- **Web/research mode**: Tìm kiếm và tổng hợp tài liệu online trong quá trình thực hiện task.
- **Permission/approval, session/resume, context management**: Kiểm soát an toàn các lệnh hệ thống, checkpoint khôi phục trạng thái.
- **Marketplace**: Kho chia sẻ skills, plugins, agents, MCP chính thức và cộng đồng.
- **VS Code Extension đồng bộ với CLI**: Dùng chung profile config, session, context và permission engine.
- **Hạ tầng domain riêng biệt**: `cli.hitechcloud.vn`, `mcp-cli.hitechcloud.vn` cùng các subdomain chuẩn hóa tiền/hậu tố `-cli` (`api-cli.`, `auth-cli.`, `registry-cli.`,...) được bảo vệ toàn diện qua WAF SafeLine (`vcwaf.hitechcloud.vn`), ngăn chặn tuyệt đối lộ IP máy chủ gốc.
- **Enterprise control plane**: Tích hợp sâu vào hệ sinh thái HiTechCloud (Cloud/Node.AI/Secure/Data/Mail).

Nguyên tắc cốt lõi: Đổi provider/model **không đổi workflow**, dù dùng qua terminal hay qua VS Code.

```bash
$ hitechcloud
```

Lệnh trên sẽ khởi động interactive agent ngay lập tức, sử dụng chung profile/model/skills/plugins với extension trong VS Code.

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

### P0 — Core
- Terminal-native AI agent (`hitechcloud`).
- Coding workflow tự động: đọc/sửa file, chạy shell commands, quản lý git branch/commit/diff.
- Model/provider switching linh hoạt không cần restart session.
- Streaming token thời gian thực, tool calling chuẩn xác, session persistence và hệ thống phê duyệt (approval system).

### P1 — Ecosystem
- Hệ thống Skills, Plugins, tích hợp MCP Server, custom Agents, Life-cycle Hooks.
- Marketplace tập trung, provider/model registry thông minh.
- **VS Code Extension**: Chat panel sidebar, inline agent (Cmd/Ctrl+K), review diff trước khi lưu.

### P2 — Enterprise & Infrastructure
- Single Sign-On (SSO), Role-Based Access Control (RBAC), central policy, private marketplace, audit trail toàn diện.
- HiTechCloud AI Gateway hỗ trợ failover, smart routing, cost optimization.
- Hệ thống Domain/subdomain hoàn chỉnh phục vụ registry, gateway, auth, tài liệu và status.

---

## 4. Competitive Design Goals

### Học hỏi từ Claude Code
- Kiến trúc Skills, Agents, Hooks, chuẩn kết nối MCP.
- Marketplace plugins phân tán, hỗ trợ project instructions (`HITECHCLOUD.md`).
- Phân quyền theo tool granularity (tool permissions).

### Học hỏi từ Codex CLI
- Trải nghiệm terminal-first trực quan, coding agent tương tác nhanh.
- Quy trình approval workflow an toàn khi thực thi lệnh nguy hiểm.
- Khả năng hiểu sâu repository (Repository Understanding) và thực thi các tác vụ dài (long-running tasks).

### Khác biệt cốt lõi của HiTechCloud
- **Multi-provider native**: Không bị khóa cứng (no vendor lock-in) vào bất kỳ nhà cung cấp AI nào.
- **Provider Abstraction SDK**: Dễ dàng bổ sung adapter provider mới dưới dạng plugin mà không phải rebuild core.
- **VS Code Extension dùng chung Core**: Cùng daemon lõi (`hitechcloud-agentd`), không phải 2 ứng dụng viết lại logic.
- **Hạ tầng độc lập & Bảo mật cao**: Sở hữu dải subdomain riêng biệt có định danh `-cli` (`cli.hitechcloud.vn`, `mcp-cli.hitechcloud.vn`, `api-cli.`, `auth-cli.`,...) tránh trùng lặp với các dịch vụ khác của hệ thống, toàn bộ lưu lượng định tuyến qua SafeLine WAF (`vcwaf.hitechcloud.vn`) để ẩn và bảo vệ IP gốc.
- **Tích hợp sâu hệ sinh thái HiTechCloud**: Tận dụng hạ tầng VPS, AI GPU Node, giải pháp bảo mật HiTechSecure, lưu trữ HiTechData và email HiTechMail.
- **Enterprise-ready & Production Hardened**: Hỗ trợ Secret Zero-Trust, Token Rate-limit Guard, Circuit Breaker, Session Crash Recovery, và Dynamic Cost Governor.

---

## 5. Core Architecture & Enterprise Engine Components

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
                   │ Circuit Breaker & Guard │
                   └───────────┬────────────┘
                               │
        ┌──────────────────────┼──────────────────────┐
        ▼                      ▼                      ▼
 ┌─────────────┐       ┌──────────────┐       ┌──────────────┐
 │ Tool Runtime │       │ Permission   │       │ Session      │
 │ (Sandbox &  │       │ Engine       │       │ Engine (WAL  │
 │  IPC Bridge)│       │ (Zero-Trust) │       │  & Snapshot) │
 └──────┬──────┘       └──────────────┘       └──────────────┘
        │
 ┌──────┴───────────────────────────────────────────┐
 │                                                    │
 ▼                                                    ▼
┌───────────────┐                          ┌────────────────────┐
│ Extensions     │                          │ Provider Layer      │
│ Skills/Plugins │                          │ Multi-gateway: GW, │
│ Agents/MCP/Hooks│                         │ Nube, Claude, OpenAI│
└───────────────┘                          └─────────┬───────────┘
                                                       ▼
                                              Model Providers
```

Core engine bao gồm:
- **Agent Runtime**: Điều phối vòng lặp suy nghĩ (ReAct Loop), lập kế hoạch, chọn tool, Circuit Breaker ngắt lặp vô hạn và kiểm tra kết quả.
- **Provider Abstraction**: Chuẩn hóa giao tiếp với các mô hình LLM, tự động retry exponential backoff và chuyển tiếp failover provider.
- **Tool Runtime**: Môi trường thực thi an toàn cho các tác vụ file, shell, git, dev tools, chạy trong sandbox cô lập.
- **Permission Engine**: Kiểm soát chính sách bảo mật Zero-Trust, xin phép người dùng trước khi thay đổi hệ thống và redactor che mờ secret.
- **Session Engine**: Lưu trữ lịch sử với cơ chế SQLite Write-Ahead Logging (WAL), checkpoint phục hồi sự cố (Crash Recovery).

Core là **một thư viện dùng chung duy nhất** (viết bằng Rust core với các language bindings), được cả CLI và VS Code Extension gọi vào — đảm bảo hành vi nhất quán, chỉ cần một nơi để tối ưu hiệu năng và vá lỗi bảo mật.

---

## 6. Các Tính Năng Nâng Cao Sẵn Sàng Cho Production (Production-Ready Features)

Nhằm đảm bảo sản phẩm đạt độ tin cậy 100% khi bàn giao cho người dùng và khách hàng doanh nghiệp:
1. **Self-Healing & Auto-Diagnostics**: Tự động phát hiện lỗi biên dịch hoặc lỗi cú pháp sau khi sửa code, tự động lặp kiểm thử và sửa lỗi không cần nhắc lệnh.
2. **Context Memory Hierarchy**: Phân tầng 3 lớp ngữ cảnh (User Memory toàn cục, Project Memory trong Git, và Ephemeral Session Context) giúp Agent duy trì kiến thức dự án qua nhiều phiên.
3. **Budget & Cost Governor**: Đặt ngưỡng cảnh báo và khóa cứng ngân sách token tối đa cho từng lệnh (`--max-budget-usd 0.50`), tránh việc Agent tiêu thụ token mất kiểm soát trong các vòng lặp lớn.
4. **Offline Air-Gapped Mode**: Hỗ trợ 100% hoạt động độc lập không cần Internet thông qua các endpoint Local LLM (Ollama, vLLM) dành cho ngân hàng và cơ quan chính phủ.
5. **Multi-Region Latency Routing**: Tự động đo Ping tới các gateway (`gateway-sg-cli`, `gateway-eu-cli`) để chọn đường truyền có độ trễ thấp nhất.

---

## 7. Final Architecture & Recommendation

```
                         HITECHCLOUD ECOSYSTEM
                                  │
                    ┌─────────────┴─────────────┐
             HiTechCloud AI (api.)      HiTechNode.AI
                    │                           │
                    └─────────────┬─────────────┘
                                  ▼
                    AI Gateway / Registry / Auth / WAF SafeLine
              (api-cli. · registry-cli. · mcp-cli. · auth-cli.hitechcloud.vn)
              [Bảo vệ qua WAF: vcwaf.hitechcloud.vn -> Whitelist IP gốc]
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

### Khuyến nghị kỹ thuật định hướng
1. **Không định vị chỉ là CLI wrapper**: Tránh phát triển theo hướng clone giao diện Claude Code hay Codex đơn giản. Định vị chính xác là **Agent Platform độc lập với provider**, phục vụ song song trên cả terminal và editor.
2. **3 lớp cốt lõi cần hoàn thiện vững chắc đầu tiên**:
   - **Agent Runtime**: Vòng lặp quan sát - hành động ổn định, chịu tải các prompt phức tạp.
   - **Provider Abstraction**: Tương thích triệt để với OpenAI và Anthropic API format.
   - **Extension Runtime**: Nền tảng thực thi Skills, Plugins, MCP, Custom Agents và Hooks.
3. **Mở rộng theo chu kỳ**: Sau khi lõi ổn định, từng bước triển khai Marketplace công khai, hoàn thiện VS Code Extension lên bản GA, mở rộng Enterprise Control Plane và triển khai AI Gateway đa vùng (multi-region).
