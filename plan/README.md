# HiTechCloud CLI — Danh Mục Kế Hoạch Phát Triển (Modular Plans)

Hệ thống kế hoạch kỹ thuật và sản phẩm của **HiTechCloud CLI (`hitechcloud`)** đã được phân rã thành **9 module chuyên biệt** đặt trong thư mục `plan/`. Việc phân tách này giúp các nhóm kỹ sư (Core Rust, Agent Runtime, Extension, DevOps/Infra) có thể độc lập chỉnh sửa, theo dõi và cập nhật mà không bị xung đột tài liệu.

> **Tài liệu gốc tham chiếu:** `HiTechCloud-CLI-Master-Plan.md` (Lưu trữ toàn văn Master Plan v2).

---

## 🗺️ Bản đồ điều hướng các Sub-plans

| File | Tên Kế Hoạch | Chương Trong Master Plan | Nội Dung Trọng Tâm |
|---|---|---|---|
| [`01-vision-and-architecture.md`](./01-vision-and-architecture.md) | **Tầm Nhìn & Kiến Trúc Tổng Thể** | Chương 1, 2, 3, 4, 5, 38 | Executive Summary, Định vị sản phẩm, Goals (P0/P1/P2), Phân tích đối thủ (Claude Code vs Codex), Sơ đồ kiến trúc Core, Tính năng nâng cao sẵn sàng cho Production (Self-Healing, Circuit Breaker, Memory Hierarchy, Cost Governor) & Khuyến nghị chung |
| [`02-provider-and-models.md`](./02-provider-and-models.md) | **Tầng Trừu Tượng Provider & Models** | Chương 6, 7, 8, 31 | Provider Abstraction Layer (trait/SDK), Tương thích OpenAI/Anthropic API, Tích hợp chuyên sâu Nube.SH (Multi-format Gateway, Live Pricing API, đánh giá workload), Model Registry & Chiến lược Routing, Độ tin cậy & Fallback |
| [`03-agent-runtime-and-engine.md`](./03-agent-runtime-and-engine.md) | **Agent Runtime & Lõi Động Cơ** | Chương 9, 10, 11, 12, 13, 19 | Vòng lặp Agent (ReAct loop), Built-in Tools (Files, Shell, Git), Permission Engine & Bảo mật sandbox, Context Engine & Memory Hierarchy, Session WAL & Crash Recovery, Web/Research Mode, Self-Healing Loop, Circuit Breaker |
| [`04-extensibility-skills-plugins-mcp.md`](./04-extensibility-skills-plugins-mcp.md) | **Hệ Thống Mở Rộng: Skills, Plugins & MCP** | Chương 14, 15, 16, 17, 18, 27 | Cấu trúc Skills & Skill packs, Plugin System & Manifest, MCP Client Integration (`mcp-cli.hitechcloud.vn`), Sub-agents điều phối song song & Consensus Protocol, Hooks, Kiến trúc Marketplace & Sandbox Detonation |
| [`05-cli-ux-and-commands.md`](./05-cli-ux-and-commands.md) | **Giao Diện CLI, Lệnh & Cấu Hình** | Chương 20, 21, 22, 39 | Bản đồ lệnh `hitechcloud`, Trải nghiệm Interactive TUI, Lệnh in-session (`/`), Cấu hình Project (`HITECHCLOUD.md`) & Global (`~/.hitechcloud/`), Command Cheat Sheet, Thiết lập ngân sách `--max-budget-usd` |
| [`06-vscode-extension.md`](./06-vscode-extension.md) | **Kế Hoạch VS Code Extension** | Chương 24 | Kiến trúc Extension ↔ `hitechcloud-agentd` qua IPC/JSON-RPC, Chat Panel, Inline Diff Review, Command Palette, Settings, Hot Reload Daemon, Cost HUD, Packaging (VSIX/Marketplace) |
| [`07-infrastructure-and-domains.md`](./07-infrastructure-and-domains.md) | **Hạ Tầng Domain, WAF & Doanh Nghiệp** | Chương 23, 28, 29 | Quy hoạch các subdomain chuẩn hóa hậu tố `-cli` (`cli.`, `mcp-cli.`, `api-cli.`, `auth-cli.`,...), Bảo vệ qua SafeLine WAF (`vcwaf.hitechcloud.vn`), Chống lộ IP gốc (Origin IP Whitelisting), Enterprise Control Plane (`enterprise-cli.`), Tích hợp hệ sinh thái HiTechCloud |
| [`08-operations-security-observability.md`](./08-operations-security-observability.md) | **Vận Hành, Giám Sát & Bảo Mật** | Chương 30, 32, 33 | Logging, Metrics, Cost tracking (Live Pricing Sync), Bảo vệ IP gốc & SafeLine WAF reverse proxy, Phân phối bản build (Linux/macOS/Windows), Kênh cập nhật (`hitechcloud update`), Chiến lược kiểm thử 4 tầng (E2E Self-Healing/Circuit Breaker) & Hardening bảo mật |
| [`09-roadmap-and-delivery.md`](./09-roadmap-and-delivery.md) | **Lộ Trình, Cấu Trúc Codebase & Tickets** | Chương 25, 26, 34, 35, 36, 37, 40, 41 | Cấu trúc Monorepo Rust/TS, Tech stack, Lộ trình 12 tháng (Phase 0 → 6), Tiêu chuẩn Production DoD (100% Ready), Chỉ số thành công, Nguyên tắc team, Khởi tạo repo & 25 Engineering Tickets đầu tiên (WAF + Subdomains `-cli`) |

---

## 💡 Hướng Dẫn Chỉnh Sửa & Cập Nhật Kế Hoạch

1. **Khi thay đổi API Provider hoặc thêm model mới:** Cập nhật file [`02-provider-and-models.md`](./02-provider-and-models.md).
2. **Khi thay đổi cơ chế cấp quyền, tool mới hoặc logic vòng lặp Agent:** Cập nhật file [`03-agent-runtime-and-engine.md`](./03-agent-runtime-and-engine.md).
3. **Khi thiết kế thêm Skill, Plugin hoặc cấu hình MCP:** Cập nhật file [`04-extensibility-skills-plugins-mcp.md`](./04-extensibility-skills-plugins-mcp.md).
4. **Khi thêm lệnh CLI hoặc đổi cú pháp config:** Cập nhật file [`05-cli-ux-and-commands.md`](./05-cli-ux-and-commands.md).
5. **Khi mở rộng tính năng cho Extension trong VS Code:** Cập nhật file [`06-vscode-extension.md`](./06-vscode-extension.md).
6. **Khi điều chỉnh hạ tầng subdomain hoặc tích hợp dịch vụ cloud:** Cập nhật file [`07-infrastructure-and-domains.md`](./07-infrastructure-and-domains.md).
7. **Khi theo dõi tiến độ sprint, phân bổ ticket kỹ thuật:** Cập nhật file [`09-roadmap-and-delivery.md`](./09-roadmap-and-delivery.md).
