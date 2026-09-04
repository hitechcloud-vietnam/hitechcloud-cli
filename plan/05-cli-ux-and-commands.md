# Kế Hoạch 05 — Giao Diện CLI, Lệnh & Cấu Hình (CLI UX & Configuration)

**Module:** CLI Command Map, Interactive Terminal UI, In-session Commands, Project/Global Config, Cheat Sheet  
**Repo Apps/Crates liên quan:** `apps/cli`, `crates/core`  

---

## 1. Bản Đồ Lệnh CLI (`hitechcloud`)

Cú pháp gốc: `hitechcloud [command] [subcommand] [flags]`  
> **Đặc biệt:** Chạy `hitechcloud` không kèm tham số sẽ tự động đưa người dùng vào chế độ **Interactive Agent**.

```
hitechcloud                     # Mở Interactive Coding Agent session
hitechcloud exec "<prompt>"     # Chạy tác vụ một lần (one-shot mode) rồi thoát
hitechcloud init                # Khởi tạo HITECHCLOUD.md và thư mục .hitechcloud/
hitechcloud model [sub]         # Quản lý mô hình: list | use | info
hitechcloud provider [sub]      # Quản lý nhà cung cấp: list | add | remove | test
hitechcloud auth [sub]          # Xác thực tài khoản: login | logout | status
hitechcloud skill [sub]         # Quản lý kỹ năng: list | search | install | remove | update
hitechcloud plugin [sub]        # Quản lý plugin: list | search | install | remove | update
hitechcloud agent [sub]         # Quản lý sub-agent: list | run <name>
hitechcloud mcp [sub]           # Quản lý máy chủ MCP: list | add | remove | test | auth
hitechcloud session [sub]       # Quản lý phiên: list | resume <id> | export | delete
hitechcloud checkpoint [sub]    # Khôi phục trạng thái code: list | restore <id>
hitechcloud config [sub]        # Cấu hình: get | set | edit
hitechcloud usage               # Xem thống kê token & chi phí (--session | --month)
hitechcloud review [--diff]     # Đánh giá thay đổi git diff trước khi commit
hitechcloud ci "<prompt>"       # Chế độ non-interactive tối ưu cho CI/CD pipeline
hitechcloud serve               # Chạy agentd daemon phục vụ VS Code Extension qua IPC
hitechcloud doctor              # Chẩn đoán kết nối mạng, provider, tools, MCP, config
hitechcloud update              # Tự động kiểm tra và nâng cấp phiên bản binary
hitechcloud version             # Xem thông tin phiên bản, commit hash và môi trường
```

---

## 2. Trải Nghiệm Tương Tác Trong Terminal (Interactive UX)

Giao diện Terminal UI (TUI) được thiết kế hiện đại, hỗ trợ màu sắc ANSI 24-bit, thanh trạng thái thời gian thực và biểu tượng chỉ thị rõ ràng:

```
╭──────────────────────────────────────────────────────────╮
│                   HiTechCloud CLI v1.0.0                 │
│   Model: ag/gemini-3.7-flash-high (HiTechCloud Gateway)   │
│   Project: /home/workspace/my-api  | Approval: auto-safe │
│                                                          │
│ > Fix the authentication middleware                      │
│                                                          │
│ ◉ Analyzing repository structure...                      │
│ ◉ Reading src/middleware/auth.ts (lines 1-85)           │
│ ◉ Proposing edit to handle expired JWT tokens            │
│ ◉ Running unit tests: npm test                           │
│                                                          │
│ ✓ All 18 tests passed successfully!                      │
│                                                          │
│ Agent: I have updated the token expiration handler and   │
│ verified with your test suite. Would you like to commit? │
╰──────────────────────────────────────────────────────────╯
```

---

## 3. Lệnh Trong Phiên (In-session Commands)

Trong khi đang ở trong phiên chat interactive của `hitechcloud`, developer có thể nhập các lệnh bắt đầu bằng dấu gạch chéo `/`:

| Lệnh Trong Session | Tác Dụng |
|---|---|
| `/help` | Hiển thị hướng dẫn sử dụng và danh sách lệnh có sẵn |
| `/model [name]` | Xem model hiện tại hoặc chuyển đổi nhanh sang model khác |
| `/provider [name]` | Xem hoặc chuyển đổi nhà cung cấp AI |
| `/compact` | Chủ động nén lịch sử trò chuyện để giải phóng token context |
| `/context` | Hiển thị chi tiết số lượng token đã dùng và các file đang nằm trong context |
| `/skills` | Xem danh sách kỹ năng đang kích hoạt trong phiên |
| `/plugins` | Xem danh sách các plugin đang nạp |
| `/mcp` | Kiểm tra trạng thái các MCP servers đang kết nối |
| `/agents` | Liệt kê hoặc chuyển đổi sub-agent chuyên trách |
| `/session` | Hiển thị thông tin ID phiên hiện tại |
| `/diff` | Xem bản so sánh thay đổi file (git diff) mà agent vừa tạo ra |
| `/test` | Yêu cầu agent chạy lại bộ test của dự án |
| `/undo` | Hoàn tác thay đổi file vừa thực hiện gần nhất |
| `/clear` | Xóa sạch màn hình terminal |
| `/exit` hoặc `Ctrl+C` | Thoát phiên làm việc (lưu trạng thái vào session engine) |

---

## 4. Hệ Thống Cấu Hình: Global & Project Level

