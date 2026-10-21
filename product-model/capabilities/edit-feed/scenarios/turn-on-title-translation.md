---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose Edit on the feed in Added feeds
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
  - text: Turn on Translate titles and choose Save
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
  - text: Mark the feed for title translation from its newest current article
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: changes
        facts:
          - Translate titles
          - Titles translated from
      - entity: article
        effect: reads
        facts:
          - Published
    contexts:
      desktop:
        place: desktop::edit-feed
---

# Turn on title translation for a feed

## Trigger

The Reader wants the titles of a foreign-language feed translated in the article list.

## Outcome

The feed is marked Translate titles and its newest current article is recorded, so Only translate future titles covers only articles that arrive later.
