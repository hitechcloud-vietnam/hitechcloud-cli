# Kế Hoạch 06 — Kế Hoạch Chi Tiết VS Code Extension (VS Code Extension Full Plan)

**Module:** VS Code Extension Architecture, Agent Daemon (`hitechcloud-agentd`), UI/UX, Editor Integration, Packaging & Publishing  
**Repo Apps/Crates liên quan:** `apps/vscode-extension`, `crates/agentd`, `crates/core`  

---

## 1. Mục Tiêu & Định Vị Của Extension

Extension **HiTechCloud for VS Code** mang toàn bộ sức mạnh của coding agent `hitechcloud` trực tiếp vào môi trường soạn thảo quen thuộc của lập trình viên.

### Nguyên tắc kiến trúc cốt lõi:
- **Dùng chung 100% Core Engine với CLI**: Extension **không tự triển khai lại logic agent** bằng TypeScript, mà kết nối trực tiếp với backend daemon `hitechcloud-agentd` (viết bằng Rust).
- **Đồng bộ hóa trải nghiệm hoàn toàn**:
  - Cùng chung cấu hình mô hình và provider.
  - Cùng chung kỹ năng (Skills), Plugins và cấu hình MCP Servers.
  - Cùng chung phiên làm việc (Session Engine): Developer có thể bắt đầu một prompt trên CLI terminal và tiếp tục review/hoàn thành trên VS Code hoặc ngược lại (`hitechcloud session resume`).
  - Áp dụng thống nhất một chính sách phê duyệt quyền (Permission Engine).

---

## 2. Kiến Trúc Kết Nối (Extension Architecture)

```
┌────────────────────────────────────────────────────────┐
│             VS Code (Extension Host, Node.js)          │
├────────────────────────────────────────────────────────┤
│ Extension Frontend (TypeScript)                         │
│  - Chat / Agent Panel (Webview React UI)               │
│  - Inline Diff / Edit UI (Decorations & Diff View)     │
│  - Command Palette Handlers                            │
│  - Status Bar Controller (Model/Provider Switcher)     │
│  - Quick Fix / Diagnostics Code Action Provider        │
└──────────────────────────┬─────────────────────────────┘
                           │ 
                           │ Giao thức JSON-RPC qua IPC
                           │ (Unix Domain Socket / Named Pipe Windows)
                           ▼
┌────────────────────────────────────────────────────────┐
│                   hitechcloud-agentd                   │
│         (Background Agent Daemon Core - Rust)          │
├────────────────────────────────────────────────────────┤
│ Agent Runtime · Context Engine · Repository Map        │
│ Tool Runtime · Permission Engine · Session Engine      │
│ Provider Abstraction Layer (OpenAI/Anthropic/Local/GW) │
└────────────────────────────────────────────────────────┘
```

### 2.1 Cơ chế Daemon `hitechcloud-agentd`
- Daemon chạy ngầm trên máy cục bộ, quản lý tài nguyên và kết nối tới các LLM providers.
- Giao tiếp với Extension thông qua **JSON-RPC 2.0** trên Unix domain socket (trên Linux/macOS) hoặc Named Pipe (trên Windows), tránh việc mở port mạng TCP cục bộ ra ngoài máy.
- Extension tự động phát hiện và khởi động daemon khi mở VS Code, hoặc kết nối tới daemon có sẵn nếu người dùng đã chạy lệnh `hitechcloud serve`.
- Tự động nhận diện phiên đăng nhập: Nếu developer đã chạy `hitechcloud auth login` ở terminal, extension sẽ tự động dùng credentials tại `~/.hitechcloud/credentials/` mà không bắt đăng nhập lại.

---

## 3. Các Tính Năng Chính Trong VS Code

