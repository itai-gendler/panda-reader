---
appliesTo:
  - type: capability
    id: edit-feed
  - type: capability
    id: refresh-feeds
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/subscriptions.rs#update_feed
  - kind: code
    role: implementation
    target: apps/panda-reader/src/ui/feed_editor.rs#save_feed_editor
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/provider.rs#save_remote_feeds
---

# Provider feed details come from the provider

Title, folder and feed URL can be edited only for Local feeds; a provider sync overwrites them from the provider. For provider feeds Panda Reader keeps only its own choice of title translation.
