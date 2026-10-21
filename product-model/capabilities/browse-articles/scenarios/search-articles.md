---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Type a phrase into Search articles
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::article-list
  - text: List the articles of the current view whose title, author, body or translation contains the phrase
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
          - Author
          - Feed content
          - Full text
          - Translation
    contexts:
      desktop:
        place: desktop::article-list
---

# Search the articles in view

## Trigger

The Reader types into Search articles.

## Outcome

Shortly after typing stops, the list shows only articles in the current view whose title, author, body or translation contains the typed words as a phrase; Chinese, Japanese and Korean text matches short phrases too.
