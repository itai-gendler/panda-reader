---
appliesTo:
  - type: capability
    id: add-feed
  - type: capability
    id: import-opml
  - type: entity
    id: feed
    facts:
      - Feed URL
references:
  - kind: code
    role: implementation
    target: crates/panda-store/src/store.rs#has_feed_url
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/schema.rs
---

# A feed URL is kept once per library

Adding or importing a URL the Local library already has adds nothing. The same URL may still exist in another library, with its own articles and state.
