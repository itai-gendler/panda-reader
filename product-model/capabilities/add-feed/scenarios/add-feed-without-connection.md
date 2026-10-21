---
kind: validation
routes:
  desktop: Desktop
steps:
  - text: Enter a feed URL and choose Add feed
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts: []
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
  - text: Refuse with "Connect the selected provider before adding feeds"
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

# Refuse to add a feed without a connection

## Trigger

A provider library is selected but its connection was never saved or was disconnected.

## Outcome

"Connect the selected provider before adding feeds" is shown.
