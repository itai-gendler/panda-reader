---
kind: primary
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
  - text: Check that the URL uses HTTP or HTTPS and is not already in the library
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts:
          - Feed URL
      - entity: library
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Download and read the feed
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Save the feed with its title and site, and its entries as unread articles
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
        with: feed
    contexts:
      desktop:
        place: desktop::general-settings
---

# Add a feed to the Local library

## Trigger

The Reader has the address of an RSS or Atom feed and the Local library is selected.

## Outcome

The feed and its current entries are in the Local library, the entries unread, and "Feed added" is shown.
