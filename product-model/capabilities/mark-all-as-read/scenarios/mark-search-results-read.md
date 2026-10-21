---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Type a phrase into Search articles
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::article-list
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
  - text: Mark read the unread articles of the view that match the phrase
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

# Mark only the search results as read

## Trigger

The Reader has searched the current view and chooses Read all.

## Outcome

Only the unread articles of the view that match the search are read; the others stay unread.
