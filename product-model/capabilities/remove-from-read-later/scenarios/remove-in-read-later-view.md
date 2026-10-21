---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Remove from Read Later on the open article
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Read later
    contexts:
      desktop:
        place: desktop::article
  - text: Take the article off the Read Later list
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: changes
        facts:
          - Read later
    contexts:
      desktop:
        place: desktop::article
---

# Remove an article from Read Later

## Trigger

The Reader has read an article they saved for later.

## Outcome

The article is off the Read Later list and leaves that view.
