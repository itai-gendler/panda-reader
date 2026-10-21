---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Mark all as read in the feed's menu
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts:
          - Title
          - Unread count
    contexts:
      desktop:
        place: desktop::feeds
  - text: Show the feed and mark all its unread articles read
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

# Mark a whole feed as read

## Trigger

The Reader wants to catch up on a busy feed without reading it.

## Outcome

The feed is shown and all its unread articles are read; "Marked {} articles as read" gives the count.
