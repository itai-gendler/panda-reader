---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose Japanese as the language to translate into
    kind: actor
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts:
          - Article language
    contexts:
      desktop:
        place: desktop::translation-settings
  - text: Save the article language, hide a translation on screen and queue listed titles in the new language
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: changes
        facts:
          - Article language
      - entity: article
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::translation-settings
---

# Change the article language

## Trigger

The Reader wants translations into another language.

## Outcome

A translation on screen is replaced by the original text, and listed titles are queued for translation into the new language.
