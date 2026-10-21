---
appliesTo:
  - type: capability
    id: refresh-feeds
  - type: capability
    id: connect-provider-account
  - type: entity
    id: library
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/sync.rs#sync_provider_async
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/provider.rs#promote_provider_workspace
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/tests.rs#provider_workspace_promotion_keeps_active_data_until_commit
---

# Another account replaces the library only after a complete sync

When a provider connection reaches a different account or server than the one the library holds, the next sync builds the new account's library separately and swaps it in only when every page has arrived. Until then, or if the sync fails, the old cached library stays visible. Read and star changes still queued for the old account are dropped, not sent to the new one.
