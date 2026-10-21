---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: Fifteen minutes have passed while the window is open
    kind: condition
    unattended: true
    entities: []
    contexts:
      desktop:
        place: desktop::feeds
  - text: Refresh the selected library
    kind: product
    entities:
      - entity: library
        effect: reads
        facts:
          - Source
    contexts:
      desktop:
        place: desktop::feeds
  - text: Save new entries as unread articles and record the time of this refresh
    kind: product
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
      - entity: settings
        effect: changes
        facts:
          - Last refresh
    contexts:
      desktop:
        place: desktop::feeds
---

# Refresh every 15 minutes

## Trigger

Fifteen minutes have passed since the window opened or since the last scheduled refresh.

## Outcome

The selected library is refreshed the same way as with Refresh feeds.
