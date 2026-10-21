---
kind: validation
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
  - text: The selected translation provider is missing its keys
    kind: condition
    entities:
      - entity: translation-provider
        effect: reads
        facts:
          - Provider
    contexts:
      desktop:
        place: desktop::article-list
  - text: Report "Title translation failed:" with the missing-key message
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::article-list
---

# Translate a title before a provider is configured

## Trigger

The selected translation provider has no keys.

## Outcome

"Title translation failed:" is shown with the provider's missing-key message.
