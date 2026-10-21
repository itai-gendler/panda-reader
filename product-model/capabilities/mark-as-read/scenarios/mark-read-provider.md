---
kind: alternate
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
  - text: Mark the article read and queue the change for the provider
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: changes
        from: Unread
        to: Read
        facts: []
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
references:
  - kind: code
    role: implementation
    target: apps/panda-reader/src/services/articles/mod.rs#queue_remote_mark_flush
---

# Send the read state to the provider

## Trigger

The article belongs to a connected provider library.

## Outcome

The article is read at once; the change is queued and sent to the provider in the background.

## Edge cases

- If the provider cannot be reached, the article stays read here and the change is sent with the next sync.
