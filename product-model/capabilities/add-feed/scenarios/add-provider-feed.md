---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Enter the feed URL and choose Add feed
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Ask the provider to subscribe to the URL
    kind: product
    actor: reader
    entities:
      - entity: library
        effect: reads
        facts:
          - Source
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Synchronize the library, adding the new feed and its entries
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: creates
        to: Active
        facts:
          - Title
          - Feed URL
          - Site URL
          - Folder
      - entity: article
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
        with: feed
      - entity: library
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::general-settings
references:
  - kind: code
    role: implementation
    target: crates/panda-providers/src/lib.rs#add_feed
---

# Add a feed through the provider

## Trigger

The Reader reads from their Miniflux account and wants a new subscription there.

## Outcome

The provider holds the new subscription and the library has synchronized it with its articles.
