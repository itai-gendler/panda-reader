---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose an article read before
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::article-list
  - text: Remember reading position is on and a position is saved
    kind: condition
    entities:
      - entity: settings
        effect: reads
        facts:
          - Remember reading position
      - entity: article
        effect: reads
        facts:
          - Reading position
    contexts:
      desktop:
        place: desktop::article
  - text: Show the article scrolled to the saved position
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
          - Feed content
          - Full text
          - Reading position
    contexts:
      desktop:
        place: desktop::article
---

# Resume where the Reader left off

## Trigger

The Reader reopens an article they had scrolled through while Remember reading position is on.

## Outcome

The article opens scrolled to the saved position.
