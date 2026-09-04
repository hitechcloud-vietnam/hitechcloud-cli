# Kế Hoạch 09 — Lộ Trình, Cấu Trúc Codebase & Tickets (Roadmap & Engineering Delivery)

**Module:** Repository Structure, Technology Stack, 12-Month Roadmap, MVP DoD, Metrics, Team Principles, Bootstrap & First Tickets  
**Mục tiêu:** Hướng dẫn triển khai kỹ thuật thực tế cho đội ngũ kỹ sư phát triển  

---

## 1. Cấu Trúc Monorepo (`hitechcloud-cli`)

Dự án được tổ chức theo mô hình Monorepo kết hợp giữa Rust Workspace (cho core và CLI) và Node.js/TypeScript (cho VS Code Extension):

```
hitechcloud-cli/
├── apps/
│   ├── cli/                      # Binary chính của dòng lệnh `hitechcloud`
│   └── vscode-extension/         # Ứng dụng Extension cho Visual Studio Code (TypeScript)
├── crates/
│   ├── core/                     # Các kiểu dữ liệu, config chung và tiện ích cơ sở
│   ├── agent-runtime/            # Vòng lặp ReAct, bộ lập kế hoạch (Planner) và điều phối
│   ├── provider-sdk/             # Provider trait, kiểu dữ liệu request/response chuẩn
│   ├── provider-openai/          # Native adapter cho OpenAI API
│   ├── provider-anthropic/       # Native adapter cho Anthropic Claude API
│   ├── provider-compatible/      # Adapter chung cho các cổng OpenAI/Anthropic compatible
│   ├── context/                  # Quản lý Context, Compact history và AST Repository Map
│   ├── tools/                    # Tập hợp các công cụ: File, Shell, Git, Dev, Network
│   ├── permissions/              # Permission Engine, kiểm duyệt lệnh và lọc secret
│   ├── sessions/                 # Quản lý phiên làm việc, lưu trữ SQLite và checkpoint
│   ├── skills/                   # Bộ nạp kỹ năng (Skill loader), parser frontmatter Markdown
│   ├── plugins/                  # Quản lý plugin, manifest schema và nạp mở rộng
│   ├── mcp/                      # MCP Client (kết nối stdio, http, sse tới mcp-cli)
│   ├── agents/                   # Quản lý Sub-agents chuyên trách và chạy song song
│   ├── agentd/                   # Daemon chạy nền phục vụ JSON-RPC cho VS Code Extension
│   └── marketplace/              # Client tương tác với registry-cli.hitechcloud.vn
├── sdk/
│   ├── typescript/               # SDK cho tác giả viết plugin bằng TypeScript
│   └── python/                   # SDK cho tác giả viết plugin bằng Python
├── plan/                         # Toàn bộ tài liệu quy hoạch kiến trúc và sub-plans
├── registry/                     # Mã nguồn máy chủ Registry metadata
├── docs/                         # Tài liệu kỹ thuật chi tiết
├── examples/                     # Các dự án mẫu và skills mẫu
└── tests/                        # Bộ kiểm thử End-to-End tự động
```

---

## 2. Công Nghệ Sử Dụng (Technology Stack)

- **Lõi Hệ Thống (Core Engine) & CLI**: Ngôn ngữ **Rust** — Đảm bảo tốc độ khởi động tức thì (< 20ms), an toàn bộ nhớ tuyệt đối, quản lý luồng song song tốt và khả năng đóng gói thành file binary tĩnh chạy đa nền tảng.
- **Agent Daemon (`hitechcloud-agentd`)**: Cùng codebase Rust, chạy tiến trình nền lắng nghe kết nối JSON-RPC qua Unix domain socket / Named pipe.
- **VS Code Extension**: **TypeScript**, sử dụng chính thức bộ VS Code Extension API kết hợp với React UI trên Webview cho giao diện Chat Panel trực quan.
- **Cơ sở dữ liệu cục bộ**: **SQLite** nhúng gọn nhẹ thông qua crate `rusqlite` để lưu trữ dữ liệu Session, chỉ mục Repo Map và lịch sử thao tác.
- **Định dạng cấu hình**: Sử dụng `TOML` cho cấu hình hệ thống, `YAML`/`JSON` cho schema plugin và `Markdown` cho tài liệu kỹ năng (Skills).

---

## 3. Lộ Trình Phát Triển 12 Tháng (12-Month Roadmap)

