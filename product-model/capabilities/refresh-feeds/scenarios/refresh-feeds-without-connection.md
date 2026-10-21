---
kind: validation
routes:
  desktop: Desktop
steps:
  - text: Choose Refresh feeds
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::feeds
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
        place: desktop::feeds
  - text: Refuse with "Connect the selected provider before syncing"
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::feeds
---

# Refuse to sync without a connection

## Trigger

A refresh starts, by hand or on schedule, while the selected provider has no saved connection.

## Outcome

"Connect the selected provider before syncing" is shown and the cached library is unchanged.
