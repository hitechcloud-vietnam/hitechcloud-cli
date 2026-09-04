# Kế Hoạch 02 — Tầng Trừu Tượng Provider & Mô Hình (Provider & Models)

**Module:** Provider Abstraction, Model Registry, Routing, Reliability & Fallback  
**Repo Crates liên quan:** `crates/provider-sdk`, `crates/provider-openai`, `crates/provider-anthropic`, `crates/provider-compatible`  

---

## 1. Provider Abstraction Layer

Tầng trừu tượng Provider cung cấp interface đồng nhất cho tất cả các nhà cung cấp mô hình LLM, cô lập hoàn toàn logic của Agent khỏi API đặc thù của từng vendor.

```
Provider Trait (SDK)
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

### Các nhóm Provider được hỗ trợ:
- **Built-in Native:** `openai`, `anthropic`, `azure-openai`, `hitechcloud` (Gateway chính thức), `local`.
- **Compatible Formats:** `openai-compatible`, `anthropic-compatible`.
- **Tích hợp sẵn Cổng Nube.SH (Multi-format AI Gateway):** `nube` (Hỗ trợ cả OpenAI-compatible lẫn Anthropic-compatible với cùng 1 API key, có live pricing sync).
- **Lộ trình tích hợp tương lai:** `google`, `mistral`, `xai`, `deepseek`, `qwen`, `ollama`, `llama.cpp`, `custom-http`.

### Mở rộng Provider qua Plugin:
Nhà phát triển có thể bổ sung các adapter mô hình mới dưới dạng plugin mà không cần rebuild core CLI binary:

```toml
[providers.google]
type = "plugin"
plugin = "hitechcloud-provider-google"
```

---

## 2. Chuẩn Tương Thích API (API Compatibility)

### 2.1 Chuẩn OpenAI-compatible
Hỗ trợ đầy đủ:
- Chat Completions / Responses style endpoints.
- Streaming responses (Server-Sent Events).
- Function calling / Tool calling theo spec chuẩn của OpenAI.
- Structured Outputs (`response_format = { type: "json_schema" }`).
- Hỗ trợ multimodal inputs (Vision / Images).
- Báo cáo usage tokens (prompt, completion, total).

```toml
[providers.my-gateway]
type = "openai-compatible"
base_url = "https://gateway.example.com/v1"
api_key_env = "MY_GATEWAY_API_KEY"
```

### 2.2 Chuẩn Anthropic-compatible
Hỗ trợ đầy đủ:
- Messages API style endpoints.
- Event-stream format chuẩn Anthropic (`content_block_start`, `content_block_delta`).
- Tool use blocks (`tool_use`, `tool_result`).
- Phân tách system prompt riêng biệt.
- Extended context window và Vision inputs.

```toml
[providers.my-anthropic]
type = "anthropic-compatible"
base_url = "https://gateway.example.com"
api_key_env = "ANTHROPIC_API_KEY"
```

> **Lưu ý:** Các Provider có API schema khác biệt lớn (không map được vào 2 chuẩn trên) sẽ được viết native adapter riêng biệt trong thư mục `crates/`.

---

## 3. Tích Hợp Chuyên Biệt Nube.SH (Nube AI Gateway Integration)

Nube.SH (`ai.nube-api.com`) là một nhà cung cấp AI Gateway chi phí tối ưu, mô hình pay-as-you-go không lock-in, hỗ trợ truy cập đa giao thức.

### 3.1 Cấu Hình Truy Cập API (API Access Config)

> **Điểm nổi bật:** **The same API Key can be used for OpenAI-compatible protocol, Anthropic-compatible protocol, and mainstream AI tools.**  
> Developer chỉ cần 1 API Key duy nhất để cấu hình cho cả hai chuẩn API và tích hợp với các công cụ lập trình AI khác.

#### Base URLs Chuẩn:
- **OpenAI Compatible Base URL:**  
  `https://ai.nube-api.com/v1`
