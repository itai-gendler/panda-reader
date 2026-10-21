---
appliesTo:
  - type: capability
    id: refresh-feeds
  - type: entity
    id: feed
    facts:
      - Refresh error
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/sync.rs#refresh_local_feeds
---

# A Local refresh fails only when every feed fails

Feeds are refreshed independently. A feed that fails keeps its error, shown in the feed column, until it next refreshes successfully, while the others are saved; the refresh as a whole reports failure only when no feed could be refreshed.