| Tính Năng | Mô Tả Trải Nghiệm Lập Trình |
|---|---|
| **Chat / Agent Panel** | Webview bên sidebar: Trò chuyện với Agent, theo dõi kế hoạch từng bước, xem log tool call thời gian thực và kết quả kiểm thử. |
| **Inline Agent (`Ctrl/Cmd+K`)** | Bôi đen một đoạn mã bất kỳ và bấm tổ hợp phím để yêu cầu giải thích, tái cấu trúc (refactor), viết unit test hoặc sửa bug ngay tại dòng code. |
| **Diff Review UI** | Mọi chỉnh sửa file mà agent đề xuất sẽ hiển thị trực quan dưới dạng Git Diff view của VS Code. Lập trình viên có thể duyệt hoặc từ chối từng block code (hunk-by-hunk approval). |
| **Context Tự Động** | Tự động lấy file đang mở, vị trí con trỏ, đoạn mã đang bôi đen hoặc các lỗi đang có trong panel Problems làm ngữ cảnh cho agent. |
| **Approval Modal Trong Editor** | Khi agent cần chạy lệnh terminal, cài đặt thư viện hoặc gọi mạng, một popup thông báo xin phép sẽ xuất hiện trực tiếp trong VS Code theo đúng cấp độ approval mode đã cấu hình. |
| **Tree View Quản Lý Extension** | Bảng điều khiển trực quan liệt kê danh sách Skills, Plugins và MCP Servers đang kích hoạt; cho phép bật/tắt nhanh bằng chuột. |
| **Đồng Bộ Phiên Làm Việc** | Cho phép mở danh sách phiên và khôi phục lại bất kỳ session nào từng chạy ở terminal CLI vào ngay trong Chat panel. |
| **Model / Provider Switcher** | Nút bấm trên Status Bar (thanh trạng thái bên dưới) giúp chuyển đổi nhanh giữa Gemini, Claude, GPT hoặc Ollama. |
| **Terminal Bridge** | Nút bấm "Run in Terminal" cho phép chuyển lệnh do agent đề xuất sang Integrated Terminal của VS Code để người dùng tự tay chạy nếu muốn. |
| **Tự Động Chạy Test/Lint** | Sau khi agent chỉnh sửa mã nguồn, extension sẽ tự động kích hoạt bộ test/linter đã định nghĩa trong `HITECHCLOUD.md` và báo cáo kết quả pass/fail vào chat. |
| **Hỗ Trợ Multi-root Workspace** | Hỗ trợ các workspace chứa nhiều dự án độc lập, mỗi thư mục con tự nhận cấu hình `.hitechcloud/` riêng biệt. |
| **Hot Reload Daemon Connection** | Tự động phát hiện và kết nối lại ngay lập tức khi daemon `hitechcloud-agentd` khởi động lại, không làm gián đoạn trạng thái panel chat. |
| **Cost & Token HUD** | Hiển thị lượng token tiêu thụ và chi phí USD thời gian thực trực tiếp trên status bar hoặc góc panel. |

---

## 4. Danh Sách Lệnh Command Palette (`Ctrl/Cmd+Shift+P`)

```
HiTechCloud: Start Agent                         # Mở chat panel agent
HiTechCloud: Ask About Selection                 # Hỏi về đoạn mã đang chọn
HiTechCloud: Explain File                        # Giải thích cấu trúc file hiện tại
HiTechCloud: Fix Diagnostics                     # Tự động sửa các lỗi đỏ trong tab Problems
HiTechCloud: Generate Tests                      # Tự động sinh file kiểm thử cho file hiện tại
HiTechCloud: Review Changes (git diff)           # Kiểm tra chất lượng các thay đổi chưa commit
HiTechCloud: Switch Model                        # Chuyển đổi mô hình AI
HiTechCloud: Switch Provider                     # Chuyển đổi nhà cung cấp
HiTechCloud: Manage Skills                       # Quản lý danh mục kỹ năng
HiTechCloud: Manage Plugins                      # Quản lý các plugin
HiTechCloud: Manage MCP Servers                  # Xem và cấu hình máy chủ MCP
HiTechCloud: Resume Session                      # Mở lại phiên làm việc cũ
HiTechCloud: Open Config                         # Mở nhanh file cấu hình HITECHCLOUD.md
HiTechCloud: Reset Context Memory                # Làm mới ngữ cảnh tạm thời
HiTechCloud: Doctor                              # Kiểm tra sức khỏe kết nối hệ thống
```

---

## 5. Cấu Hình VS Code (`settings.json`)

