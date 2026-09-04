# Kế Hoạch 07 — Hạ Tầng Domain, SafeLine WAF & Doanh Nghiệp (Infrastructure, WAF & Enterprise)

**Module:** Subdomain Hierarchy (chuẩn hóa hậu tố `-cli`), SafeLine WAF (`vcwaf.hitechcloud.vn`), Origin Protection (Whitelist IP), Enterprise Control Plane  
**Mục tiêu:** Định hình hạ tầng mạng an toàn tuyệt đối, chống lộ IP gốc, bảo vệ qua WAF và quản lý tập trung doanh nghiệp  

---

## 1. Cơ Chế Bảo Vệ SafeLine WAF & Chống Lộ IP Gốc (WAF & Origin IP Protection)

Toàn bộ dịch vụ backend của hệ sinh thái HiTechCloud CLI **không được phép mở trực tiếp ra Internet**. Tất cả các truy cập công khai bắt buộc phải đi qua cụm Web Application Firewall: **SafeLine WAF (`vcwaf.hitechcloud.vn`)**.

### 1.1 Sơ đồ dòng chảy lưu lượng bảo mật:
```
Người dùng / CLI / Extension
             │ (HTTPS Request tới các subdomain *-cli.hitechcloud.vn)
             ▼
┌─────────────────────────────────────────────────────────────┐
│                 SafeLine WAF (Chắn phía trước)              │
│                    `vcwaf.hitechcloud.vn`                   │
│                   (IP hiện tại: 103.249.117.146)            │
├─────────────────────────────────────────────────────────────┤
│ - Kiểm soát chống DDoS, Botnet, Rate Limiting                │
│ - Chặn SQL Injection, Command Injection, Malicious Payloads │
│ - Quản lý SSL/TLS Termination tập trung                      │
│ - Reverse Proxy chuyển tiếp an toàn vào máy chủ nội bộ      │
└──────────────────────────────┬──────────────────────────────┘
                               │ (Lưu lượng sạch sau khi lọc)
                               │ [CHỈ CHO PHÉP IP CỦA WAF]
                               ▼
┌─────────────────────────────────────────────────────────────┐
│               Máy Chủ Ứng Dụng Gốc (Origin Servers)          │
│            (AI Gateway, Auth, Registry, MCP Hub,...)         │
├─────────────────────────────────────────────────────────────┤
│ 🛡️ BẢO VỆ CHỐNG LỘ IP GỐC:                                 │
│ 1. Cấu hình tường lửa máy chủ (UFW / iptables / Security    │
│    Group): CHỈ WHITELIST IP CỦA SAFELINE WAF                │
│    (Ví dụ: 103.249.117.146 và các node WAF phụ trợ).        │
│ 2. REJECT / DROP tất cả các kết nối trực tiếp từ Internet   │
│    bỏ qua WAF vào port 80/443 của Origin Server.            │
│ 3. Ứng dụng đọc Real IP qua header `X-Forwarded-For`        │
│    do WAF gắn vào một cách an toàn.                         │
└─────────────────────────────────────────────────────────────┘
```

### 1.2 Chiến lược Whitelist IP & Chống Bypass WAF:
1. **Kiểm tra IP của SafeLine WAF**:
   - Địa chỉ phân giải hiện tại của `vcwaf.hitechcloud.vn`: `103.249.117.146` (hoặc dải IP WAF Cluster của hạ tầng HiTechCloud).
   - Tự động hóa kiểm tra định kỳ (health check & IP sync) để cập nhật danh sách IP WAF hợp lệ.
2. **Quy tắc tường lửa máy chủ gốc (Origin Server Firewall Rules)**:
   ```bash
   # Chỉ cho phép WAF kết nối tới cổng web dịch vụ nội bộ
   sudo ufw default deny incoming
   sudo ufw allow from 103.249.117.146 to any port 443 proto tcp comment "SafeLine WAF HTTPS"
   sudo ufw allow from 103.249.117.146 to any port 80 proto tcp comment "SafeLine WAF HTTP"
   # Không public bất kỳ bản ghi A nào của Origin Server ra DNS công khai!
   ```
3. **Ẩn hoàn toàn IP gốc khỏi DNS công khai**:
   - Tất cả bản ghi DNS của các subdomain `-cli` đều trỏ CNAME về `vcwaf.hitechcloud.vn` hoặc trỏ A record về dải IP của SafeLine WAF (`103.249.117.146`).
   - Tuyệt đối không để lộ IP thật trong MX records, mail server header hoặc direct subdomain.
4. **Bảo Vệ Đa Tầng Cho Cổng AI Gateway (`api-cli`)**:
   - SafeLine WAF áp dụng quy tắc kiểm tra Token Throttling & DDoS Mitigations.
   - Hỗ trợ mTLS (Mutual TLS) giữa SafeLine WAF và Origin Server nếu hệ thống yêu cầu mã hóa nội bộ nghiêm ngặt.

