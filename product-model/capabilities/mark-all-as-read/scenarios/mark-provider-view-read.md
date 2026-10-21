---
kind: alternate
routes:
  desktop: Desktop
steps:
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
  - text: Mark the view's unread articles read and queue the changes for the provider
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
      - entity: library
        effect: reads
        facts:
          - Source
    contexts:
      desktop:
        place: desktop::article-list
  - text: Send the queued read changes to the provider in the background
    kind: product
    actor: reader
    entities: []
    contexts:
      desktop:
        place: desktop::article-list
---

# Mark a provider view as read

## Trigger

The Reader chooses Read all in a connected Miniflux or FreshRSS library.

## Outcome

The articles are read here at once and the changes are sent to the provider in the background.
