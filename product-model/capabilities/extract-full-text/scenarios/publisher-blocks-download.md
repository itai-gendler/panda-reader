---
kind: edge
routes:
  desktop: Desktop
steps:
  - text: Choose Extract full text
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Link
    contexts:
      desktop:
        place: desktop::article
  - text: Report that the publisher blocked the download and suggest opening the original page
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::article
---

# The publisher refuses the download

## Trigger

The publisher answers with 401 or 403.

## Outcome

"Publisher blocked automated article downloads … Open the original page instead." is shown.
