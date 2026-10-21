---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose Read Later in the article's menu
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
  - text: Put the article on the Read Later list without queuing anything for the provider
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: changes
        facts:
          - Read later
      - entity: library
        effect: reads
        facts:
          - Source
    contexts:
      desktop:
        place: desktop::article-list
---

# Save for later in a provider library

## Trigger

The article belongs to a Miniflux or FreshRSS library.

## Outcome

The article is on Read Later on this computer only; nothing is sent to the provider, and syncs keep the mark.
