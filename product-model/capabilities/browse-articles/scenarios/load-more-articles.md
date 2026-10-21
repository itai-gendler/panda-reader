---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose Load more articles
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
          - Title translation
          - Published
    contexts:
      desktop:
        place: desktop::article-list
  - text: Append the next 20 articles of the view
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
          - Title translation
          - Published
    contexts:
      desktop:
        place: desktop::article-list
---

# Load more articles

## Trigger

The Reader reaches the end of the loaded list, or runs Load more articles.

## Outcome

The next 20 articles of the view are appended after the last one shown.
