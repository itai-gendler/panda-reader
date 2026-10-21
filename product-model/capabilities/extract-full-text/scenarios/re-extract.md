---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose Re-extract full text
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Full text
    contexts:
      desktop:
        place: desktop::article
  - text: Download the original page again and extract it with the chosen content extractor
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts:
          - Content extractor
      - entity: article
        effect: reads
        facts:
          - Link
    contexts:
      desktop:
        place: desktop::article
  - text: Replace the full text and drop the translation made from the old body
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

# Re-extract after changing the extractor

## Trigger

The Reader picked another content extractor in Reading settings and wants to compare.

## Outcome

The original page is downloaded again and the new extraction replaces the old full text.
