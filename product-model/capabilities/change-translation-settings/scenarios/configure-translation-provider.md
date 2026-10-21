---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose OpenAI-compatible under Provider
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
  - text: Enter the API URL, API key and Model and choose Save translation settings
    kind: actor
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::translation-settings
  - text: Save the provider and its keys
    kind: product
    actor: reader
    entities:
      - entity: translation-provider
        effect: changes
        facts:
          - Provider
          - API URL
          - API key
          - Model
    contexts:
      desktop:
        place: desktop::translation-settings
---

# Choose a provider and save its keys

## Trigger

The Reader has an API key for one of the translation services.

## Outcome

The provider and keys are saved and "Translation settings saved" is shown; Translate works from now on.
