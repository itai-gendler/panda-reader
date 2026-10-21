---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose Reset settings
    kind: actor
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Choose Cancel at "Reset app settings now?"
    kind: actor
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::general-settings
---

# Cancel the reset

## Trigger

The Reader chose Reset settings by mistake.

## Outcome

Nothing changes.
