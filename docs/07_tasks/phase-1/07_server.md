# Task 07: Agent server

Goal
- Expose agent sessions via HTTP/SSE.

Inputs
- Module contract: docs/03_contracts/modules/phase-1/06_server.md

Outputs
- ripd exposes the server API (HTTP/SSE) for remote control.

Acceptance criteria
- Start session, send input, stream events, cancel.
- Ordered SSE stream.
- OpenAPI spec generated from server code and served via HTTP.
- Canonical schema snapshot stored in `schemas/`.

Tests
- Session lifecycle integration tests.
- SSE compliance tests.
- OpenAPI schema validation tests.
