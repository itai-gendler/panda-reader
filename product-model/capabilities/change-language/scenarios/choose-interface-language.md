---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose 简体中文 under Language
    kind: actor
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts:
          - Language
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Save the language and redraw the interface in it
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: changes
        facts:
          - Language
    contexts:
      desktop:
        place: desktop::general-settings
---

# Choose the interface language

## Trigger

The Reader prefers another interface language.

## Outcome

Menus, settings and messages switch to the chosen language at once; the article language for translation is unchanged.
