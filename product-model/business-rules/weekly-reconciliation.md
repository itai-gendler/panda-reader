---
appliesTo:
  - type: capability
    id: refresh-feeds
  - type: capability
    id: refresh-feed
  - type: entity
    id: library
    facts:
      - Last full reconciliation
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/sync.rs#sync_provider_async
  - kind: code
    role: implementation
    target: crates/panda-providers/src/lib.rs#entries_page
---

# Provider libraries are fully reconciled at least once a week

A provider sync normally downloads only entries changed since the last checkpoint. The first sync, a sync after the account changed, and any sync seven or more days after the last full reconciliation read every entry instead. Miniflux servers that cannot filter by change time are read in full, page by page, resuming where an interrupted sync stopped.
