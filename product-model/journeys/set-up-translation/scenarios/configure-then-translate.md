---
kind: primary
result: achieved
routes:
  desktop: Desktop
steps:
  - text: Choose Translate on the open article
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::article
    capability: translate-article
  - text: Ask the Reader to configure a translation provider in Settings and open Settings
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts: []
      - entity: translation-provider
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop
    capability: translate-article
  - text: >-
      Choose Anthropic under Provider on the Translation page, enter its API key and choose Save translation
      settings
    kind: actor
    actor: reader
    entities:
      - entity: translation-provider
        effect: reads
        facts:
          - Provider
      - entity: settings
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::translation-settings
    capability: change-translation-settings
  - text: Save the provider and its key
    kind: product
    actor: reader
    entities:
      - entity: translation-provider
        effect: changes
        facts:
          - Provider
          - API key
    contexts:
      desktop:
        place: desktop::translation-settings
    capability: change-translation-settings
  - text: Return to the article and choose Translate
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::article
    capability: translate-article
  - text: Translate the article and show the translation
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: changes
        facts:
          - Translation
          - Translated title
    contexts:
      desktop:
        place: desktop::article
    capability: translate-article
---

# Configure a provider, then translate

## Trigger

The Reader chooses Translate on an article before any translation provider has its keys.

## Outcome

The provider is saved and the article is shown translated.
