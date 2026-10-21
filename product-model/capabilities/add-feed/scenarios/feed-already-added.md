---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Enter a feed URL that was added before and choose Add feed
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: The Local library already has a feed with this URL
    kind: condition
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
---

# The feed is already in the library

## Trigger

The URL entered is already a feed of the Local library.

## Outcome

Nothing is downloaded and the library keeps its one copy of the feed.
