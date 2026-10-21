---
kind: alternate
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
  - text: Ask Miniflux to refresh that feed on the server
    kind: product
    actor: reader
    entities:
      - entity: library
        effect: reads
        facts:
          - Source
      - entity: feed
        effect: reads
        facts: []
    contexts:
      desktop:
        place: desktop::feeds
  - text: Synchronize the library
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
          - Starred
      - entity: library
        effect: changes
        facts:
          - Last synchronized
    contexts:
      desktop:
        place: desktop::feeds
---

# Refresh one provider feed

## Trigger

The Reader wants one feed of their connected provider account checked now.

## Outcome

Miniflux has refreshed the feed on the server and the library has synchronized; FreshRSS offers no single-feed refresh, so the library synchronizes with what the server has.
