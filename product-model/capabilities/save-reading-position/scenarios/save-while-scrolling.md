---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Scroll through the open article
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
  - text: Remember reading position is on and scrolling has paused for a moment
    kind: condition
    entities:
      - entity: settings
        effect: reads
        facts:
          - Remember reading position
    contexts:
      desktop:
        place: desktop::article
  - text: Save the article's reading position
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: changes
        facts:
          - Reading position
    contexts:
      desktop:
        place: desktop::article
---

# Save the position while scrolling

## Trigger

The Reader scrolls through an article and pauses.

## Outcome

The position is saved for this article in this library; it is also saved when another article opens.
