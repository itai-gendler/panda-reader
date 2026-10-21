---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Turn on Only translate future titles
    kind: actor
    actor: reader
    entities:
      - entity: settings
        effect: reads
        facts:
          - Only translate future titles
    contexts:
      desktop:
        place: desktop::translation-settings
  - text: Save the choice and restart every marked feed from its newest current article
    kind: product
    actor: reader
    entities:
      - entity: settings
        effect: changes
        facts:
          - Only translate future titles
      - entity: feed
        effect: changes
        facts:
          - Titles translated from
      - entity: article
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::translation-settings
---

# Turn on Only translate future titles

## Trigger

Automatic title translation is on and the Reader no longer wants titles already in the library translated.

## Outcome

Every feed marked Translate titles now starts from its newest current article, so only articles that arrive later are translated automatically.
