---
kind: primary
routes:
  desktop: Desktop
steps:
  - text: Choose Sync now in the feed's menu
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::feeds
  - text: Download the feed, skipping it when the server reports it unchanged
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts:
          - Feed URL
    contexts:
      desktop:
        place: desktop::feeds
  - text: Save new and changed entries and clear the feed's error
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: changes
        from: Refresh failed
        to: Active
        facts:
          - Refresh error
      - entity: article
        effect: creates
        to: Unread
        facts:
          - Title
          - Link
          - Author
          - Published
          - Summary
          - Feed content
    contexts:
      desktop:
        place: desktop::feeds
---

# Refresh one Local feed

## Trigger

The Reader wants the latest entries of one Local feed now.

## Outcome

The feed's new entries are in the library, its error is cleared and "Feed refreshed" is shown.