```
Tháng 1-2      Tháng 3-4      Tháng 5-6      Tháng 7-8      Tháng 9-10     Tháng 11-12
[Phase 1] ───> [Phase 2] ───> [Phase 3] ───> [Phase 4] ───> [Phase 5] ───> [Phase 6]
 MVP CLI        Session/       Skills/MCP     Plugins/       VS Code Ext    Enterprise/
 Basic Agent    Approval       mcp-cli.vn     Marketplace    Beta           GA Release
```

### Chi tiết từng giai đoạn:
- **Phase 0 (2 tuần đầu)**: Thiết lập kiến trúc monorepo, định nghĩa Provider Trait, chốt quy hoạch bảo mật, xây dựng khung kết nối hạ tầng domain ban đầu (`cli.`, `mcp-cli.`, `auth.`).
- **Phase 1 (Tháng 1 – 2) — CLI MVP**:
  - Phát hành bản binary dòng lệnh `hitechcloud`.
  - Tương thích tốt với các mô hình OpenAI-compatible và Anthropic-compatible.
  - Tích hợp sẵn profile provider **Nube.SH** (`ai.nube-api.com`) hỗ trợ cả 2 chuẩn API và tự động fetch live pricing (`/v1/models/pricing`).
  - Hỗ trợ đầy đủ bộ công cụ: Đọc/sửa file, chạy terminal, kiểm tra git diff.
  - Hỗ trợ streaming token thời gian thực.
- **Phase 2 (Tháng 3 – 4) — Quản lý Ngữ Cảnh & An Toàn**:
  - Hoàn thiện Session Engine (lưu và resume phiên).
  - Ra mắt bộ máy cấp quyền (Permission Engine) với chế độ `auto-safe`.
  - Hỗ trợ tạo Checkpoint rollback mã nguồn.
  - Hỗ trợ tự động fallback khi model gặp sự cố.
- **Phase 3 (Tháng 5 – 6) — Mở Rộng Hệ Sinh Thái & MCP**:
  - Tích hợp bộ nạp Skills và phát hành các built-in skill packs (DevOps, Laravel, React, Docker).
  - Ra mắt chính thức cổng `mcp-cli.hitechcloud.vn`.
  - Hỗ trợ kết nối MCP Server qua `stdio` và `http`.
- **Phase 4 (Tháng 7 – 8) — Nền Tảng Marketplace**:
  - Hoàn thiện Plugin SDK cho cộng đồng.
  - Ra mắt cổng Registry (`registry-cli.hitechcloud.vn`) và giao diện Web Marketplace (`marketplace-cli.hitechcloud.vn`).
  - Tích hợp cơ chế xác thực chữ ký số và quét bảo mật gói cài đặt.
- **Phase 5 (Tháng 9 – 10) — VS Code Extension (Alpha → Beta)**:
  - Hoàn thiện daemon `hitechcloud-agentd`.
  - Phát hành bản thử nghiệm VS Code Extension: Chat panel, inline diff review, model switcher.
  - Tích hợp tính năng Web Search / Research mode.
- **Phase 6 (Tháng 11 – 12) — Enterprise & Phát Hành Toàn Diện**:
  - Ra mắt bảng điều khiển doanh nghiệp `enterprise-cli.hitechcloud.vn`.
  - Đưa VS Code Extension lên phiên bản General Availability (GA) trên Visual Studio Marketplace.
  - Tối ưu hóa phân phối bản cài đa nền tảng và ra mắt cộng đồng lập trình viên.

---

## 4. Tiêu Chuẩn Hoàn Thành Cho Bản Sản Phẩm (Production Definition of Done - 100% Ready)

