---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Later
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
  - text: Set the update aside
    kind: product
    actor: reader
    entities:
      - entity: app-update
        effect: changes
        from: Ready
        to: Postponed
        facts: []
    contexts:
      desktop:
        place: desktop::about
---

# Postpone an update

## Trigger

An update is ready but the Reader does not want it now.

## Outcome

About shows "Version {} is available when you are ready."; the next check offers it again.
