---
kind: validation
routes:
  desktop: Desktop
steps:
  - text: Choose DeepL under Provider
    kind: actor
    actor: reader
    entities:
      - entity: translation-provider
        effect: reads
        facts:
          - Provider
    contexts:
      desktop:
        place: desktop::translation-settings
  - text: Mark the provider as coming soon and treat translation as not configured
    kind: product
    actor: reader
    entities:
      - entity: translation-provider
        effect: changes
        facts:
          - Provider
    contexts:
      desktop:
        place: desktop::translation-settings
---

# Choose a provider that is coming soon

## Trigger

The Reader chooses DeepL or LibreTranslate.

## Outcome

They are marked "Coming soon": no keys are asked for and translation stays unavailable until another provider is chosen.
