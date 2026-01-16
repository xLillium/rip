# Rolling Roadmap

Summary
- Single source for "now vs later" decisions.
- Lightweight on purpose; detailed task specs remain in `docs/07_tasks/phase-1/`.
- Each item includes references + start/finish criteria so a fresh context can resume fast.

How to use
- Every item includes a confidence tag: `[confirm spec]` or `[needs work]`.
- `[needs work]` means confirm spec or design choice before implementation.
- Date-stamp moves between Now/Next/Later to preserve intent over time.
- Each item includes: refs, ready checklist, and done criteria.

Now
- (empty)

Next
- CLI interactive renderer + approvals/diffs UI surface [needs work]
  - Refs: `docs/07_tasks/phase-1/06_cli.md`, `docs/03_contracts/modules/phase-1/05_cli.md`, `docs/04_execution/cli.md`
  - Ready: event frame schema final; approval UX spec confirmed
  - Done: interactive UI renders streams + approvals, golden render tests
- Workspace engine: checkpoint + rewind hooks integration in runtime [needs work]
  - Refs: `docs/07_tasks/phase-1/04_workspace_engine.md`, `docs/03_contracts/modules/phase-1/04_workspace_engine.md`
  - Ready: hook points in runtime defined; checkpoint event frames defined
  - Done: runtime integration + rewind behavior tests

Later
- Benchmarks: TTFT, parse overhead, tool dispatch, patch throughput, end-to-end loop [needs work]
  - Refs: `docs/07_tasks/phase-1/08_benchmarks.md`, `docs/05_quality/benchmarks.md`
  - Ready: event frames + tool runtime stable
  - Done: CI-gated benchmarks with baseline budgets
- Fixtures: deterministic tool outputs + replayable logs [needs work]
  - Refs: `docs/07_tasks/phase-1/09_fixtures.md`
  - Ready: tool runtime emits deterministic frames
  - Done: fixture repos + replay tests in CI
- SDK surface parity (TypeScript first) [needs work]
  - Refs: `docs/02_architecture/component_map.md`, `docs/02_architecture/capability_matrix.md`
  - Ready: session API + event frames stable
  - Done: TS SDK supports session lifecycle + streaming

Doc/impl gaps
- Interactive CLI is specified but not implemented (see `docs/07_tasks/phase-1/06_cli.md`).
- Tool runtime not yet integrated into session execution.
- Benchmarks are required by docs but no harness exists.
- Headless CLI currently streams raw SSE messages, not validated JSON frames.

Decisions
- Event frames live in `rip-kernel`; schema documented at `docs/03_contracts/event_frames.md`.
- Phase 1 frame types: `session_started`, `output_text_delta`, `session_ended`, `provider_event`, tool events.

Open questions
- (empty)

Done (recent)
- 2026-01-16: Server SSE compliance tests + session lifecycle integration.
- 2026-01-16: Tool runtime emits structured tool events with limits + tests.
- 2026-01-16: Provider adapter emits full provider_event frames + fixtures/tests.
- 2026-01-16: Event log replay equivalence + corruption detection tests.
- 2026-01-16: Event frame schema defined + serialized across ripd/log/CLI.
