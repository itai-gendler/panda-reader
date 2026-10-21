---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose Remove feed in the feed's menu
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::feeds
  - text: Open General asking to confirm that feed's removal
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Choose Confirm removal
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
  - text: Remove the feed with its cached articles
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: removes
        from: Refresh failed
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

# Start removal from the feed menu

## Trigger

The Reader chooses Remove feed in a feed's menu in the feed column.

## Outcome

General opens with that feed's removal waiting for confirmation; confirming removes it.
