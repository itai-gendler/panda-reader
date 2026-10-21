---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose an article without full text in the list
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
  - text: Auto extract full text is on and the article has a link but no full text
    kind: condition
    entities:
      - entity: settings
        effect: reads
        facts:
          - Auto extract full text
      - entity: article
        effect: reads
        facts:
          - Link
          - Full text
    contexts:
      desktop:
        place: desktop::article
  - text: Extract the article from its original page, reusing a saved page when there is one
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Original page
      - entity: settings
        effect: reads
        facts:
          - Content extractor
    contexts:
      desktop:
        place: desktop::article
  - text: Save and show the full text
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: changes
        facts:
          - Full text
          - Original page
          - Link
          - Translation
          - Translated title
    contexts:
      desktop:
        place: desktop::article
---

# Extract automatically when an article opens

## Trigger

The Reader opens an article with a link and no full text while Auto extract full text is on.

## Outcome

The full text replaces the feed content shortly after the article opens. A page downloaded earlier is reused instead of being downloaded again.
