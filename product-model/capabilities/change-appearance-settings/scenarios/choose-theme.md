---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Change theme and pick a dark theme
    kind: actor
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts:
          - Theme
    contexts:
      desktop:
        place: desktop::appearance-settings
  - text: Save and apply the theme
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: changes
        facts:
          - Theme
    contexts:
      desktop:
        place: desktop::appearance-settings
---

# Choose a theme

## Trigger

The Reader wants a darker reading palette.

## Outcome

The whole window switches to the theme at once.
