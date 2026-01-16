# Capability Registry (Source of Truth)

Summary
- This is the single source of truth for capability ids and versions.
- Each capability must link to tests and surface adapters.
- Status: bootstrap (only session lifecycle capabilities are listed today; expansion is tracked in the roadmap).
- Server and SDK surfaces must align their API schemas with this registry.

Format (per entry)
- id
- version
- intent
- inputs
- outputs
- errors
- surfaces

Registry
- session.create (v1)
- session.send_input (v1)
- session.stream_events (v1) -> outputs event frames (`docs/03_contracts/event_frames.md`)
- session.cancel (v1)