- **Anthropic Compatible Base URL:**  
  `https://ai.nube-api.com`

> **Lưu ý cấu hình quan trọng:**
> - Nhập chính xác Base URL theo bảng trên; **tuyệt đối không nối thêm `/v1` thừa** vào Anthropic Base URL.
> - Không trộn lẫn Base URL giữa 2 giao thức OpenAI và Anthropic.
> - Sử dụng chính xác Model ID được cung cấp từ console hoặc qua Pricing API.

#### Cấu hình trong `.hitechcloud/config.toml`:

- **Chế độ OpenAI Compatible:**
  ```toml
  [providers.nube-openai]
  type = "openai-compatible"
  base_url = "https://ai.nube-api.com/v1"
  api_key_env = "NUBE_API_KEY"
  ```

- **Chế độ Anthropic Compatible:**
  ```toml
  [providers.nube-anthropic]
  type = "anthropic-compatible"
  base_url = "https://ai.nube-api.com"
  api_key_env = "NUBE_API_KEY"
  ```

- **Sử dụng trực tiếp qua lệnh CLI:**
  ```bash
  # Gọi với giao thức OpenAI-compatible
  export NUBE_API_KEY="your-nube-api-key"
  hitechcloud --provider nube-openai --model Nube-Choice

  # Gọi với giao thức Anthropic-compatible (dùng chung NUBE_API_KEY)
  hitechcloud --provider nube-anthropic --model Nube-Choice
  ```

### 3.2 Tự động đồng bộ bảng giá thời gian thực (Live Pricing API)
`hitechcloud` CLI tích hợp cơ chế tự động truy vấn public pricing endpoint của Nube.SH để cập nhật giá token chính xác mà không cần API key:
- **Pricing URL:** `GET https://ai.nube-api.com/v1/models/pricing`
- **Xác thực:** Public (No API Key required)
- **Đơn vị giá:** USD / triệu tokens (USD / million tokens)

**Bảng mô hình & Giá Nube.SH tiêu biểu tích hợp vào Model Registry:**

| Tên Model ID | Context Window | Input (USD/1M) | Output (USD/1M) | Cache Read (USD/1M) | Tags & Năng Lực | Đặc Điểm |
|---|:---:|:---:|:---:|:---:|:---:|---|
| `Nube-Choice` *(Khuyên dùng)* | 1,000,000 | $0.286 | $0.858 | $0.0065 | text, reasoning, tools | Model tối ưu mặc định của Nube (hiện tại là DeepSeek-V4-Flash) |
| `DeepSeek-V4-Flash` | 1,000,000 | $0.286 | $0.858 | $0.0065 | text, reasoning, tools | Siêu tốc độ, suy luận mạnh, context 1M, chi phí cực rẻ |
| `GLM-5.3-Flash` | 128,000 | $0.060 | $0.200 | $0.0120 | text, tools | Mô hình siêu tiết kiệm chi phí cho các tác vụ phân tích nhanh |
| `Qwen3.8-27B` | 256,000 | $0.325 | $1.950 | $0.0650 | text, image, video, reasoning, tools | Đa phương thức (Multimodal), xử lý code và toán học xuất sắc |
| `GLM-5.3` | 1,000,000 | $0.910 | $2.860 | $0.1690 | text, reasoning, tools | Cân bằng chất lượng cao cho coding và kiến trúc hệ thống |
| `Kimi-K2.6` | 256,000 | $0.6175 | $2.600 | $0.1040 | text, image, video, reasoning, tools | Xử lý tài liệu và ngữ cảnh dài với chi phí hợp lý |
| `Kimi-K3` | 1,000,000 | $3.300 | $16.500 | $0.3300 | text, image, video, reasoning, tools | Flagship model năng lực cao cho các bài toán lập trình phức tạp |

### 3.3 Triết lý kiểm thử thực tế (Workload-based Evaluation)
Tuân thủ khuyến nghị từ Nube.SH: *“Does the model do your job well?”*
- `hitechcloud` CLI cung cấp lệnh kiểm thử trực tiếp prompt trên khối lượng công việc thực tế của repo:
  ```bash
  hitechcloud model test --provider nube-openai --model Nube-Choice
  ```
