# Kế Hoạch 08 — Vận Hành, Giám Sát & Bảo Mật (Operations, Security & Observability)

**Module:** Observability, Logging, Cost Tracking, Distribution, Auto-update, Testing Strategy & Security Hardening  
**Mục tiêu:** Đảm bảo tính ổn định, giám sát toàn diện, phân phối đa nền tảng và kiểm soát rủi ro an ninh mạng  

---

## 1. Giám Sát, Nhật Ký & Đo Lường Chi Phí (Observability & Cost Tracking)

### 1.1 Hệ thống ghi nhật ký (Logging System)
- **Các mức độ log**: `error`, `warn`, `info`, `debug`, `trace`.
- **Vị trí lưu trữ**: Mặc định ghi file quay vòng (log rotation) tại `~/.hitechcloud/logs/agent.log`.
- **Nguyên tắc an toàn dữ liệu tuyệt đối (Zero Plaintext Secrets)**: Toàn bộ thông tin nhạy cảm (Authorization header, Secret Key, Token, Password) bắt buộc phải bị băm nhỏ hoặc thay thế bằng chuỗi `***REDACTED***` trước khi ghi vào đĩa hoặc hiển thị lên terminal.

### 1.2 Chỉ số đo lường hiệu năng (Key Metrics)
Hệ thống theo dõi các chỉ số vận hành để đánh giá chất lượng trải nghiệm:
- **Thời gian phản hồi token đầu tiên (TTFT - Time to First Token)**: Đo lường tốc độ kết nối và xử lý của provider.
- **Thời gian thực thi công cụ (Tool Execution Latency)**: Đo thời gian chạy lệnh shell, đọc ghi đĩa hoặc truy vấn git.
- **Tỷ lệ thành công tác vụ (Task Completion Rate)**: Đo lường tỷ lệ các phiên đạt được trạng thái `COMPLETED` mà không gặp lỗi lặp vô hạn.
- **Tần suất lỗi của Provider/Model**: Thống kê mã lỗi `429` (Rate limit) hoặc `5xx` để tự động tối ưu chuỗi fallback.

### 1.3 Theo dõi ngân sách & Token (Cost & Usage Tracking)
Mỗi tương tác đều được tính toán lượng token input, output, cache-read và quy đổi chi phí theo bảng giá thực tế:
- Tự động đồng bộ biểu phí thời gian thực từ các nhà cung cấp hỗ trợ Public Pricing API (ví dụ: Nube.SH `GET https://ai.nube-api.com/v1/models/pricing`).
- Tính toán chính xác các mức giá:
  - `input_cost_per_million_tokens` (USD/1M tokens)
  - `output_cost_per_million_tokens` (USD/1M tokens)
  - `cache_read_cost_per_million_tokens` (giá tiết kiệm khi hit cache)
- Báo cáo chi phí trực quan:

```bash
hitechcloud usage               # Xem tổng quan chi phí
hitechcloud usage --session     # Xem chi tiết phiên làm việc hiện tại
hitechcloud usage --month       # Báo cáo tổng lượng token và chi phí trong tháng
```

### 1.4 Chính sách dữ liệu chẩn đoán (Telemetry Policy)
- **Mặc định tắt hoàn toàn (Opt-in only)**: Không tự ý gửi bất kỳ dữ liệu nào ra ngoài nếu developer chưa đồng ý.
- **Chỉ thu thập dữ liệu kỹ thuật tổng hợp**: Khi bật (`telemetry.enabled = true`), client chỉ gửi các thông tin ẩn danh: Phiên bản OS, phiên bản CLI, mã lỗi hệ thống và thời gian thực thi trung bình về `telemetry-cli.hitechcloud.vn` (được bảo vệ qua SafeLine WAF). Tuyệt đối không bao gồm code, file path hay prompt của người dùng.

---

## 2. Đóng Gói Phân Phối & Tự Động Cập Nhật (Distribution & Auto-update)

### 2.1 Ma trận đóng gói cài đặt đa nền tảng
Hệ sinh thái `hitechcloud` cung cấp các kênh cài đặt tiêu chuẩn cho từng hệ điều hành:

| Hệ Điều Hành | Định Dạng Gói Cài Đặt | Kênh Phân Phối / Quản Lý Gói |
|---|---|---|
| **Linux (Ubuntu, Debian)** | `.deb`, Standalone binary `.tar.gz` | `apt repository`, GitHub Releases, `curl -fsSL https://cli.hitechcloud.vn/install.sh \| sh` |
| **Linux (RHEL, Fedora, Rocky)** | `.rpm`, Standalone binary | `yum/dnf repository`, `cli.hitechcloud.vn` |
| **macOS (Intel & Apple Silicon)** | Homebrew Formula, `.pkg`, Universal binary | `brew install hitechcloud/tap/hitechcloud`, Direct download |
| **Windows (x64, ARM64)** | Winget package, Portable `.exe`, Installer `.msi` | `winget install HiTechCloud.CLI`, PowerShell installer script |
| **Containerized Environments** | Docker Image | `docker.io/hitechcloud/hitechcloud-cli:latest` |
| **VS Code / Editor** | VSIX Extension package | Visual Studio Marketplace, Open VSX, `download-cli.hitechcloud.vn` |

### 2.2 Cơ chế tự động cập nhật (`hitechcloud update`)
- CLI định kỳ kiểm tra phiên bản mới từ máy chủ `update-cli.hitechcloud.vn` (đi qua SafeLine WAF).
- Hỗ trợ 3 kênh phát hành:
  - `stable`: Dành cho môi trường làm việc hàng ngày, tính ổn định cao nhất.
  - `beta`: Dành cho các tính năng mới cần lấy phản hồi từ cộng đồng.
  - `nightly`: Bản build tự động hàng ngày từ nhánh mã nguồn chính.
