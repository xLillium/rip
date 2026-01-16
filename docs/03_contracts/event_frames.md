# Event Frames (Phase 1)

Summary
- Canonical internal event schema for all surfaces.
- Frames are compact structs in Rust; JSON only at the edges (SSE/logging).

Schema (v1)
- `id`: string (uuid)
- `session_id`: string (uuid)
- `seq`: u64 (monotonic per session)
- `timestamp_ms`: u64 (unix epoch ms)
- `type`: string (frame type)
- `payload`: fields defined by `type`

Frame types
- `session_started`
  - `input`: string
- `output_text_delta`
  - `delta`: string
- `session_ended`
  - `reason`: string

Invariants
- `seq` starts at 0 and increments by 1 for each emitted frame.
- Frames are append-only and ordered within a session.
- `session_ended` is the terminal frame for a session.

Example
```
{"id":"...","session_id":"...","timestamp_ms":0,"seq":0,"type":"session_started","input":"hi"}
{"id":"...","session_id":"...","timestamp_ms":1,"seq":1,"type":"output_text_delta","delta":"ack: hi"}
{"id":"...","session_id":"...","timestamp_ms":2,"seq":2,"type":"session_ended","reason":"completed"}
```
