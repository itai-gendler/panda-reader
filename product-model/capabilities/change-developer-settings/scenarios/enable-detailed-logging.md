---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Turn on Detailed sync logging
    kind: actor
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts:
          - Detailed sync logging
    contexts:
      desktop:
        place: desktop::developer-settings
  - text: Save the choice and add detailed entries to the sync log from now on
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: changes
        facts:
          - Detailed sync logging
    contexts:
      desktop:
        place: desktop::developer-settings
---

# Turn on detailed sync logging

## Trigger

A provider sync misbehaves and the Reader wants to report it.

## Outcome

Provider connection checks and per-page sync timings are added to sync.log, which never holds article content or credentials; Copy path gives the log's location.