---

## 2. Quy Hoạch Hạ Tầng Subdomains (Chuẩn Hóa Hậu Tố `-cli`)

Để **tránh xung đột và trùng lặp hoàn toàn** với các hệ thống hiện có của HiTechCloud (như `api.hitechcloud.vn`, `auth.hitechcloud.vn`, `docs.hitechcloud.vn` vốn dành cho các sản phẩm Web/Portal/Cloud khác), tất cả các subdomain phục vụ CLI & Agent được chuẩn hóa tiền/hậu tố **`-cli`**:

### 2.1 Bảng phân bổ Subdomains chính thức:

| Subdomain Chuẩn Hóa | Vai Trò & Chức Năng | Định Tuyến Qua WAF |
|---|---|:---:|
| **`cli.hitechcloud.vn`** | **Trang chính sản phẩm CLI**: Landing page, tài liệu giới thiệu, hướng dẫn cài đặt `curl ... \| sh`, changelog. | ✅ `vcwaf.hitechcloud.vn` |
| **`mcp-cli.hitechcloud.vn`** | **Cổng kết nối máy chủ MCP (MCP Registry & Gateway)**: Quản lý, xác thực danh tính và proxy cho các máy chủ MCP khi chạy `hitechcloud mcp add`. | ✅ `vcwaf.hitechcloud.vn` |
| **`docs-cli.hitechcloud.vn`** | **Tài liệu kỹ thuật chuyên sâu**: Hướng dẫn API, viết Skill/Plugin, mẫu cấu hình và Developer guides cho CLI & Extension. | ✅ `vcwaf.hitechcloud.vn` |
| **`api-cli.hitechcloud.vn`** | **Cổng HiTechCloud AI Gateway**: Định tuyến model, quản lý hạn mức (quota), billing tập trung cho CLI/Extension; endpoint của provider `type = "hitechcloud"`. | ✅ `vcwaf.hitechcloud.vn` |
| **`auth-cli.hitechcloud.vn`** | **Dịch vụ định danh (SSO/OAuth2) cho CLI**: Quản lý đăng nhập CLI (`hitechcloud auth login`), VS Code Extension và cấp token phiên. | ✅ `vcwaf.hitechcloud.vn` |
| **`registry-cli.hitechcloud.vn`** | **Kho lưu trữ gói mở rộng (Package Registry)**: Lưu trữ metadata, chữ ký số, package file của Skills, Plugins, Agents. | ✅ `vcwaf.hitechcloud.vn` |
| **`marketplace-cli.hitechcloud.vn`**| **Giao diện Web Marketplace**: Cổng tra cứu, đánh giá, chia sẻ Skills và Plugins cho cộng đồng. | ✅ `vcwaf.hitechcloud.vn` |
| **`download-cli.hitechcloud.vn`** | **Mạng CDN phân phối binary**: Cung cấp các gói `.deb`, `.rpm`, `.pkg`, `.exe`, checksum SHA-256 và GPG public keys. | ✅ `vcwaf.hitechcloud.vn` |
| **`update-cli.hitechcloud.vn`** | **Cổng kiểm tra cập nhật**: Phục vụ lệnh `hitechcloud update` (hỗ trợ các kênh: `stable`, `beta`, `nightly`). | ✅ `vcwaf.hitechcloud.vn` |
| **`status-cli.hitechcloud.vn`** | **Trang trạng thái dịch vụ**: Theo dõi độ sẵn sàng (uptime) và latency của Gateway, MCP Hub, Auth và Registry. | ✅ `vcwaf.hitechcloud.vn` |
| **`telemetry-cli.hitechcloud.vn`** | **Cổng chẩn đoán ẩn danh**: Thu thập crash log và mã lỗi kỹ thuật (chỉ kích hoạt khi opt-in). | ✅ `vcwaf.hitechcloud.vn` |
| **`enterprise-cli.hitechcloud.vn`** | **Bảng điều khiển quản trị doanh nghiệp**: Quản lý phân quyền tổ chức, team, phân bổ chi phí và kiểm soát policy. | ✅ `vcwaf.hitechcloud.vn` |
| **`community-cli.hitechcloud.vn`** | **Diễn đàn trao đổi cộng đồng**: Thảo luận kỹ thuật, showcase prompt và chia sẻ kinh nghiệm sử dụng CLI. | ✅ `vcwaf.hitechcloud.vn` |
| **`gateway-sg-cli.hitechcloud.vn`** | **AI Gateway khu vực Singapore**: Định tuyến tối ưu độ trễ cho người dùng Việt Nam và Đông Nam Á. | ✅ `vcwaf.hitechcloud.vn` |
| **`gateway-eu-cli.hitechcloud.vn`** | **AI Gateway khu vực Châu Âu**: Định tuyến tối ưu độ trễ cho người dùng tại thị trường EU. | ✅ `vcwaf.hitechcloud.vn` |

---

