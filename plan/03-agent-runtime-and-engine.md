# Kế Hoạch 03 — Agent Runtime & Lõi Động Cơ (Agent Runtime & Core Engine)

**Module:** Agent Lifecycle, Built-in Tools, Permission & Security, Context Engine, Sessions & Web Mode  
**Repo Crates liên quan:** `crates/agent-runtime`, `crates/tools`, `crates/permissions`, `crates/context`, `crates/sessions`  

---

## 1. Agent Runtime & Vòng Lặp Điều Phối (Agent Lifecycle)

Agent Runtime hoạt động dựa trên mô hình tự động hóa ReAct (Reasoning + Acting) có kiểm soát:

```
Understand (Hiểu yêu cầu) 
  → Plan (Lập kế hoạch hành động) 
    → Select Tools (Chọn công cụ phù hợp) 
      → Execute (Thực thi an toàn) 
        → Observe (Quan sát kết quả trả về) 
          → Verify (Kiểm thử & xác minh tính đúng đắn) 
            → Continue / Finish (Lặp tiếp hoặc hoàn thành)
```

### Các trạng thái của Agent (Agent States):
- `IDLE`: Chờ nhận lệnh mới từ developer.
- `THINKING`: Phân tích ngữ cảnh và truy vấn LLM.
- `PLANNING`: Xây dựng checklist các bước thực hiện.
- `EXECUTING`: Đang chạy tool (đọc file, chạy terminal, gọi API).
- `WAITING_APPROVAL`: Tạm dừng chờ developer phê duyệt hành động nhạy cảm.
- `VERIFYING`: Kiểm tra lại kết quả (chạy test, build, lint).
- `COMPACTING`: Nén ngữ cảnh hội thoại khi token sắp đầy.
- `COMPLETED`: Hoàn thành tác vụ thành công.
- `FAILED`: Thất bại do lỗi không thể khắc phục.
- `CANCELLED`: Người dùng hủy bỏ tiến trình (`Ctrl+C`).

---

## 2. Hệ Thống Công Cụ Tích Hợp (Built-in Tools)

Các công cụ gốc được biên dịch trực tiếp vào Rust core để tối ưu tốc độ và an toàn bộ nhớ:

| Nhóm Công Cụ | Danh Sách Tool | Mô Tả Chức Năng |
|---|---|---|
| **Filesystem** | `read_file`, `write_file`, `edit_file`, `delete_file`, `move_file`, `list_directory`, `search_files`, `grep` | Đọc/ghi file chính xác theo chunk hoặc replace chuỗi, quét cây thư mục, tìm kiếm regex tốc độ cao. |
| **Shell & Process** | `shell`, `exec`, `process_list`, `process_kill` | Chạy lệnh Bash/Zsh/PowerShell, quản lý process nền (background daemon), xử lý timeout và tín hiệu dừng. |
| **Git Integration** | `git_status`, `git_diff`, `git_log`, `git_branch`, `git_checkout`, `git_commit`, `git_restore` | Tự động phân tích nhánh hiện tại, staging diff, commit có thông điệp chuẩn và rollback an toàn. |
| **Dev & CI** | `test`, `lint`, `format`, `build`, `package_manager` | Gọi các trình quản lý gói (`npm`, `cargo`, `composer`, `pip`, `go`), chạy linter và kiểm thử tự động. |
| **Network** | `http_get`, `http_request`, `download` | Gọi HTTP request phục vụ tải tài liệu hoặc kiểm tra endpoint (bắt buộc qua Permission Engine). |

---

## 3. Permission Engine & Mô Hình Bảo Mật (Security Model)

### 3.1 Phân loại quyền hạn (Permission Granularity)
Hệ thống quản lý quyền truy cập theo từng danh mục cụ thể:
- `filesystem.read`: Được phép đọc nội dung file trong repo.
- `filesystem.write`: Được phép tạo mới hoặc sửa nội dung file.
- `filesystem.delete`: Được phép xóa file hoặc thư mục (rủi ro cao).
- `shell.execute`: Chạy lệnh trong terminal.
- `process.execute`: Khởi động process mới hoặc quản lý daemon.
- `network.connect`: Tạo kết nối mạng ra bên ngoài.
- `git.write`: Tạo commit, chuyển nhánh hoặc push lên remote.
- `secrets.read`: Quyền truy cập các file cấu hình nhạy cảm (`.env`, private keys).
- `package.install`: Cài đặt package phụ thuộc mới vào dự án.