### 4.1 Cấu hình toàn cục (Global Configuration): `~/.hitechcloud/`
Áp dụng cho mọi dự án trên máy dev của người dùng:
```
~/.hitechcloud/
├── config.toml           # Cấu hình mặc định (provider, model, approval)
├── credentials/          # Token xác thực, API keys (mã hóa an toàn)
├── sessions/             # Cơ sở dữ liệu SQLite lưu trữ lịch sử phiên
├── cache/                # Cache AST, repository map, metadata
├── skills/               # Thư mục chứa các kỹ năng tự viết của dev
├── plugins/              # Các plugin đã cài đặt từ Marketplace
├── agents/               # Định nghĩa các custom agent cá nhân
├── mcp/                  # Danh sách máy chủ MCP toàn cục
└── logs/                 # Nhật ký thực thi chi tiết
```

### 4.2 Cấu hình dự án (Project Configuration): `HITECHCLOUD.md` & `.hitechcloud/`
Nằm trong thư mục gốc của repository mã nguồn và được commit vào Git để toàn bộ team dùng chung:
- **`HITECHCLOUD.md`**: File chỉ dẫn dự án (Project Instructions) hướng dẫn agent về kiến trúc, coding conventions, cách chạy test, linter và các vùng code cấm đụng đến.
- **`.hitechcloud/config.toml`**: Cấu hình ghi đè dành riêng cho dự án này.

Ví dụ file `.hitechcloud/config.toml`:
```toml
[default]
provider = "hitechcloud"
model = "ag/gemini-3.7-flash-high"
approval = "auto-safe"

[providers.hitechcloud]
type = "openai-compatible"
base_url = "https://api-cli.hitechcloud.vn/v1"
api_key_env = "HITECHCLOUD_API_KEY"

# Cấu hình provider Nube.SH (OpenAI-compatible hoặc Anthropic-compatible)
[providers.nube]
type = "openai-compatible"
base_url = "https://ai.nube-api.com/v1"
api_key_env = "NUBE_API_KEY"

[agent]
max_turns = 100
parallel_tools = true
checkpoint = true

[mcp]
enabled = true
registry_url = "https://mcp-cli.hitechcloud.vn"

# Quản lý ngân sách và Circuit Breaker tự động
[governor]
max_budget_per_task_usd = 1.00
max_tool_repeats = 4
auto_retry_failing_tests = true
max_auto_fix_attempts = 3
```

### 4.3 Các biến môi trường hỗ trợ (Environment Variables):
- `HITECHCLOUD_API_KEY`: API Key kết nối cổng dịch vụ HiTechCloud AI.
- `HITECHCLOUD_BASE_URL`: URL cổng gateway AI tùy chỉnh.
- `NUBE_API_KEY`: API Key dùng chung cho cả OpenAI- và Anthropic-compatible format trên Nube.SH (`ai.nube-api.com`).
- `OPENAI_API_KEY`: API Key kết nối trực tiếp OpenAI.
- `ANTHROPIC_API_KEY`: API Key kết nối trực tiếp Anthropic.
- `HITECHCLOUD_MODEL`: Chỉ định model mặc định khi khởi động (ví dụ: `Nube-Choice`, `DeepSeek-V4-Flash`).
- `HITECHCLOUD_PROVIDER`: Chỉ định provider mặc định khi khởi động (`hitechcloud`, `nube`, `openai`, `anthropic`).
- `HITECHCLOUD_APPROVAL`: Thiết lập chế độ phê duyệt (`suggest`, `auto-safe`, `auto`, `unsafe`).
- `HITECHCLOUD_MAX_BUDGET`: Khóa trần ngân sách cho lệnh hiện tại (ví dụ: `0.50`).
- `HITECHCLOUD_DEBUG`: Bật log chi tiết (`1` hoặc `trace`).

---

## 5. Command Cheat Sheet (Tra Cứu Nhanh & Lệnh Nâng Cao)

```bash
# 1. Khởi động nhanh
hitechcloud                          # Khởi động interactive agent
hitechcloud exec "Fix login bug"     # Chạy tác vụ và thoát ngay
hitechcloud exec "Refactor DB" --max-budget-usd 0.50 # Giới hạn ngân sách tối đa cho task

# 2. Quản lý dự án
hitechcloud init                     # Tạo template chỉ dẫn dự án
hitechcloud memory view              # Xem các ghi nhớ dài hạn về repo và cá nhân
hitechcloud memory clear             # Xóa cache bộ nhớ ngữ cảnh cũ

# 3. Quản lý Model & Provider
hitechcloud model list               # Liệt kê các model sẵn có
hitechcloud model use gpt-5.1        # Đổi model đang dùng
hitechcloud provider list            # Danh sách nhà cung cấp
hitechcloud provider test            # Kiểm tra kết nối mạng tới provider

# 4. Xác thực
hitechcloud auth login               # Đăng nhập tài khoản HiTechCloud
hitechcloud auth status              # Kiểm tra trạng thái phiên đăng nhập

# 5. Mở rộng (Skills & MCP)
hitechcloud skill list               # Xem kỹ năng hiện có
hitechcloud skill install docker     # Cài kỹ năng DevOps Docker
hitechcloud mcp list                 # Danh sách máy chủ MCP
hitechcloud mcp add local stdio ...  # Thêm MCP server chạy local

# 6. Quản lý Phiên làm việc
hitechcloud session list             # Danh sách các phiên cũ
hitechcloud session resume <id>      # Tiếp tục phiên làm việc theo ID

# 7. Kiểm toán & Tiện ích
hitechcloud doctor                   # Quét chẩn đoán sức khỏe hệ thống
hitechcloud usage --month            # Báo cáo chi phí và token trong tháng
hitechcloud update                   # Nâng cấp CLI lên bản mới nhất
```
