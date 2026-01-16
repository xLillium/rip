# Phase 1 Overview

Summary
- Deliver a working agent runtime with CLI (interactive + headless) and server access (HTTP/SSE + OpenAPI).
- Validate speed with baseline benchmarks and replayable fixtures.

Scope
- ripd core runtime
- provider adapters (Open Responses at edge only)
- tool runtime
- workspace engine
- event log + snapshots
- CLI (interactive + headless)
- agent server (HTTP/SSE + OpenAPI spec)
- benchmarks + fixtures

Out of scope
- search/index, memory, context compiler, policy engine, sync, DSPy
- TUI surface (`rip-tui`), MCP surface (`rip-mcp`), SDKs

Exit criteria
- All Phase 1 tasks have acceptance tests.
- Benchmarks run in CI and enforce budgets.
