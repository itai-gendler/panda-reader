---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Translate title in the article's menu
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
  - text: Translate the title into the article language, even when a recent attempt failed
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts:
          - Article language
      - entity: translation-provider
        effect: reads
        facts:
          - Provider
      - entity: article
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::article-list
  - text: Save and show the title translation
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

# Translate one title

## Trigger

The Reader cannot tell from its title whether an article is worth opening.

## Outcome

The list shows the translated title in place of the original; the request is counted in daily usage.
