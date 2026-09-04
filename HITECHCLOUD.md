# HITECHCLOUD.md — Project Guidelines for HiTechCloud Agent

## Overview
HiTechCloud CLI is an enterprise-grade AI coding agent platform following the **Claude CLI architecture** (Prompt-driven, Tool-calling loop, Skills, MCP, Sub-agents, Hooks, Permissions).

## Architecture Principles
1. **Claude CLI Structure**: Prioritize composable Skills (`SKILL.md`), Sub-agents, lifecycle Hooks, and Model Context Protocol (MCP).
2. **Provider Agnostic**: Native support for Nube.SH (`ai.nube-api.com`), OpenAI-compatible, Anthropic-compatible, and local models.
3. **Zero-Trust Security**: SafeLine WAF (`vcwaf.hitechcloud.vn`) origin protection, secret redactor, auto-safe permission gate.

## Build & Test Commands
- Run Cargo checks: `cargo check --workspace`
- Run Unit tests: `cargo test --workspace`
- Format code: `cargo fmt`
