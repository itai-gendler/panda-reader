---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Sans-serif and raise the Text size
    kind: actor
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts:
          - Article font
          - Text size
          - Line height
          - Text width
          - Paragraph spacing
    contexts:
      desktop:
        place: desktop::reading-settings
  - text: Save the typography within its limits
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: changes
        facts:
          - Article font
          - Text size
    contexts:
      desktop:
        place: desktop::reading-settings
---

# Adjust the typography

## Trigger

The Reader finds the article text too small.

## Outcome

Articles use the new font and spacing; sizes stay within their limits (text size 14 to 30, line height 1.2 to 2.2, text width 520 to 1000, paragraph spacing 0.5 to 2.5).
