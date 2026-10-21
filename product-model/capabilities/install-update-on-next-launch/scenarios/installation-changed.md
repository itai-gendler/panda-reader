---
kind: edge
routes:
  desktop: Desktop
steps:
  - text: The app starts with an update scheduled for a different installation
    kind: condition
    unattended: true
    entities:
      - entity: app-update
        effect: reads
        facts:
          - Version
    contexts:
      desktop:
        place: desktop::about
  - text: Drop the scheduled update and record why
    kind: product
    entities:
      - entity: app-update
        effect: removes
        from: Scheduled for next launch
    contexts:
      desktop:
        place: desktop::about
---

# The installation changed since scheduling

## Trigger

At start-up the scheduled update no longer matches how Panda Reader is installed.

## Outcome

The update is dropped and About shows "The pending update no longer matches this installation. Check for updates again."
