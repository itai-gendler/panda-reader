---
kind: alternate
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
          - Title
    contexts:
      desktop:
        place: desktop::article
  - text: Draw the quote image
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
  - text: Choose Save PNG and pick a location
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::bamboo-leaf
---

# Save a Bamboo Leaf as PNG

## Trigger

The Reader wants the quote image as a file.

## Outcome

The image is saved where the Reader chose and "Poster saved" is shown.
