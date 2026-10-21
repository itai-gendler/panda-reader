---
appliesTo:
  - type: capability
    id: add-feed
  - type: capability
    id: remove-feed
  - type: capability
    id: import-opml
  - type: capability
    id: export-opml
  - type: capability
    id: refresh-feeds
  - type: capability
    id: refresh-feed
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/subscriptions.rs#add_feed
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/sync.rs#refresh
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/providers.md
---

# Provider changes need a saved connection

In a Miniflux or FreshRSS library, adding, removing, importing, exporting and refreshing feeds go through the provider and are refused with "Connect the selected provider before …" until its connection is saved. Reading and marking the cached articles still work.
