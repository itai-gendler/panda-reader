---
kind: edge
routes:
  desktop: Desktop
steps:
  - text: Select a passage and choose Create Bamboo Leaf
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
  - text: Report "Article link is too long for a QR code"
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::article
---

# The link is too long for a QR code

## Trigger

The article's link does not fit in a QR code.

## Outcome

"Article link is too long for a QR code" is shown and no image is made.
