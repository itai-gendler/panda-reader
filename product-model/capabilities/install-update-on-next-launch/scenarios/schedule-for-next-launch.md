---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Next launch
    kind: actor
    actor: reader
    entities:
      - entity: app-update
        effect: reads
        facts:
          - Version
    contexts:
      desktop:
        place: desktop::about
  - text: Record the update for the next start
    kind: product
    actor: reader
    entities:
      - entity: app-update
        effect: changes
        from: Ready
        to: Scheduled for next launch
        facts: []
    contexts:
      desktop:
        place: desktop::about
---

# Schedule the update for the next launch

## Trigger

An update is ready but the Reader is in the middle of reading.

## Outcome

"Update will install next time you start Panda Reader." is shown and the status bar shows "Update on next launch".
