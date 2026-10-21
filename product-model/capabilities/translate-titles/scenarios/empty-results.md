---
kind: edge
routes:
  desktop: Desktop
steps:
  - text: Choose a feed marked Translate titles
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::feeds
  - text: Save the non-empty translations and report the empty ones
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: changes
        facts:
          - Title translation
    contexts:
      desktop:
        place: desktop::article-list
---

# The provider returns empty titles

## Trigger

Some translations come back empty.

## Outcome

Those titles keep their original text and "Translation API returned {} empty result(s)" is shown.