- Cho phép developer tự đánh giá chất lượng đầu ra thực tế so với chi phí token trước khi đưa vào workflow chính thức, tận dụng chính sách pay-as-you-go không bị ràng buộc (no lock-in).

---

## 4. Model Registry & Chiến Lược Routing

### 4.1 Cấu hình Model Registry
Mỗi mô hình trong hệ thống được định danh và quản lý chi tiết trong registry:

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

### 4.2 Chiến lược Dynamic Routing
Hỗ trợ các chiến lược điều phối mô hình linh hoạt:
- `fixed`: Luôn sử dụng một mô hình chỉ định.
- `fallback`: Chuyển sang mô hình dự phòng nếu mô hình chính gặp sự cố hoặc quá tải.
- `fastest`: Tối ưu hóa thời gian phản hồi (TTFT - Time to First Token).
- `cheapest`: Tối ưu hóa chi phí token (tận dụng `GLM-5.3-Flash` hoặc `DeepSeek-V4-Flash` từ Nube.SH).
- `quality`: Ưu tiên mô hình có năng lực suy luận và lập trình cao nhất.
- `balanced`: Cân bằng giữa chi phí, tốc độ và chất lượng kết quả (`Nube-Choice`).
- `custom`: Định tuyến theo rules tùy chỉnh của doanh nghiệp.

```toml
[routing]
strategy = "fallback"
primary = "hitechcloud/ag/gemini-3.7-flash-high"
fallback = ["nube-openai/Nube-Choice", "anthropic/claude-sonnet", "openai/gpt-5.1"]
```

### 4.3 Điều phối theo loại tác vụ (Task-based Routing):
- **Coding / Refactor phức tạp:** Điều hướng sang mô hình cao cấp (Claude Sonnet / GPT-5.1 / Kimi-K3 / Gemini High).
- **Phân tích nhanh / Tìm kiếm file / Linter:** Điều hướng sang mô hình tốc độ cao, chi phí rẻ (`GLM-5.3-Flash`, `DeepSeek-V4-Flash`).
- **Dự án bảo mật cao / Offline:** Điều hướng sang mô hình local (Ollama / vLLM chạy on-premise).
- **Tra cứu tài liệu online:** Điều hướng sang mô hình có hỗ trợ Web browsing.

---

## 5. Độ Tin Cậy & Cơ Chế Fallback (Reliability & Fallback)

### 5.1 Chuỗi Fallback tự động
Khi một provider gặp sự cố ngoài ý muốn, hệ thống tự động thử lại hoặc chuyển tiếp request theo chuỗi đã định cấu hình:

```
Provider A (Hạ tầng chính) → (gặp lỗi mạng/5xx/quota) → Nube.SH (Nube-Choice) → (gặp lỗi) → Provider C
```

### 5.2 Phân loại lỗi (Error Classification)
Hệ thống phân biệt rõ bản chất lỗi để quyết định có fallback hay không:
- **Lỗi CẦN Fallback:** `rate_limit (429)`, `timeout`, `server_error (5xx)`, `network_error`.
- **Lỗi KHÔNG Fallback (dừng ngay & báo lỗi cho người dùng):** `authentication_error (401/403)`, `invalid_request (400 - prompt hoặc schema sai)`, `context_limit (vượt quá token window)`, `tool_error` (logic tool trả về lỗi).

### 5.3 Chế độ Offline & Local Model
- Tương thích tốt với các môi trường local LLM: **Ollama**, **llama.cpp**, **vLLM**, **LocalAI**.
- Tự động fallback về endpoint local khi mất kết nối Internet nếu đã cấu hình sẵn:
  ```toml
  [providers.local-ollama]
  type = "openai-compatible"
  base_url = "http://127.0.0.1:11434/v1"
  api_key = "ollama"
  ```