Một phiên bản được coi là đạt chuẩn Production thương mại khi thỏa mãn đầy đủ các điều kiện khắt khe sau:
- [ ] Cài đặt và thực thi trơn tru lệnh `hitechcloud` trên cả 3 hệ điều hành: Linux, macOS và Windows.
- [ ] Kết nối và xác thực thành công ít nhất 1 nhà cung cấp OpenAI-compatible, 1 nhà cung cấp Anthropic-compatible và cổng tích hợp Nube.SH (`ai.nube-api.com`).
- [ ] Agent tự động đọc hiểu cấu trúc thư mục, chỉnh sửa chính xác các file mã nguồn và chạy lệnh kiểm thử thành công.
- [ ] Cơ chế Self-Healing tự động sửa lỗi biên dịch/linter tối đa 3 lần lặp thành công.
- [ ] Circuit Breaker ngắt lặp vô hạn sau 4 lần gọi lặp tool tương đương.
- [ ] Chế độ phê duyệt an toàn (`--approval auto-safe`) chặn đứng các lệnh nguy hại và xin phép người dùng hợp lệ.
- [ ] Tính năng Crash Recovery khôi phục chính xác trạng thái phiên sau khi bị ngắt đột ngột (WAL Mode).
- [ ] Nạp thành công các kỹ năng từ thư mục `.hitechcloud/skills/` qua lệnh `/skills`.
- [ ] Kết nối thành công tới máy chủ MCP thông qua cổng `mcp-cli.hitechcloud.vn`.
- [ ] Cho phép chuyển đổi linh hoạt mô hình/provider ngay trong phiên làm việc mà không cần khởi động lại CLI.
- [ ] Module đo lường chi phí (Cost Governor) khóa trần ngân sách chính xác theo flag `--max-budget-usd`.
- [ ] Cụm SafeLine WAF (`vcwaf.hitechcloud.vn` - 103.249.117.146) đã kích hoạt, Origin Servers đã whitelist IP WAF 100%, không bị lộ IP thật.
- [ ] Toàn bộ dải subdomain `-cli` (`cli.`, `mcp-cli.`, `api-cli.`, `auth-cli.`, `docs-cli.`, `registry-cli.`) phân giải trơn tru.
- [ ] Hệ thống ghi log chi tiết, tự động che mờ secrets (Zero-Trust Redaction), có cờ `--debug`.
- [ ] Bộ kiểm thử Unit test, Integration test và Security test đạt tỷ lệ pass 100%.

---

## 5. Chỉ Số Thành Công Của Dự Án (Success Metrics)

- **Trải nghiệm lập trình viên (Developer Experience)**:
  - Thời gian từ lúc cài đặt đến khi hoàn thành task đầu tiên (Time-to-first-task): `< 5 phút`.
  - Tỷ lệ hoàn thành tác vụ sửa code tự động thành công ngay lần đầu: `> 80%`.
- **Hiệu năng kỹ thuật**:
  - Thời gian khởi động CLI: `< 30ms`.
  - Độ trễ nạp token đầu tiên (TTFT): Phụ thuộc vào model nhưng overhead từ CLI `< 50ms`.
  - Dung lượng RAM tiêu thụ ở trạng thái chờ: `< 45MB`.
- **Phát triển hệ sinh thái**:
  - Đạt trên 50+ kỹ năng và 20+ plugins cộng đồng sau 6 tháng ra mắt.
  - Số lượt cài đặt VS Code Extension tăng trưởng ổn định.

---

## 6. Đội Ngũ Nhân Sự & Nguyên Tắc Phát Triển

### 6.1 Cơ cấu đội ngũ tối thiểu cho giai đoạn MVP:
- **1 Lead Software Architect**: Định hình kiến trúc tổng thể, thiết kế interface các crate, kiểm soát chất lượng code.
- **2 Rust Core / CLI Engineers**: Chuyên trách phát triển các crate lõi, vòng lặp Agent, hệ thống công cụ và bảo mật.
- **1–2 Agent / Prompt / LLM Engineers**: Tinh chỉnh prompt hệ thống, tối ưu ngữ cảnh, xây dựng bộ kỹ năng mẫu (Skill packs).
- **1 DevOps & Security Engineer**: Thiết lập CI/CD pipeline, hạ tầng domain, chứng chỉ SSL, cổng bảo mật và đóng gói bản build.
- *(Bổ sung thêm 1 Frontend/TypeScript Engineer từ Phase 5 để tập trung toàn lực cho VS Code Extension).*

### 6.2 Nguyên tắc làm việc bất di bất dịch:
1. **Provider-agnostic First**: Tuyệt đối không viết code phụ thuộc cứng vào bất kỳ một nhà cung cấp LLM cụ thể nào.
2. **Security by Default**: Mọi tính năng mới mặc định phải được khóa an toàn trước khi có sự chấp thuận của người dùng.
3. **Local-First**: Ưu tiên xử lý và lưu trữ dữ liệu trên máy tính của developer; không phụ thuộc vào cloud nếu không bắt buộc.
4. **Extension là Công Dân Hạng Nhất**: Luôn bảo đảm mọi tính năng trên CLI đều có thể tương tác trơn tru trên VS Code Extension.
5. **Khả năng quan sát toàn diện (Everything Observable)**: Mọi thao tác lỗi đều phải có mã lỗi rõ ràng và thông điệp hướng dẫn khắc phục cụ thể.

---

## 7. Khởi Tạo Dự Án Mẫu (Appendix B — Repository Bootstrap)

