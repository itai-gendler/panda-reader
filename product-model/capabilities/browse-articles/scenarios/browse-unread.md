---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Unread in the feed column
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::feeds
  - text: List the first 20 unread articles of the selected library, newest first
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
          - Title translation
          - Published
      - entity: feed
        effect: reads
        facts:
          - Title
      - entity: library
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::article-list
---

# Browse unread articles

## Trigger

The Reader wants to see what they have not read yet.

## Outcome

The article list shows the library's unread articles, newest first, each with its feed and date and, where one exists, its title translation in place of the title.