- Tự động kiểm tra tính toàn vẹn thông qua chữ ký số SHA-256 trước khi ghi đè binary cũ.
- Đối với môi trường doanh nghiệp, quản trị viên có thể vô hiệu hóa tính năng auto-update qua policy để cố định phiên bản đã được phê duyệt.

---

## 3. Chiến Lược Kiểm Thử (Testing Strategy)

Hệ thống được bảo đảm chất lượng thông qua 4 cấp độ kiểm thử tự động trong pipeline CI/CD:

```
┌────────────────────────────────────────────────────────┐
│                   End-to-End (E2E)                     │  ← Luồng người dùng hoàn chỉnh
├────────────────────────────────────────────────────────┤
│                 Integration Tests                      │  ← Tương tác giữa các Crates & Tool
├────────────────────────────────────────────────────────┤
│                    Unit Tests                          │  ← Logic thuật toán, Parser, Regex
├────────────────────────────────────────────────────────┤
│               Security & Hardening Tests               │  ← Kiểm tra lỗ hổng bảo mật
└────────────────────────────────────────────────────────┘
```

### 3.1 Unit Testing:
- Kiểm thử các adapter tương thích: OpenAI-compatible, Anthropic-compatible JSON schema.
- Kiểm thử parser cấu hình: `TOML`, `YAML`, frontmatter trong `SKILL.md`.
- Kiểm thử bộ lọc quyền: Rule matching, Path normalization, Secret redaction.
- Kiểm thử logic Session compaction và thuật toán phân bổ token.

### 3.2 Integration Testing:
- Kiểm thử giao tiếp IPC giữa `hitechcloud-agentd` (Rust) và Extension (TypeScript) qua local socket.
- Kiểm thử kết nối client MCP tới máy chủ mẫu (`stdio` & `http`).
- Kiểm thử tương tác Git thực tế: Branch creation, commit, revert, diff generation trên repo giả lập.

### 3.3 End-to-End (E2E) Testing:
- Tự động dựng môi trường sandbox kiểm tra kịch bản hoàn chỉnh:
  *Khởi tạo project (`init`) → Phân tích lỗi test → Agent tự động sửa file → Chạy lại test suite → Xác nhận pass.*
- Kiểm thử cơ chế Self-Healing: Tạo tình huống test lỗi nhân tạo và xác minh agent tự sửa được trong tối đa 3 lần thử.
- Kiểm thử cơ chế Circuit Breaker: Giả lập tool lỗi lặp vô hạn và xác nhận agent tự động ngắt sau 4 lần lặp.
- Kiểm thử cơ chế Crash Recovery: Sử dụng SIGKILL ngắt tiến trình giữa chừng và chạy lại `hitechcloud session resume` để xác nhận toàn vẹn dữ liệu.

### 3.4 Quy Chuẩn Commit & Đồng Hành Cùng Agent (Co-Authored-By Policy):
- Mọi commit mã nguồn vào hệ thống (thông qua CLI Git Tool hoặc dev thủ công) bắt buộc gắn kèm metadata đồng tác giả:
  ```gitcommit
  Co-Authored-By: HiTechCloud Agents <agent@hitechcloud.vn>
  ```
- Được tự động kích hoạt thông qua git hook `.github/hooks/prepare-commit-msg` và commit template `.github/git-commit-template.txt`.

---

## 4. Kiểm Toán Bảo Mật Chuyên Sâu (Security Hardening)

Vì AI Coding Agent có khả năng can thiệp trực tiếp vào file và terminal, các bài kiểm thử bảo mật chống xâm nhập được ưu tiên hàng đầu:

| Kịch Bản Tấn Công | Giải Pháp Phòng Ngự Của HiTechCloud CLI |
|---|---|
| **Command Injection qua Shell** | Sử dụng API gọi tiến trình có tham số tách biệt (`std::process::Command` trong Rust), không truyền chuỗi thô qua `/bin/sh -c` trừ khi ở chế độ shell có kiểm duyệt; áp dụng whitelist ký tự nguy hiểm. |
| **Path Traversal (`../../`)** | Mọi đường dẫn file trước khi đọc/ghi đều phải chuẩn hóa (`canonicalize`) và kiểm tra nghiêm ngặt xem có nằm trong root directory của workspace hay không. |
| **Prompt Injection qua Dữ liệu ngoài** | Toàn bộ nội dung đọc từ README, Issue bên ngoài, kết quả tìm kiếm Web hay output của MCP server đều được đóng gói trong thẻ phân cách an toàn (delimited untrusted tags) và có system instruction cảnh báo mô hình không thực hiện các mệnh lệnh bên trong đó. |
| **Rò rỉ Secret & API Keys** | Tích hợp bộ quét mẫu Regular Expression theo chuẩn của HiTechSecure, tự động bắt các chuỗi mã bí mật (Stripe, AWS, GitHub Token, Private Key) và xóa bỏ trước khi đẩy ra mạng. |
| **Mã độc từ Plugin & Skill** | Toàn bộ package trên Marketplace phải được ký số bởi tác giả đã định danh; hệ thống kiểm tra chữ ký điện tử trước khi nạp vào runtime; chạy scan SAST tĩnh trước khi duyệt lên chợ ứng dụng. |
| **Lộ IP máy chủ gốc (Origin IP Exposure)** | Cấu hình tường lửa máy chủ gốc CHỈ chấp nhận lưu lượng từ IP của SafeLine WAF (`vcwaf.hitechcloud.vn` - ví dụ `103.249.117.146`), DROP toàn bộ kết nối thẳng; DNS công khai chỉ trỏ về WAF. |
