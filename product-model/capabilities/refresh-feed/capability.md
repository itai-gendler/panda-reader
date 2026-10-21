---
availability:
  - place: desktop
domain: feed-management
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/sync.rs#refresh_feed
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/subscriptions.rs#refresh_one_feed
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/feeds.rs#set_feed_refresh_error
---

# Refresh feed

Refresh one feed with **Sync now** in its menu in the feed column.
