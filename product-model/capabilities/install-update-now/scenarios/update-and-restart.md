---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: An update is ready
    kind: condition
    entities:
      - entity: app-update
        effect: reads
        facts:
          - Version
    contexts:
      desktop:
        place: desktop::about
  - text: Choose Update and restart
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
  - text: Check the package again, hand it to the updater and close
    kind: product
    actor: reader
    entities:
      - entity: app-update
        effect: removes
        from: Ready
    contexts:
      desktop:
        place: desktop::about
---

# Update and restart now

## Trigger

An update is ready and the Reader has a moment.

## Outcome

The window closes, the verified package is installed and the new version starts.

## Edge cases

- If the package no longer matches its checksum, the error is shown and Panda Reader keeps running.
