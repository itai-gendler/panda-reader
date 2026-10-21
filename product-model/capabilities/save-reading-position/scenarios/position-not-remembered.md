---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Scroll through the open article
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::article
  - text: Remember reading position is off
    kind: condition
    entities:
      - entity: settings
        effect: reads
        facts:
          - Remember reading position
    contexts:
      desktop:
        place: desktop::article
---

# Positions are not remembered

## Trigger

Remember reading position is off.

## Outcome

Nothing is saved and articles always open at the top.
