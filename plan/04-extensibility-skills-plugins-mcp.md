# Kế Hoạch 04 — Hệ Thống Mở Rộng: Skills, Plugins & MCP (Extensibility)

**Module:** Skills System, Plugin Architecture, Model Context Protocol (MCP), Sub-agents, Hooks, Marketplace  
**Repo Crates liên quan:** `crates/skills`, `crates/plugins`, `crates/mcp`, `crates/agents`, `crates/marketplace`  

---

## 1. Hệ Thống Kỹ Năng (Skills System)

Skills là các gói chỉ dẫn chuyên sâu (domain-specific instructions), script mẫu và tài liệu tham chiếu giúp Agent nâng cao năng lực giải quyết bài toán theo từng công nghệ cụ thể.

### 1.1 Cấu trúc chuẩn của một Skill
```
my-skill/
├── SKILL.md          # File định nghĩa chính (Frontmatter + Markdown)
├── scripts/          # Shell/Python scripts bổ trợ
├── templates/        # Boilerplate code mẫu
├── references/       # Tài liệu API, cheat sheets kỹ thuật
└── assets/           # Dữ liệu tĩnh phụ trợ
```

File `SKILL.md`:
```markdown
---
name: laravel-review
description: Review Laravel application for bugs, security and performance.
version: 1.0.0
triggers:
  - "review laravel"
  - "check eloquent queries"
---
# Laravel Code Review Guidelines
Review routes, controllers, Eloquent relationships, N+1 query problems, middleware authentication, CSRF, and validation logic.
```

### 1.2 Phạm vi hiệu lực (Scopes)
- **Global:** `~/.hitechcloud/skills/` — Có sẵn cho mọi dự án của developer trên máy.
- **Project:** `.hitechcloud/skills/` — Gắn liền theo repository của dự án, chia sẻ chung qua Git cho cả team.
- **Plugin-bundled:** Đi kèm theo các plugin được cài đặt từ Marketplace.

### 1.3 Cơ chế Progressive Loading
Để tránh làm tràn context window của LLM:
1. Khi khởi động, Agent chỉ đọc phần metadata ngắn (`name`, `description`, `triggers`) trong header của các skill.
2. Chỉ khi người dùng yêu cầu bài toán liên quan hoặc trigger trùng khớp, nội dung chi tiết của Skill mới được nạp vào context.

### 1.4 Danh mục Skill Packs có sẵn (Built-in)
- **Lập trình & Ngôn ngữ:** TypeScript, JavaScript, Python, Rust, Go, PHP/Laravel, Next.js, React, Vue.
- **DevOps & Hạ tầng:** Docker, Kubernetes, Terraform, Ansible, Nginx, Linux sysadmin.
- **Cơ sở dữ liệu:** MySQL, PostgreSQL, Redis, MongoDB.
- **Bảo mật:** Security-review, DevSecOps, OWASP Top 10 auditing.
- **Cloud:** HiTechCloud VPS/Compute, AWS, Azure, GCP.

---

## 2. Kiến Trúc Plugin (Plugin System)

Plugin cho phép mở rộng khả năng của `hitechcloud` CLI vượt xa việc chỉ đưa thêm prompt: plugin có thể đăng ký thêm công cụ (tools), thêm lệnh CLI mới, và cung cấp adapter provider riêng.

### 2.1 Cấu trúc Manifest: `hitechcloud-plugin.json`
```json
{
  "name": "hitechcloud-devops",
  "version": "1.0.0",
  "description": "HiTechCloud DevOps & Infrastructure toolkit",
  "publisher": "hitechcloud",
  "permissions": [
    "filesystem.read",
    "filesystem.write",
    "shell.execute",
    "network.connect"
  ],
  "skills": ["docker", "kubernetes", "terraform"],
  "agents": ["devops"],
  "entrypoint": "bin/devops-plugin"
}
```

### 2.2 Đặt tên & Tránh xung đột (Namespacing)
Mọi lệnh và skill từ plugin đều tuân thủ namespace rõ ràng:
- Gọi tool hoặc skill từ plugin: `/hitechcloud-devops:docker`
- Expose lệnh CLI phụ trợ: `hitechcloud hcloud server list`

---

## 3. Tích Hợp Giao Thức Ngữ Cảnh Mô Hình (MCP Integration)

`hitechcloud` CLI đóng vai trò là một **MCP Client** toàn diện, hỗ trợ kết nối tới cả các server chạy local (`stdio`) và server từ xa (`http`/`sse`).

```bash
hitechcloud mcp list              # Liệt kê các server đang kết nối
hitechcloud mcp add <name> <url>  # Thêm MCP server mới
hitechcloud mcp remove <name>     # Xóa MCP server
hitechcloud mcp test <name>       # Kiểm tra kết nối và danh sách tools
hitechcloud mcp auth <name>       # Cung cấp token xác thực cho server
```

### 3.1 Cấu hình trong `config.toml`
```toml
[mcp.github]
transport = "stdio"
command = "github-mcp-server"
args = ["stdio"]

[mcp.official-registry]
transport = "http"
url = "https://mcp-cli.hitechcloud.vn/servers/official"
```

### 3.2 Kiểm soát an toàn MCP
- Mỗi MCP Server khi được add vào bắt buộc phải khai báo rõ ràng các tool và tài nguyên mà nó cung cấp.
- Mọi tool call từ MCP server vẫn phải đi qua **Permission Engine** của `hitechcloud` — MCP server không bao giờ được tự ý cấp quyền vượt rào.

