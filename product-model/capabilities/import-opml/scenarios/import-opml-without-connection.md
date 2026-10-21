---
kind: validation
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
  - text: The selected provider has no saved connection
    kind: condition
    entities:
      - entity: freshrss-connection
        effect: reads
        facts: []
      - entity: library
        effect: reads
        facts:
          - Source
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Refuse with "Connect the selected provider before importing subscriptions"
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::general-settings
---

# Refuse to import without a connection

## Trigger

The Reader picks an OPML file while the selected provider has no saved connection.

## Outcome

"Connect the selected provider before importing subscriptions" is shown.
