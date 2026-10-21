---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Star on the open article
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Starred
    contexts:
      desktop:
        place: desktop::article
  - text: Star the article
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: changes
        facts:
          - Starred
    contexts:
      desktop:
        place: desktop::article
---

# Star the open article

## Trigger

The Reader wants to keep an article they are reading.

## Outcome

The article is starred and listed under Starred; in a provider library the star is sent to the provider.
