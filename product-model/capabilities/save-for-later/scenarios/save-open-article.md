---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Save for later on the open article
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
  - text: Put the article on the Read Later list
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

# Save the open article for later

## Trigger

The Reader finds an article they want to read properly later.

## Outcome

The article is listed under Read Later.
