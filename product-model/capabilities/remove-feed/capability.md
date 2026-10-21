---
availability:
  - place: desktop
domain: feed-management
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/subscriptions.rs#remove_feed
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/subscriptions.rs#remove_feed
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/feeds.rs#remove_feed
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/context_menus.rs
---

# Remove feed

Remove a feed and its cached articles from the selected library after **Confirm removal**. In a provider library the provider unsubscribes first.
