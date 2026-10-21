---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Mark as unread in the article's menu
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
  - text: Mark the article unread and raise its feed's unread count
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: changes
        from: Read
        to: Unread
        facts: []
      - entity: feed
        effect: changes
        facts:
          - Unread count
    contexts:
      desktop:
        place: desktop::article-list
---

# Mark an article as unread

## Trigger

The Reader wants to come back to an article later as new.

## Outcome

The article is unread again and its feed's unread count rises; in a provider library the change is sent to the provider.
