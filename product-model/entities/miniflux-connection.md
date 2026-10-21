---
domain: subscription-source
relations:
  - entity: library
    verb: syncs
    cardinality: one-to-one
references:
  - kind: code
    role: implementation
    target: crates/panda-miniflux/src/lib.rs#Connection
  - kind: code
    role: implementation
    target: crates/panda-providers/src/lib.rs#ProviderSettings
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/providers.md
---

# Miniflux connection

The saved Miniflux server and API token the Miniflux library synchronizes with.

## Information kept

- **Server URL** — the Miniflux instance address; a pasted /v1 API address works too
- **API token** — the Miniflux API token