## 3. Nền Tảng Quản Trị Doanh Nghiệp (Enterprise Control Plane: `enterprise-cli.hitechcloud.vn`)

Dành cho các công ty và tập đoàn cần kiểm soát chặt chẽ việc sử dụng AI trong toàn bộ tổ chức:

### 3.1 Các tính năng quản trị chính:
- **Quản lý Tổ Chức & Phòng Ban (Multi-tenant Orgs & Teams)**: Phân bổ nhân sự theo từng dự án hoặc bộ phận kỹ thuật.
- **Kiểm soát Mô hình & Nhà cung cấp**:
  - Khóa danh sách các model được phép sử dụng (chỉ cho phép các mô hình đạt chuẩn bảo mật).
  - Cấm hoàn toàn việc gửi dữ liệu ra các nhà cung cấp bên ngoài nếu dự án đòi hỏi bảo mật on-premise.
- **Chính sách Thực thi An toàn (Security Policy Enforcement)**:
  - Tự động áp đặt chế độ phê duyệt tối thiểu (`approval = auto-safe`).
  - Danh sách đen các lệnh cấm chạy (`blocked_commands`: `rm -rf`, `dd`, `shutdown`, `kubectl delete ns`,...).
- **Chợ ứng dụng riêng (Private Marketplace)**: Chỉ cho phép cài đặt các Skills và Plugins đã được nội bộ công ty phê duyệt và kiểm toán bảo mật qua `registry-cli`.
- **Kiểm toán & Báo cáo (Audit Log & Cost Control)**:
  - Báo cáo chi tiết lượng token tiêu thụ và ngân sách hàng tháng theo từng thành viên/phòng ban.
  - Lưu trữ lịch sử các câu lệnh nhạy cảm đã được phê duyệt phục vụ công tác thanh tra bảo mật.

### 3.2 Ví dụ cấu hình chính sách doanh nghiệp tải về máy client:
```yaml
# Policy được đồng bộ tự động từ enterprise-cli.hitechcloud.vn
policy_version: "2026.1"
organization: "Enterprise-Corp"
enforce_rules:
  allowed_providers: ["hitechcloud-internal-gw"]
  allowed_models: ["secure-coder-v2", "claude-sonnet-enterprise"]
  allow_network_tools: false
  allow_shell_execution: true
  blocked_commands:
    - "rm -rf /"
    - "mkfs"
    - "curl * | bash"
    - "docker push *"
  marketplace:
    mode: "private-only"
    registry: "https://registry-cli.hitechcloud.vn/private"
```

---

## 4. Tích Hợp Hệ Sinh Thái HiTechCloud (HiTechCloud Ecosystem Integration)

HiTechCloud CLI không đứng riêng rẽ mà đóng vai trò là "chìa khóa vạn năng" kết nối toàn bộ hệ sinh thái dịch vụ đám mây của HiTechCloud:

```
                            Hệ Sinh Thái HiTechCloud
                                       │
        ┌──────────────┬───────────────┼───────────────┬──────────────┐
        ▼              ▼               ▼               ▼              ▼
   HiTechCloud    HiTechNode.AI   HiTechSecure    HiTechData     HiTechMail
  (Hạ tầng VPS)   (GPU & Model)   (Bảo mật SOC)  (Lưu trữ/DB)   (Email Doanh Nghiệp)
```

| Dịch Vụ Trong Hệ Sinh Thái | Khả Năng Tích Hợp Của CLI & Agent |
|---|---|
| **HiTechCloud Infrastructure** | Quản lý máy chủ VPS, cấu hình mạng riêng ảo (VPC), tự động tạo bản sao lưu snapshot, theo dõi tải CPU/RAM thông qua plugin lệnh `hitechcloud hcloud ...`. |
| **HiTechNode.AI** | Kết nối trực tiếp vào hạ tầng máy chủ GPU hiệu năng cao của HiTechCloud, cho phép chạy suy luận (inference) các mô hình mã nguồn mở (DeepSeek, Qwen, Llama 3) với độ trễ cực thấp tại Việt Nam. |
| **HiTechSecure** | Tích hợp sâu cùng cụm SafeLine WAF (`vcwaf.hitechcloud.vn`) để bảo vệ luồng kết nối; cung cấp skill kiểm toán an ninh chuyên sâu: Quét mã tĩnh (SAST), kiểm tra cấu hình WAF, rà soát lỗ hổng trước khi deploy. |
| **HiTechData** | Tích hợp MCP Server kết nối trực tiếp với dịch vụ S3-compatible Object Storage, tự động hóa sao lưu cơ sở dữ liệu MySQL/PostgreSQL và hỗ trợ truy vấn dữ liệu an toàn. |
| **HiTechMail** | Bộ kỹ năng tự động chẩn đoán và cấu hình bản ghi máy chủ thư điện tử (SPF, DKIM, DMARC, MX records), hỗ trợ developer kiểm tra luồng gửi mail của ứng dụng. |
