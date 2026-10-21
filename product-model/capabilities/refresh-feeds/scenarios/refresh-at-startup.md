---
kind: alternate
routes:
  desktop: Desktop
steps:
  - text: The first article list after start-up has loaded
    kind: condition
    unattended: true
    entities:
      - entity: article
        effect: reads
        facts:
          - Title
    contexts:
      desktop:
        place: desktop::article-list
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
  - text: Save new entries as unread articles
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
    contexts:
      desktop:
        place: desktop::feeds
---

# Refresh after start-up

## Trigger

The app has started and loaded its first article list from the saved library.

## Outcome

The saved articles are shown at once and the library refreshes behind them.
