---
appliesTo:
  - type: capability
    id: change-subscription-source
  - type: capability
    id: disconnect-provider-account
  - type: entity
    id: library
references:
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/providers.md
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/command.rs#QueuedCommand
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/tests.rs#same_feed_url_is_isolated_between_local_and_provider_workspaces
---

# Each subscription source keeps its own library

Local subscriptions, Miniflux and FreshRSS each have their own feeds, articles, reading positions and reading state. Changing the subscription source or disconnecting a provider never merges, moves or deletes another library, and the same feed URL in two libraries shares nothing.

## Rationale

Every operation captures the library it was started in, so switching sources while work is queued cannot redirect that work into the other library.
