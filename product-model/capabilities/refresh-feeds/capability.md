---
availability:
  - place: desktop
domain: feed-management
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/sync.rs#refresh
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/sync.rs#refresh_local_feeds
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/sync.rs#sync_provider_async
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/subscriptions.rs#refresh_with_force
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/window.rs#new
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/providers.md
---

# Refresh feeds

Bring the whole selected library up to date: the Local library downloads its feeds, a provider library synchronizes with the provider. Started from the feed column's refresh button, **Sync now** in General, the Refresh feeds command, at startup and every 15 minutes.
