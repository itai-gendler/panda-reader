---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Hide images on the open article
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::article
  - text: Save the choice and show the article without images
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: changes
        facts:
          - Hide images
      - entity: article
        effect: reads
        facts:
          - Feed content
          - Full text
    contexts:
      desktop:
        place: desktop::article
---

# Hide the images of articles

## Trigger

The Reader wants text only.

## Outcome

The open article is shown again without images, and so are articles opened later.
