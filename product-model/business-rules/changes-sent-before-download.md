---
appliesTo:
  - type: capability
    id: refresh-feeds
  - type: capability
    id: refresh-feed
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/sync.rs#flush_remote_marks
---

# Queued changes are sent before a sync downloads

A provider sync first sends the read and star changes queued in this library, and only then downloads the provider's state, so a change made here is not undone by the provider's older state.
