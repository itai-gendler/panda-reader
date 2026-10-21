---
kind: edge
routes:
  desktop: Desktop
steps:
  - text: Choose Refresh feeds
    kind: actor
    actor: reader
    entities:
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::feeds
  - text: >-
      Download every Local feed, eight at a time, sending each feed's cache validators so unchanged feeds are
      skipped
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
  - text: Save new entries as unread articles
    kind: product
    actor: reader
    entities:
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
  - text: Update the saved entries whose content changed
    kind: product
    actor: reader
    entities:
      - entity: article
        effect: changes
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
  - text: Record the error on each feed that failed
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: changes
        from: Active
        to: Refresh failed
        facts:
          - Refresh error
    contexts:
      desktop:
        place: desktop::feeds
---

# Some feeds fail

## Trigger

One feed cannot be downloaded or read while the others answer.

## Outcome

The refresh still succeeds; the failing feed shows its error in the feed column until a later refresh succeeds.
