---
kind: edge
routes:
  desktop: Desktop
steps:
  - text: Choose Extract full text
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Link
    contexts:
      desktop:
        place: desktop::article
  - text: The article's content changed after the extraction started
    kind: condition
    entities:
      - entity: article
        effect: reads
        facts:
          - Feed content
    contexts:
      desktop:
        place: desktop::article
  - text: Download and extract the page once more against the new content
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::article
  - text: Save the full text when the article has not changed again
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

# The article changes during extraction

## Trigger

A refresh changes the article while its page is being extracted.

## Outcome

The extraction is redone once from a fresh download; if the article changed again, "Article changed while extracting full text. Please try again." is shown and nothing is saved.
