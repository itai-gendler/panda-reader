---
kind: validation
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
  - text: The selected provider has no saved connection
    kind: condition
    entities:
      - entity: miniflux-connection
        effect: reads
        facts: []
      - entity: library
        effect: reads
        facts:
          - Source
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Refuse with "Connect the selected provider before removing feeds"
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::general-settings
---

# Refuse to remove a feed without a connection

## Trigger

The selected provider library has no saved connection.

## Outcome

"Connect the selected provider before removing feeds" is shown and the feed stays.
