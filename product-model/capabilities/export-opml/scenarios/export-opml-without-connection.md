---
kind: validation
routes:
  desktop: Desktop
steps:
  - text: Choose Export OPML
    kind: actor
    actor: reader
    entities: []
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
  - text: Refuse with "Connect the selected provider before exporting subscriptions"
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::general-settings
---

# Refuse to export without a connection

## Trigger

The Reader chooses Export OPML while the selected provider has no saved connection.

## Outcome

"Connect the selected provider before exporting subscriptions" is shown and no file is written.
