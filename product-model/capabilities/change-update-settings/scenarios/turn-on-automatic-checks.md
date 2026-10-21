---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Turn on Check for updates automatically
    kind: actor
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts:
          - Check for updates automatically
    contexts:
      desktop:
        place: desktop::about
  - text: Save the choice and check for updates now
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: changes
        facts:
          - Check for updates automatically
    contexts:
      desktop:
        place: desktop::about
---

# Turn on automatic update checks

## Trigger

The Reader wants Panda Reader to keep itself current.

## Outcome

A check starts right away and runs again at start-up and every six hours.
