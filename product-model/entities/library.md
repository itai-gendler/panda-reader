---
domain: subscription-source
relations:
  - entity: feed
    verb: holds
    cardinality: one-to-many
references:
  - kind: doc
    role: intent
    target: docs/src/content/docs/docs/providers.md
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/provider.rs#provider_sync_state
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/provider.rs#promote_provider_workspace
  - kind: code
    role: implementation
    target: apps/panda-reader/src/app/preferences.rs#LibrarySource
---

# Library

The feeds and cached articles of one subscription source. Panda Reader keeps three: Local subscriptions, Miniflux and FreshRSS, each with its own feeds, articles and reading state.

## Information kept

- **Source** — Local subscriptions, Miniflux or FreshRSS
- **Provider account** — the provider server and account a provider library was last synchronized with
- **Last synchronized** — when a provider library last finished downloading a page of changes
- **Last full reconciliation** — when a provider library last read every entry from the provider rather than only recent changes
