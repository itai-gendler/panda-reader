---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Select a passage and choose Create Bamboo Leaf in the menu
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
          - Link
    contexts:
      desktop:
        place: desktop::article
  - text: Draw the quote with the article title, feed, feed icon and a QR code of the link
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
          - Link
      - entity: feed
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::bamboo-leaf
  - text: Choose Copy image
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::bamboo-leaf
---

# Copy a Bamboo Leaf

## Trigger

The Reader selects a passage they want to share as an image.

## Outcome

The quote image is on the clipboard and "Poster copied" is shown.
