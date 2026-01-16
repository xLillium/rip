# Contract: agent server

Summary
- Exposes the coding agent via HTTP/SSE sessions.
- Not an Open Responses API.

Inputs
- Session lifecycle requests (start, send input, cancel).

Outputs
- Structured event stream over SSE (event frames: `docs/03_contracts/event_frames.md`).
- Session status and artifacts.

Config
- Bind address, auth, session limits.

Invariants
- One session maps to one agent run.
- Event stream is ordered and replayable.

Tests
- Session lifecycle integration tests.
- SSE stream compliance tests.
