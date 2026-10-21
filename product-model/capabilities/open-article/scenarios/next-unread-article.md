---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Run Next unread
    kind: actor
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::article
  - text: Find the next unread article in the list, loading more when needed
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
          - Published
    contexts:
      desktop:
        place: desktop::article-list
  - text: Show it and mark it read
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
        place: desktop::article
---

# Move to the next unread article

## Trigger

The Reader runs Next unread (J), or Next article (j or Mod+]) for the next one in any state.

## Outcome

The next unread article below the current one opens and is marked read; when the loaded list runs out, more articles are loaded and the move continues.
