---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: An update was postponed
    kind: condition
    entities:
      - entity: app-update
        effect: reads
        facts:
          - Version
    contexts:
      desktop:
        place: desktop::about
  - text: Choose Check now
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::about
  - text: Find the downloaded package still matching its checksum and offer it again
    kind: product
    actor: reader
    entities:
      - entity: app-update
        effect: changes
        from: Postponed
        to: Ready
        facts: []
    contexts:
      desktop:
        place: desktop::about
---

# Offer a postponed update again

## Trigger

The Reader chose Later earlier and the next check finds the same release.

## Outcome

The package already downloaded is checked against its checksum and offered again without a new download.