### 3.2 Các cấp độ phê duyệt (Approval Levels)
Developer có thể linh hoạt chuyển đổi chế độ kiểm soát:
- `suggest`: Agent chỉ đề xuất lệnh và chỉnh sửa; người dùng duyệt từng hành động.
- `auto-safe` *(mặc định)*: Tự động cho phép các hành vi an toàn (đọc file, search, `git diff`, chạy unit test). Các thao tác can thiệp hệ thống như `rm -rf`, `git reset --hard`, gửi request lạ ra ngoài bắt buộc phải hỏi người dùng.
- `auto`: Tự động duyệt các lệnh thông thường trong workspace dự án.
- `unsafe`: Tự động duyệt tất cả (chỉ khuyến nghị chạy trong container sandbox cô lập).

```bash
hitechcloud --approval auto-safe
```

### 3.3 Nguyên tắc bảo mật nền tảng:
- **Nguyên tắc quyền tối thiểu (Least Privilege)**: Không cấp thừa quyền so với tác vụ hiện tại.
- **Che giấu bí mật (Secret Redaction)**: Tự động quét và che mờ (`***REDACTED***`) các chuỗi nhạy cảm trước khi gửi lên LLM hoặc ghi ra log: API keys, tokens, mật khẩu, JWT, khóa SSH/GPG.
- **Cách ly Workspace (Project Isolation)**: Mặc định ngăn chặn agent truy cập các đường dẫn ngoài thư mục gốc dự án (ngăn Path Traversal `../../etc/passwd`).
- **Phòng thủ Prompt Injection**: Mọi dữ liệu đọc từ file mã nguồn bên thứ ba, output terminal, nội dung web hoặc issue bên ngoài đều được đánh dấu là *untrusted data* và không được phép ghi đè (override) system security policy.

---

## 4. Context Engine & Repository Intelligence

### 4.1 Chiến lược nạp ngữ cảnh (Context Priority)
Hệ thống sắp xếp độ ưu tiên ngữ cảnh đưa vào model:
```
System Instructions > Security Policy > Project Policy (HITECHCLOUD.md) > User Prompt > Active Skill > Tool Output
```

### 4.2 Repository Intelligence (Lập bản đồ dự án)
- **Tự động lập chỉ mục (Indexing)**: Quét cấu trúc repo, phát hiện ngôn ngữ, frameworks, config files (`package.json`, `Cargo.toml`, `composer.json`,...).
- **AST & Symbol Extraction**: Phân tích quan hệ giữa functions, classes, imports, routes để agent hiểu ngữ cảnh mà không cần nạp toàn bộ file vào prompt.
- **Repository Map**: Tạo sơ đồ kiến trúc tóm tắt, giúp agent định vị nhanh file cần chỉnh sửa.
- **Compaction Strategy**: Khi kích thước ngữ cảnh đạt ngưỡng cảnh báo (80% context window), Context Engine tự động tóm tắt các tool output cũ, giữ lại các thông tin quan trọng nhất để duy trì luồng tư duy liền mạch.

---

## 5. Session Engine & Checkpoints

### 5.1 Quản lý Session
Mỗi phiên làm việc được định danh bằng UUID và lưu trữ trạng thái bền vững trong SQLite cục bộ:

```bash
hitechcloud session list          # Xem danh sách phiên gần đây
hitechcloud session resume <id>   # Tiếp tục phiên làm việc dở dang
hitechcloud session export <id>   # Xuất session ra file Markdown hoặc JSON
hitechcloud session delete <id>   # Xóa dữ liệu phiên cũ
```

