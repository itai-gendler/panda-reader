---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose Sync now
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Send the read and star changes queued since the last sync to the provider
    kind: product
    actor: reader
    entities:
      - entity: article
        as: synced-article
        effect: reads
        facts:
          - Starred
    contexts:
      desktop:
        place: desktop::general-settings
  - text: >-
      Download the subscriptions, adding new feeds, updating titles, URLs and folders, and removing feeds the
      provider no longer has
    kind: product
    actor: reader
    entities:
      - entity: feed
        as: new-feed
        effect: creates
        to: Active
        facts:
          - Title
          - Feed URL
          - Site URL
          - Folder
          - Language
      - entity: feed
        as: existing-feed
        effect: changes
        facts:
          - Title
          - Feed URL
          - Site URL
          - Folder
          - Language
      - entity: feed
        as: dropped-feed
        effect: removes
        from: Active
      - entity: article
        as: dropped-unread-article
        effect: removes
        from: Unread
        with: dropped-feed
      - entity: article
        as: dropped-read-article
        effect: removes
        from: Read
        with: dropped-feed
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Download entries changed since the last sync, 500 at a time, saving new ones as articles
    kind: product
    actor: reader
    entities:
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
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Take the read and starred state the provider reports for saved articles
    kind: product
    actor: reader
    entities:
      - entity: article
        as: synced-article
        effect: changes
        from: Unread
        to: Read
        facts:
          - Starred
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Record how far the library has synchronized
    kind: product
    actor: reader
    entities:
      - entity: library
        effect: changes
        facts:
          - Last synchronized
    contexts:
      desktop:
        place: desktop::general-settings
references:
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/provider.rs#save_remote_feeds
  - kind: code
    role: implementation
    target: crates/panda-store/src/store/provider.rs#save_prepared_remote_entries
---

# Synchronize a provider library

## Trigger

The Reader chooses Sync now in General while a connected provider library is selected.

## Outcome

The provider has every queued read and star change, and the library matches the provider's subscriptions and recent entries, including read and starred state changed elsewhere.

## Edge cases

- An entry the provider reports as removed is deleted from the library.
- An entry whose content changed at the provider loses its full text and translations, which belonged to the old content.