```json
{
  "hitechcloud.provider": "hitechcloud",
  "hitechcloud.model": "ag/gemini-3.7-flash-high",
  "hitechcloud.approvalMode": "auto-safe",
  "hitechcloud.agentdPath": "auto",
  "hitechcloud.showInlineDiff": true,
  "hitechcloud.autoRunTestsAfterEdit": true,
  "hitechcloud.mcpRegistryUrl": "https://mcp-cli.hitechcloud.vn",
  "hitechcloud.maxBudgetUsdPerTask": 1.0,
  "hitechcloud.autoRetryFailingTests": true,
  "hitechcloud.telemetry.enabled": false
}
```

---

## 6. Chính Sách Bảo Mật Trong Editor

- **Tôn trọng Sandbox của VS Code**: Extension tuân thủ hoàn toàn môi trường Extension Host của VS Code. Mọi thao tác ghi đĩa hoặc chạy process đều được thực hiện qua Permission Engine của Rust Core.
- **Minh bạch mã nguồn trước khi lưu**: Mọi chỉnh sửa mã nguồn đều hiển thị diff rõ ràng; agent không được phép ngầm ghi đè file khi chưa có sự đồng ý của developer (ở các chế độ `suggest` và `auto-safe`).
- **Bảo mật thông tin đăng nhập**: Không bao giờ lưu API key hoặc mật khẩu dạng plaintext trong `settings.json`. Thông tin xác thực luôn được bảo vệ trong OS Keychain hoặc thư mục bảo mật `~/.hitechcloud/credentials/`.
- **Giới hạn Context gửi lên Cloud**: Không bao giờ tự ý quét và gửi toàn bộ ổ cứng lên server. Chỉ những file liên quan trực tiếp đến tác vụ mới được nạp vào context.

---

## 7. Đóng Gói, Phân Phối & Phát Hành (Packaging & Publishing)

- **Công cụ build**: Sử dụng `vsce` (Visual Studio Code Extension Manager) để đóng gói thành file `.vsix`.
- **Các kênh phát hành chính thức**:
  1. **Visual Studio Marketplace**: Kênh phát hành chính thức cho người dùng VS Code toàn cầu.
  2. **Open VSX Registry**: Kênh dành cho các trình soạn thảo mã nguồn mở như VSCodium, Gitpod, Theia.
  3. **Bản VSIX Nội Bộ (Enterprise)**: Cung cấp qua cổng tải riêng `download-cli.hitechcloud.vn` dành cho các doanh nghiệp có hạ tầng mạng cô lập (air-gapped).
- **Tự động hóa Bootstrap CLI Core**:
  - Khi người dùng cài đặt Extension từ Marketplace nhưng trên máy chưa có sẵn binary CLI `hitechcloud`, extension sẽ hiển thị hướng dẫn cài đặt nhanh chỉ với 1 click chuột, tự động tải bản build Rust phù hợp với OS/Architecture vào `~/.hitechcloud/bin/`.
- **Bảo đảm tính tương thích (Compatibility Matrix)**:
  - Khai báo rõ ràng phiên bản tương thích giữa Extension và CLI Core để đảm bảo giao thức IPC không bị xung đột schema.

---

## 8. Lộ Trình Phát Triển VS Code Extension

| Giai Đoạn | Mục Tiêu & Tính Năng |
|---|---|
| **Alpha (Tháng 9)** | Chat panel cơ bản, Inline Diff view, kết nối ổn định với `hitechcloud-agentd` qua local socket. |
| **Beta (Tháng 10)** | Giao diện Approval UI hoàn chỉnh, Sidebar quản lý Skills/Plugins/MCP, đồng bộ session 2 chiều giữa Terminal và VS Code. |
| **GA (Tháng 11)** | Tích hợp tự động Test/Lint, hỗ trợ multi-root workspace, chính thức phát hành lên Visual Studio Marketplace & Open VSX. |
| **Enterprise (Tháng 12)** | Hỗ trợ phân phối Private VSIX, nhận chính sách bảo mật tập trung đẩy từ `enterprise-cli.hitechcloud.vn` (ví dụ: khóa cứng danh sách model được phép dùng). |
