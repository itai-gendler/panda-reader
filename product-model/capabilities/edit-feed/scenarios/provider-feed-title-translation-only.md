---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose Edit on a provider feed in Added feeds
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::general-settings
  - text: A provider library is selected, so the title, folder and feed URL are not offered
    kind: condition
    entities:
      - entity: library
        effect: reads
        facts:
          - Source
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::edit-feed
  - text: Turn off Translate titles and choose Save
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts:
          - Translate titles
    contexts:
      desktop:
        place: desktop::edit-feed
  - text: Save only the title translation choice
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: changes
        facts:
          - Translate titles
          - Titles translated from
    contexts:
      desktop:
        place: desktop::edit-feed
---

# Edit a provider feed

## Trigger

The Reader opens Edit on a feed in a Miniflux or FreshRSS library.

## Outcome

Only Translate titles can be changed; the dialog says "Feed details are managed by the selected subscription provider."
