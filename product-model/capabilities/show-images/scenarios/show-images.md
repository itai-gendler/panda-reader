---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Show images on the open article
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::article
  - text: Save the choice and show the article with its images
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

# Show the images of articles again

## Trigger

Images are hidden and the Reader wants them back.

## Outcome

The open article and articles opened later show their images.
