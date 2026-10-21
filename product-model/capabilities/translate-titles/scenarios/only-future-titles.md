---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose a feed marked Translate titles
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::feeds
  - text: Only translate future titles is on
    kind: condition
    entities:
      - entity: settings
        effect: reads
        facts:
          - Only translate future titles
      - entity: feed
        effect: reads
        facts:
          - Titles translated from
    contexts:
      desktop:
        place: desktop::article-list
  - text: Translate the titles of articles newer than the feed's starting point
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::article-list
  - text: Save and show each title translation
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

# Translate only titles that arrive later

## Trigger

Only translate future titles is on (the default).

## Outcome

Only articles newer than the point where the feed was marked are translated automatically; older titles remain available through Translate title.
