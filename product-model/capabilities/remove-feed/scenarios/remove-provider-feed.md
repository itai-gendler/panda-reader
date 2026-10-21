---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose Confirm removal for the feed
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Ask the provider to unsubscribe from the feed
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
  - text: Remove the feed with its cached articles and their queued read and star changes
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: removes
        from: Active
      - entity: article
        as: unread-articles
        effect: removes
        from: Unread
        with: feed
      - entity: article
        as: read-articles
        effect: removes
        from: Read
        with: feed
    contexts:
      desktop:
        place: desktop::general-settings
---

# Remove a feed from the provider and locally

## Trigger

The Reader unsubscribes from a feed of their connected FreshRSS account.

## Outcome

The provider no longer has the subscription and "Feed removed from provider and locally" is shown.
