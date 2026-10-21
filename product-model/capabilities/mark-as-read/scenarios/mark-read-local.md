---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Mark as read in the article's menu
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::article-list
  - text: Mark the article read and lower its feed's unread count
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: changes
        from: Unread
        to: Read
        facts: []
      - entity: feed
        effect: changes
        facts:
          - Unread count
    contexts:
      desktop:
        place: desktop::article-list
---

# Mark an article as read

## Trigger

The Reader has seen enough of an article in the list.

## Outcome

The article is read and its feed's unread count drops; in the Unread view it leaves the list.
