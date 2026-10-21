---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose an unread article in the list
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
  - text: >-
      Prepare the body from the full text, or else the feed content, through the enabled cleanup plugins that
      match its link
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: reads
        facts:
          - Feed content
          - Full text
          - Link
      - entity: article-plugin
        effect: reads
        facts:
          - Domains
          - Stage
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::article
  - text: Show the article and mark it read
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

# Open an unread article

## Trigger

The Reader chooses an unread article in the list.

## Outcome

The article is shown with its feed, author, date and body (its full text when extracted, otherwise the feed content, cleaned by enabled cleanup plugins). It is now read and its feed's unread count is one lower; in a provider library the change is queued for the provider.
