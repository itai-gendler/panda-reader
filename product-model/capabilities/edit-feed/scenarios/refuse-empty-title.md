---
kind: validation
routes:
  desktop: Desktop
steps:
  - text: Clear the Title and choose Save
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::edit-feed
  - text: Refuse with "Feed title cannot be empty"
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::edit-feed
---

# Refuse an empty title or feed URL

## Trigger

The Reader clears the Title or Feed URL and saves.

## Outcome

"Feed title cannot be empty" or "Feed URL cannot be empty" is shown and the feed is unchanged.
