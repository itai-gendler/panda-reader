---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose Switch under Display mode
    kind: actor
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts:
          - Display mode
    contexts:
      desktop:
        place: desktop::translation-settings
  - text: Save the other display mode and show the open article's translation that way
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: changes
        facts:
          - Display mode
      - entity: article
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::translation-settings
---

# Switch the display mode

## Trigger

The Reader chooses Switch under Display mode, or the Bilingual / Translated button on the open article.

## Outcome

Translations show as Immersive (original and translation together) or Translation only, for the open article and later ones.
