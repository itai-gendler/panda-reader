---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Show original on the open article
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Translation
    contexts:
      desktop:
        place: desktop::article
  - text: Show the article without its translation
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Feed content
          - Full text
    contexts:
      desktop:
        place: desktop::article
---

# Show the original text

## Trigger

The Reader wants to read the original wording.

## Outcome

The article shows its original text; Show translation brings the saved translation back without a new request.
