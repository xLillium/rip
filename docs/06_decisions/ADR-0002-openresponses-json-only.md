# ADR-0002: OpenResponses request bodies are JSON-only

Status
- Accepted

Context
- OpenResponses specification states request bodies MUST be `application/json`.
- Reference docs list `application/x-www-form-urlencoded` as allowed.
- Provider boundary behavior must be consistent for validation and tooling.

Decision
- Enforce JSON-only request bodies for OpenResponses.
- Do not accept or emit form-encoded bodies.

Consequences
- Clients must send `application/json`.
- If the upstream spec changes, revisit this decision.