```bash
# Khởi tạo workspace
mkdir -p hitechcloud-cli && cd hitechcloud-cli
git init
cargo init --name hitechcloud-cli

# Tạo cấu trúc monorepo phân tầng
mkdir -p \
  apps/cli apps/vscode-extension \
  crates/core crates/agent-runtime crates/provider-sdk \
  crates/provider-openai crates/provider-anthropic crates/provider-compatible \
  crates/context crates/tools crates/permissions crates/sessions \
  crates/skills crates/plugins crates/mcp crates/agents crates/agentd \
  crates/marketplace docs examples tests
```

### Các mục tiêu kiểm tra của Milestone 0:
```
├── Binary dòng lệnh `hitechcloud` biên dịch thành công
├── File cấu hình `~/.hitechcloud/config.toml` tự động sinh và nạp đúng
├── Interface `Provider` trait được định nghĩa hoàn chỉnh
├── Adapter OpenAI-compatible gọi và stream phản hồi thành công
├── Adapter Anthropic-compatible gọi và xử lý tool-use thành công
├── Khung tương tác công cụ (Tool interface) đọc ghi được file giả lập
├── Vòng lặp Agent cơ bản (ReAct loop) chạy được 1 vòng suy nghĩ
└── Daemon `agentd` phản hồi được gói tin "ping" JSON-RPC đầu tiên qua local socket
```

---

## 8. Danh Sách 25 Tickets Kỹ Thuật Đầu Tiên (Appendix C — Engineering Tickets)

```
HTC-001  Khởi tạo Rust workspace (Cấu hình Cargo.toml gốc, apps/cli và apps/vscode-extension)
HTC-002  Định nghĩa Provider Trait và chuẩn hóa kiểu dữ liệu Request/Response trong crates/provider-sdk
HTC-003  Xây dựng adapter tương thích OpenAI-compatible trong crates/provider-openai (hỗ trợ Nube.SH OpenAI endpoint)
HTC-004  Xây dựng adapter tương thích Anthropic Claude Messages API trong crates/provider-anthropic (hỗ trợ Nube.SH Anthropic endpoint)
HTC-005  Xây dựng hệ thống stream sự kiện phản hồi (SSE event stream model)
HTC-006  Triển khai bộ công cụ thao tác Filesystem (read, write, edit, search) trong crates/tools
HTC-007  Triển khai công cụ thực thi lệnh Shell có kiểm soát timeout và background process
HTC-008  Xây dựng lõi phân quyền (Permission Engine) và bộ lọc che giấu chuỗi bí mật (Secret Redactor)
HTC-009  Triển khai vòng lặp suy nghĩ và thực thi cơ bản của Agent (Agent ReAct Loop)
HTC-010  Xây dựng giao diện dòng lệnh tương tác trực quan (TUI) cho binary `hitechcloud`
HTC-011  Xây dựng bộ công cụ phân tích trạng thái và thao tác với Git repository
HTC-012  Xây dựng cơ chế lưu trữ lịch sử phiên làm việc vào SQLite trong crates/sessions
HTC-013  Triển khai tính năng khôi phục phiên làm việc dở dang (`hitechcloud session resume`)
HTC-014  Xây dựng thuật toán tóm tắt và nén ngữ cảnh (Context Compaction) khi gần đầy token
HTC-015  Xây dựng Model Registry định nghĩa năng lực mô hình và tích hợp module đồng bộ giá động từ Nube.SH Pricing API (`/v1/models/pricing`)
HTC-016  Triển khai cơ chế tự động Fallback linh hoạt giữa các nhà cung cấp AI khi có lỗi
HTC-017  Xây dựng bộ nạp kỹ năng (Skills loader) phân tích cú pháp frontmatter Markdown
HTC-018  Xây dựng MCP Client kết nối giao tiếp với các máy chủ trên mcp-cli.hitechcloud.vn
HTC-019  Định nghĩa schema manifest JSON cho hệ thống Plugin của bên thứ ba
HTC-020  Xây dựng nguyên mẫu giao tiếp với Package Registry (registry-cli.hitechcloud.vn)
HTC-021  Xây dựng bộ khung Daemon `agentd` lắng nghe giao tiếp JSON-RPC qua Unix Domain Socket
HTC-022  Khởi tạo dự án VS Code Extension, kết nối tới `agentd` và hiển thị Chat Panel cơ bản
HTC-023  Xây dựng giao diện hiển thị và phê duyệt từng khối code thay đổi (Inline Diff Review UI)
HTC-024  Tích hợp cửa sổ xác nhận quyền hạn (Approval modal) trên VS Code đồng bộ với Permission Engine
HTC-025  Thiết lập SafeLine WAF (vcwaf.hitechcloud.vn) bảo vệ IP gốc và khởi tạo các subdomain -cli: cli., mcp-cli., docs-cli., auth-cli., api-cli., registry-cli.
```
