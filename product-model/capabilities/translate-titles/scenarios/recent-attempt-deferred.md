---
kind: edge
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
  - text: These titles were sent for translation less than a minute ago
    kind: condition
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::article-list
  - text: Defer the titles and report the deferred count
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::article-list
---

# Wait before retrying a title

## Trigger

The list reloads within a minute of a translation attempt for the same titles.

## Outcome

Those titles are not sent again yet; "Title translation retry deferred for {} title(s) after a recent attempt" is shown.
