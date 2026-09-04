# HITECHCLOUD.md — Project Guidelines for HiTechCloud Agent

## Overview
HiTechCloud CLI is an enterprise-grade AI coding agent platform following the **Claude CLI architecture** (Prompt-driven, Tool-calling loop, Skills, MCP, Sub-agents, Hooks, Permissions).

## Architecture Principles
1. **Claude CLI Structure**: Prioritize composable Skills (`SKILL.md`), Sub-agents, lifecycle Hooks, and Model Context Protocol (MCP).
2. **Provider Agnostic**: Native support for Nube.SH (`ai.nube-api.com`), OpenAI-compatible, Anthropic-compatible, and local models.
3. **Zero-Trust Security**: SafeLine WAF (`vcwaf.hitechcloud.vn`) origin protection, secret redactor, auto-safe permission gate.

## Git & Commit Guidelines (Bắt Buộc)
Mỗi commit trong repository này (dù do developer hay Agent tạo ra) **bắt buộc phải có dòng Co-Authored-By** ở cuối thông điệp commit:

### Định dạng commit chuẩn:
```gitcommit
<type>(<scope>): <mô tả ngắn gọn>

<chi tiết thay đổi nếu có>

Co-Authored-By: <GitHub-Username> <<GitHub-Email>>
Co-Authored-By: HiTechCloud Agents <agent@hitechcloud.vn>
```

### Quy tắc commit:
1. **Luôn lấy GitHub user hiện tại** từ `gh api user` hoặc `git config user.name`
2. **Luôn thêm HiTechCloud Agent** như một co-author
3. **Sử dụng git hook** `prepare-commit-msg` để tự động thêm Co-Authored-By
4. **CI/CD tự động commit** với `${{ github.actor }}` và Agent attribution

### Các loại commit (type):
- `feat`: Tính năng mới
- `fix`: Sửa lỗi
- `docs`: Tài liệu
- `style`: Format code
- `refactor`: Tái cấu trúc
- `test`: Thêm/sửa test
- `chore`: Công việc bảo trì
- `ci`: Thay đổi CI/CD
- `perf`: Cải thiện hiệu suất

### Ví dụ:
```gitcommit
feat(provider): add Nube.SH OpenAI-compatible adapter

Implement streaming support and live pricing sync for Nube.SH gateway.

Co-Authored-By: thanhan92-f1 <thanhan92-f1@users.noreply.github.com>
Co-Authored-By: HiTechCloud Agents <agent@hitechcloud.vn>
```

## Build & Test Commands
- Run Cargo checks: `cargo check --workspace`
- Run Unit tests: `cargo test --workspace`
- Format code: `cargo fmt`
- Run clippy: `cargo clippy --workspace --all-targets -- -D warnings`

## Setup Git Hooks
```bash
# Cài đặt git hooks để tự động thêm Co-Authored-By
cp .github/hooks/prepare-commit-msg .git/hooks/prepare-commit-msg
chmod +x .git/hooks/prepare-commit-msg
```
