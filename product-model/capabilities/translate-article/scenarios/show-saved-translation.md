---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose Show translation on the open article
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Translation
    contexts:
      desktop:
        place: desktop::article
  - text: The saved translation matches the body, language and provider
    kind: condition
    entities:
      - entity: article
        effect: reads
        facts:
          - Translation
      - entity: translation-provider
        effect: reads
        facts:
          - Provider
      - entity: settings
        effect: reads
        facts:
          - Article language
    contexts:
      desktop:
        place: desktop::article
  - text: Show the saved translation
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Translation
          - Translated title
    contexts:
      desktop:
        place: desktop::article
---

# Show a saved translation

## Trigger

The article was translated before into the same language, by the same provider, from the same body.

## Outcome

The saved translation is shown at once; nothing is sent to the provider.
