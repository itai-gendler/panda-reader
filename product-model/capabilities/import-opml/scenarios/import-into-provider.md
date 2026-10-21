---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose Import OPML and pick a file
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: 'Send the file to the provider: Miniflux imports it whole, FreshRSS subscribes to each feed URL'
    kind: product
    actor: reader
    entities:
      - entity: library
        effect: reads
        facts:
          - Source
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Synchronize the library
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
---

# Import an OPML file through the provider

## Trigger

The Reader brings an OPML file into their connected Miniflux or FreshRSS account.

## Outcome

The provider has the file's subscriptions and the library has synchronized them.
