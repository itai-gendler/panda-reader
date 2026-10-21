---
kind: validation
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
  - text: The selected translation provider is missing its keys
    kind: condition
    entities:
      - entity: translation-provider
        effect: reads
        facts:
          - Provider
          - Azure API key
    contexts:
      desktop:
        place: desktop::article
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
---

# Translate before a provider is configured

## Trigger

The Reader chooses Translate before entering the keys of the selected translation provider.

## Outcome

"Configure a translation provider in Settings before translating." is shown and Settings opens.
