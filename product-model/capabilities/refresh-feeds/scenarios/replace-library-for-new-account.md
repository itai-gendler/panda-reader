---
kind: edge
routes:
  desktop: Desktop
steps:
  - text: Choose Refresh feeds
    kind: actor
    actor: reader
    entities:
      - entity: feed
        as: old-feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::feeds
  - text: The provider reports a different account than the library's Provider account
    kind: condition
    entities:
      - entity: library
        effect: reads
        facts:
          - Provider account
    contexts:
      desktop:
        place: desktop::feeds
  - text: Synchronize the new account into a separate copy while the old library stays visible
    kind: product
    actor: reader
    entities:
      - entity: feed
        as: old-feed
        effect: reads
        facts:
          - Title
      - entity: library
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::feeds
  - text: Replace the library with the completed copy and drop the old account's queued changes
    kind: product
    actor: reader
    entities:
      - entity: library
        effect: changes
        facts:
          - Provider account
          - Last synchronized
          - Last full reconciliation
      - entity: feed
        as: old-feed
        effect: removes
        from: Active
      - entity: article
        as: old-unread-article
        effect: removes
        from: Unread
        with: old-feed
      - entity: article
        as: old-read-article
        effect: removes
        from: Read
        with: old-feed
      - entity: feed
        as: new-feed
        effect: creates
        to: Active
        facts:
          - Title
          - Feed URL
          - Site URL
          - Folder
      - entity: article
        as: new-article
        effect: creates
        to: Unread
        facts:
          - Title
          - Link
          - Author
          - Published
          - Summary
          - Feed content
          - Starred
        with: new-feed
    contexts:
      desktop:
        place: desktop::feeds
references:
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/provider.rs#promote_provider_workspace
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/provider.rs#begin_provider_sync
---

# Replace the library for a different account

## Trigger

The connection was updated and now reaches a different account (or server) than the one the library was synchronized with.

## Outcome

The new account's feeds and articles replace the cached library in one step once the sync completes; read and star changes still queued for the old account are dropped. If the sync fails, the old cached library stays visible.
