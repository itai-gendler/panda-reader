---
availability:
  - place: desktop
domain: feed-management
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/subscriptions.rs#add_feed
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/subscriptions.rs#add_feed
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/feeds.rs#persist_prepared_feed
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/feed_fetch.rs#fetch_feed_data
---

# Add feed

Subscribe the selected library to a feed URL from **Feed management** in General (also reached from the title bar's Add feed button and the Add feed command).
