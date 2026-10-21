---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: The app starts with an update scheduled
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
  - text: Hand the package to the updater instead of opening the window
    kind: product
    entities:
      - entity: app-update
        effect: removes
        from: Scheduled for next launch
    contexts:
      desktop:
        place: desktop::about
---

# Install the scheduled update at start-up

## Trigger

Panda Reader starts with an update scheduled for this launch.

## Outcome

The updater installs the package before the window opens and the new version starts.
