---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Turn off Auto extract full text and choose Trafilatura as Content extractor
    kind: actor
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts:
          - Auto extract full text
          - Content extractor
    contexts:
      desktop:
        place: desktop::reading-settings
  - text: Save both choices
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: changes
        facts:
          - Auto extract full text
          - Content extractor
    contexts:
      desktop:
        place: desktop::reading-settings
---

# Choose how full text is extracted

## Trigger

The Reader wants to try another content extractor and stop automatic extraction.

## Outcome

Articles no longer fetch their original page on opening, and Extract full text uses the chosen extractor.