---

## 4. Sub-Agents & Điều Phối Đa Tác Vụ (Agent Orchestration)

Khi xử lý bài toán lớn, Agent chính (Parent Agent) có thể ủy quyền các phần việc chuyên môn cho các Sub-agents chạy song song trong không gian context độc lập.

### 4.1 Danh sách Sub-agents chuyên trách:
- `coder`: Tập trung viết mã, triển khai thuật toán và sửa lỗi cú pháp.
- `reviewer`: Soát lỗi logic, bảo mật, chuẩn coding conventions và hiệu năng.
- `tester`: Tự động viết test case và chạy kiểm thử hồi quy.
- `debugger`: Phân tích stack trace, log lỗi và tìm nguyên nhân gốc rễ (root cause).
- `researcher`: Quét tài liệu, tìm kiếm giải pháp và tổng hợp thông tin kiến trúc.
- `security`: Phân tích lỗ hổng bảo mật, kiểm tra chuỗi phụ thuộc và kiểm toán quyền.
- `devops`: Tối ưu Dockerfile, pipeline CI/CD, cấu hình k8s và hạ tầng đám mây.

### 4.2 Giới hạn an toàn khi chạy Sub-agents
Mỗi sub-agent chạy dưới sự giám sát chặt chẽ của các chỉ số:
- `max_sub_agents`: Số lượng tối đa sub-agent chạy đồng thời (mặc định: 3).
- `max_tokens`: Hạn mức token cho mỗi nhánh phân tích.
- `max_runtime`: Thời gian thực thi tối đa trước khi tự động dừng.
- `max_cost`: Ngân sách tối đa tính theo USD/VND cho sub-task.

### 4.3 Giao Thức Đồng Thuận Phản Hồi (Sub-Agent Consensus Protocol)
- Khi thực hiện các tác vụ tối quan trọng (Security Audit, Production Deployment Prep), hệ thống kích hoạt **Multi-Agent Consensus**:
  - `security` agent và `reviewer` agent cùng phân tích độc lập một diff code.
  - Parent Agent chỉ phê duyệt merge khi cả hai sub-agent đạt đồng thuận 100% không còn lỗ hổng nghiêm trọng (Zero High/Critical Vulnerabilities).

---

## 5. Life-cycle Hooks (Vòng Đời Sự Kiện)

Hooks cho phép developer cài đặt các script tự động kích hoạt tại các thời điểm quan trọng trong chu trình làm việc của Agent.

### 5.1 Danh sách sự kiện được hỗ trợ:
- `session.start` / `session.end`: Khi phiên bắt đầu hoặc kết thúc.
- `before_tool` / `after_tool`: Trước và sau khi một tool được gọi.
- `before_file_write` / `after_file_write`: Trước và sau khi agent lưu file vào đĩa.
- `before_command` / `after_command`: Trước và sau khi agent thực thi lệnh shell.
- `agent.error` / `model.error`: Khi phát sinh lỗi mạng hoặc lỗi mô hình.

### 5.2 Cấu hình Hook trong `.hitechcloud/config.toml`
```yaml
hooks:
  after_file_write:
    - command: npm run lint:fix
    - command: git status -s
  session.end:
    - command: notify-send "HiTechCloud Agent: Task hoàn thành!"
```

---

## 6. Kiến Trúc Marketplace (`marketplace-cli.hitechcloud.vn`)

```
                    Marketplace Web UI (marketplace-cli.hitechcloud.vn)
                                  │
              ┌───────────────────┼───────────────────┐
              ▼                   ▼                   ▼
      Official Packages    Community Packages    Private Enterprise Packages
 (registry-cli.hitechcloud.vn)             (enterprise-cli.hitechcloud.vn)
```

### 6.1 Tiêu chuẩn Package trên Marketplace
Mỗi package phát hành lên hệ thống đều bắt buộc phải có:
- Định danh độc nhất: `publisher/package-name`.
- Phiên bản tuân thủ SemVer (`1.0.0`).
- Chữ ký số GPG / Ed25519 và Checksum SHA-256 xác thực nguồn gốc.
- Bản khai báo quyền hạn bắt buộc (`permissions.json`).
- Kết quả kiểm định tự động: Static Code Analysis, quét mã độc, không chứa backdoor hoặc API key rò rỉ.
- **Automated Sandbox Detonation**: Mỗi package khi submit lên `registry-cli` đều được tự động cài đặt và chạy thử nghiệm trong môi trường container cô lập để kiểm tra hành vi bất thường trước khi được cấp trạng thái Verified.

### 6.2 Lệnh quản lý nguồn Marketplace
```bash
hitechcloud marketplace list                    # Danh sách các registry đã đăng ký
hitechcloud marketplace add private <reg-url>   # Thêm registry riêng của công ty
hitechcloud marketplace remove <name>           # Xóa registry
hitechcloud marketplace verify <package>        # Kiểm tra chữ ký số và tính toàn vẹn của package
```
hitechcloud marketplace list                    # Danh sách các registry đã đăng ký
hitechcloud marketplace add private <reg-url>   # Thêm registry riêng của công ty
hitechcloud marketplace remove <name>           # Xóa registry
```
