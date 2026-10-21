---
kind: primary
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
  - text: Clear the error of feeds that answered and record the time of this refresh
    kind: product
    actor: reader
    entities:
      - entity: feed
        effect: changes
        from: Refresh failed
        to: Active
        facts:
          - Refresh error
      - entity: settings
        effect: changes
        facts:
          - Last refresh
    contexts:
      desktop:
        place: desktop::feeds
---

# Refresh the Local library

## Trigger

The Reader chooses Refresh feeds in the feed column.

## Outcome

New and changed entries are in the library, a "Synced" message is shown, and the status bar shows the time of the last refresh.
