---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose Export OPML
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Ask the provider for its subscriptions as OPML
    kind: product
    actor: reader
    entities:
      - entity: library
        effect: reads
        facts:
          - Source
    contexts:
      desktop:
        place: desktop::general-settings
  - text: Choose where to save the file
    kind: actor
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::general-settings
---

# Export a provider library

## Trigger

The Reader exports the subscriptions of their connected provider account.

## Outcome

The file holds the provider's own export: Miniflux's OPML, or for FreshRSS the subscription list without folders.
