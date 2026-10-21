---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Turn off Check for updates automatically
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
  - text: Save the choice
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

# Turn off automatic update checks

## Trigger

The Reader prefers to check by hand.

## Outcome

No automatic checks run; Check now still works.
