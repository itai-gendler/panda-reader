---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Choose Starred in the feed column
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::feeds
  - text: Choose Read all
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
  - text: Starred is the current view
    kind: condition
    entities:
      - entity: article
        effect: reads
        facts:
          - Starred
    contexts:
      desktop:
        place: desktop::article-list
  - text: Mark read the unread articles that are starred
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

# Mark the Starred view as read

## Trigger

The Reader chooses Read all while Starred or Read Later is shown.

## Outcome

Only the starred (or read-later) unread articles are read; other unread articles stay unread.
