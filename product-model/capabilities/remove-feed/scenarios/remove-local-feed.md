---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Remove on the feed in Added feeds
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

# Remove a Local feed and its articles

## Trigger

The Reader no longer wants a Local feed.

## Outcome

The feed and all its articles, read or unread, are gone from the Local library and the list returns to All Articles.