Dữ liệu lưu trữ gồm: Message history, Tool calls, File modifications, Terminal logs, Token usage & Chi phí tính toán.

### 5.2 Cơ chế Checkpoint & Crash Recovery (Rollback an toàn)
Trước khi thực hiện các thay đổi lớn hoặc nguy cơ cao trên nhiều file, Session Engine tự động tạo checkpoint cục bộ (sử dụng git diff hoặc stash snapshot ngầm):

```bash
hitechcloud checkpoint list       # Xem các mốc khôi phục
hitechcloud checkpoint restore <id> # Quay ngược toàn bộ thay đổi về mốc checkpoint
```

- **Crash Recovery Protocol**: Sử dụng SQLite Write-Ahead Logging (WAL) cùng bản sao lưu trạng thái atomic; nếu process CLI bị tắt đột ngột (mất điện, crash OS, SIGKILL), lần chạy tiếp theo của `hitechcloud` sẽ tự động phát hiện và khôi phục lại bước dở dang mà không làm hỏng repo mã nguồn.

---

## 6. Chế Độ Nghiên Cứu & Tra Cứu Web (Web & Research Mode)

- **Web Provider Abstraction**: Hỗ trợ tìm kiếm thông tin tài liệu kỹ thuật cập nhật mà không bị bó cứng vào một search engine cụ thể.
- **Chu trình Research**:
  1. `web_search`: Tìm kiếm các bài viết, tài liệu, issues trên GitHub.
  2. `web_fetch`: Tải trang và trích xuất nội dung văn bản thuần túy (loại bỏ quảng cáo, scripts).
  3. `synthesize`: Tổng hợp thông tin, trích dẫn nguồn (citations) và lưu thành research artifact phục vụ việc lập trình.
- **Mở rộng**: Có thể cắm thêm plugin kết nối cơ sở tri thức nội bộ công ty (Internal Knowledge Base, Confluence, Notion).

---

## 7. Các Tính Năng Lõi Nâng Cao Cho Production (Advanced Production Features)

### 7.1 Vòng Lặp Tự Chữa Lỗi Mã Nguồn (Self-Healing Loop)
Khi agent chỉnh sửa code hoặc viết mới tính năng, nếu lệnh kiểm thử hoặc build bị thất bại:
1. `Error Trapper`: Tự động trích xuất chính xác file, dòng code và thông điệp lỗi trong stack trace.
2. `Hypothesis & Auto-Fix`: Lập giả thuyết nguyên nhân gốc rễ và tự động thử tối đa 3 lần điều chỉnh sửa lỗi.
3. `Regression Verification`: Chạy toàn bộ test suite để đảm bảo việc sửa lỗi không gây hỏng hóc các phần code cũ.

### 7.2 Cơ Chế Ngắt Lặp Thông Minh (Circuit Breaker)
Để ngăn chặn tình trạng Agent gọi tool lặp vô tận (Tool Call Looping) gây lãng phí ngân sách:
- Nếu cùng một tool với tham số tương đương được gọi liên tiếp 4 lần mà kết quả không thay đổi, Circuit Breaker lập tức ngắt tiến trình, chuyển agent về trạng thái `WAITING_APPROVAL` và hỏi ý kiến lập trình viên.
- Hạn mức ngân sách tối đa (`--max-budget-usd`): Tự động dừng hẳn tác vụ khi chi phí token chạm ngưỡng đặt trước.

### 7.3 Phân Tầng Bộ Nhớ Ngữ Cảnh Dài Hạn (Context Memory Hierarchy)
- **User Memory (`~/.hitechcloud/memory/`)**: Lưu trữ phong cách lập trình cá nhân, preferences (ví dụ: "luôn dùng TypeScript strict mode", "không dùng thư viện Axios").
- **Project Memory (`.hitechcloud/memory/`)**: Ghi nhớ các quyết định kiến trúc của repo, danh sách các endpoint đặc thù và quy tắc nghiệp vụ.
- **Session Memory**: Bộ nhớ ngắn hạn trong phiên, tự động nén (compaction) theo thuật toán trích xuất thực thể khi đạt 80% context window.
