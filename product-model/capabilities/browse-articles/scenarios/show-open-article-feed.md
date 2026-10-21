---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose the feed name above the article
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts:
          - Title
      - entity: article
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::article
  - text: List that feed's articles and keep the article open
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
          - Title translation
          - Published
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::article-list
---

# Show the open article's feed

## Trigger

The Reader chooses the feed name above the open article.

## Outcome

The list switches to that feed while the article stays open.
